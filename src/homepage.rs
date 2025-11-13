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
        Link::new("github", "https://github.com/Pesteves2002/"),
        Link::new("linkedin", "https://www.linkedin.com/in/tomase-pt/"),
        Link::new("mail", "mailto:me@tomase.pt"),
        Link::new("strava", "https://www.strava.com/athletes/26750651"),
    ];

    let presentation = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec lacinia dictum justo ut tempor. Etiam mattis suscipit ipsum, ut condimentum urna eleifend non. Nunc sed tincidunt odio, a dictum velit. Suspendisse sagittis elementum erat, vitae ultrices ex dapibus vitae. Pellentesque placerat, lacus ac molestie lobortis, turpis lorem interdum enim, id pharetra lacus sapien in leo. Suspendisse fringilla sodales lorem nec vehicula. Vestibulum non mollis magna, sit amet lacinia leo. Phasellus venenatis sem leo, ac tempor nunc efficitur sed. Mauris quam lorem, scelerisque vitae purus vel, mattis lacinia ex. Donec gravida auctor libero fringilla dictum. In ac felis et enim tempor feugiat. Donec vel ultrices dui. Donec at nisi malesuada, facilisis arcu nec, auctor nisl. Interdum et malesuada fames ac ante ipsum primis in faucibus.
";

    view! {
        <div class="container">
            <div class="left">
                <img src="me.jpg" alt="Me" />
            </div>
            <div class="right">
                <div class="presentation">{presentation}</div>
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
