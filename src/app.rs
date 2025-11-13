use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::adhd::*;
use crate::homepage::*;

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
                <nav class="navbar">
                    <a href="/">Home</a>
                    <div class="nav-right">
                        <a href="/ADHD">ADHD Corner</a>
                    </div>
                </nav>
                <Routes fallback=|| "Page not found :(".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("/ADHD") view=ADHDPage />
                </Routes>
            </main>
        </Router>
    }
}
