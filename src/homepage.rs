use leptos::{
    html::{div, h1, h2},
    prelude::*,
};

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
        Link::new("github", "https://github.com/Pesteves2002/"),
        Link::new("linkedin", "https://www.linkedin.com/in/tomase-pt/"),
        Link::new("mail", "mailto:me@tomase.pt"),
        Link::new("strava", "https://www.strava.com/athletes/26750651"),
    ];
    let title = h1().inner_html("Tomás Esteves");

    let subtitle = h2().inner_html("Compute Science & Engineering Graduate at <a href=\"https://tecnico.ulisboa.pt/en/\">Instituto Superior Técnico</a>");

    let presentation = div().class("presentation").inner_html("Recently gratuated from my Master's with a thesis in distributing the computation of matrix functions.");

    // Create nested childs
    // let content_container = div()
    //     .class("content-container")
    //     .child(title)
    //     .child(subtitle)
    //     .child(presentation);

    view! {
        <div class="container">
            <div class="left">
                <img src="me.jpg" alt="Me" />
            </div>
            <div class="right">
                <div class="content-container">{title} {subtitle} {presentation}</div>
                <div class="links-container">
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
                </div>
            </div>
        </div>
    }
}
