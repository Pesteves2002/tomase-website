use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::adhd::*;
use crate::homepage::*;
use crate::music::*;

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
        <Title text="Tomás Esteves Website" />

        // content for this welcome page
        <Router>
            <nav class="navbar">
                <a href="/">Home</a>
                <div class="nav-right">
                    <a href="/music">Music</a>
                </div>
            </nav>
            <main>
                <Routes fallback=|| "Page not found :(".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("/music") view=MusicPage />
                    <Route path=StaticSegment("/ADHD") view=ADHDPage />
                </Routes>
            </main>
        </Router>
    }
}
