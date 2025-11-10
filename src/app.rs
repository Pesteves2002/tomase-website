use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use std::marker::PhantomData;

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

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(10);
    let on_click = move |_| *count.write() += 1;
    let single_count = move || count.get();
    let double_count = move || count.get() * 2;

    let spread_onto_component =
        view! { <{..} aria-label="a component with attribute spreading" /> };

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
    }
}
