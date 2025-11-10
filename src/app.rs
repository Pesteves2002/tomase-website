use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use std::marker::PhantomData;

use gloo_timers::future::TimeoutFuture;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tomase-website.css" />

        // sets the document title
        <Title text="Home | Tomás Esteves" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found :(".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn SizeOf<T: Sized>(#[prop(optional)] _ty: PhantomData<T>) -> impl IntoView {
    std::mem::size_of::<T>()
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar<F>(
    /// How much progress should be displayed.
    progress: F,
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
) -> impl IntoView
where
    F: Fn() -> i32 + Send + Sync + 'static,
{
    view! {
        <progress max=max value=progress />
        <br />
    }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input
                type="number"
                on:input:target=move |ev| { set_value.set(ev.target().value().parse::<i32>()) }
            /> // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            // the fallback receives a signal containing current errors
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect::<Vec<_>>()
                            }}
                        </ul>
                    </div>
                }
            }>
                <p>
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    "You entered " // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `value` changes
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}

async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1000).await;
    value * 10
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(10);
    let on_click = move |_| *count.write() += 1;
    let single_count = move || count.get();
    let double_count = move || count.get() * 2;
    let is_odd = move || count.get() % 2 == 1;
    let async_data = LocalResource::new(move || load_data(count.get()));
    let stable = LocalResource::new(|| load_data(1));

    let spread_onto_component =
        view! { <{..} aria-label="a component with attribute spreading" /> };

    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    view! {
        <h1 {..spread_onto_component}>"Tomás Esteves Website"</h1>
        <button
            on:click=on_click
            // class:red=move || count.get() % 2 == 1
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", move || count.get().to_string())
        >
            "Click Me: "
            {count}
        </button>
        <p>"Double count: " {double_count}</p>
        <p>"Non reactive: " {count.get_untracked() * 2}</p>

        <ProgressBar progress=single_count />
        <ProgressBar max=200 progress=double_count />

        <SizeOf<usize> />
        <SizeOf<String> />

        <Show when=move || { is_odd() }>
            <p>Odd</p>
        </Show>

        <NumericInput />

        <p>
            <code>"stable"</code>
            ": "
            {move || stable.get()}
        </p>
        <p>
            <code>"async_value"</code>
            ": "
            {async_result}
            <br />
        </p>
    }
}
