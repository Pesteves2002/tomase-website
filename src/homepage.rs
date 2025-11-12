use leptos::prelude::*;

struct Link {
    // Get icons from https://www.svgrepo.com/vectors/strava/
    icon_path: String,
    url: String,
    name: String,
}

impl Link {
    fn new(name: &str, url: &str) -> Link {
        let icons_path = "icons/";
        Link {
            icon_path: format!("{}{}.svg", icons_path, name),
            url: url.into(),
            name: name.into(),
        }
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let links = vec![
        Link::new("strava", "https://www.strava.com/athletes/26750651"),
        Link::new("github", "https://github.com/Pesteves2002/"),
        Link::new("linkedin", "https://www.linkedin.com/in/tomase-pt/"),
        Link::new("mail", "mailto:me@tomase.pt"),
    ];

    view! {
        <img src="me.jpg" alt="Me" width="600" />
        <div>Hi!</div>
        {links
            .into_iter()
            .map(|l| {
                view! {
                    <a href=l.url>
                        <img src=l.icon_path alt=l.name />
                    </a>
                }
            })
            .collect_view()}
    }
}
