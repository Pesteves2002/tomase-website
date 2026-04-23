use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// ── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
    pub name: String,
    pub artist_name: String,
    pub album_name: String,
    pub artwork_url: String,
}

// ── Server function (keeps tokens out of WASM) ───────────────────────────────

#[server(FetchRecentTrack, "/api")]
pub async fn fetch_recent_track() -> Result<TrackInfo, ServerFnError> {
    dotenv::dotenv().ok();

    let authorization = std::env::var("APPLE_AUTHORIZATION")
        .map_err(|_| ServerFnError::new("Missing APPLE_AUTHORIZATION env var"))?;
    let media_user_token = std::env::var("APPLE_MEDIA_USER_TOKEN")
        .map_err(|_| ServerFnError::new("Missing APPLE_MEDIA_USER_TOKEN env var"))?;

    let client = reqwest::Client::new();
    let res = client
        .get("https://api.music.apple.com/v1/me/recent/played/tracks?limit=1")
        .header("authorization", authorization)
        .header("media-user-token", media_user_token)
        .header("origin", "https://music.apple.com")
        .header("priority", "u=1, i")
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let body: leptos::serde_json::Value = res
        .json()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let attrs = body["data"][0]["attributes"]
        .as_object()
        .ok_or_else(|| ServerFnError::new("No track data in response"))?;

    let artwork_url = attrs["artwork"]["url"]
        .as_str()
        .unwrap_or("")
        .replace("{w}", "600")
        .replace("{h}", "600");

    Ok(TrackInfo {
        name: attrs["name"].as_str().unwrap_or("").to_string(),
        artist_name: attrs["artistName"].as_str().unwrap_or("").to_string(),
        album_name: attrs["albumName"].as_str().unwrap_or("").to_string(),
        artwork_url,
    })
}

// ── Component ────────────────────────────────────────────────────────────────

#[component]
pub fn MusicPage() -> impl IntoView {
    let track = Resource::new(|| (), |_| fetch_recent_track());
    let refetch = move |_| track.refetch();

    view! {
        <div class="music-page">
            <Transition fallback=|| view! { <TrackSkeleton /> }>
                {move || match track.get() {
                    None => view! { <TrackSkeleton /> }.into_any(),
                    Some(Err(e)) => view! { <ErrorCard message=e.to_string() /> }.into_any(),
                    Some(Ok(t)) => view! { <TrackCard track=t /> }.into_any(),
                }}
            </Transition>
            <button class="refresh-btn" on:click=refetch>
                "↺  Refresh"
            </button>
        </div>
    }
}

// ── Sub-components ───────────────────────────────────────────────────────────

#[component]
fn TrackCard(track: TrackInfo) -> impl IntoView {
    view! {
        <div class="card">
            <div
                class="backdrop"
                style=format!("background-image: url({})", track.artwork_url)
            />
            <div class="card-inner">
                <img class="album-art" src=track.artwork_url alt="Album art" />
                <div class="info">
                    <span class="track-info">{track.name} - {track.artist_name} - {track.album_name}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TrackSkeleton() -> impl IntoView {
    view! {
        <div class="card skeleton-card">
            <div class="card-inner">
                <div class="album-art skeleton" />
                <div class="info">
                    <div class="skeleton" style="height:22px; width:75%; border-radius:6px" />
                    <div class="skeleton" style="height:14px; width:50%; border-radius:6px; margin-top:8px" />
                    <hr class="divider" />
                    <div class="skeleton" style="height:12px; width:65%; border-radius:6px" />
                </div>
            </div>
        </div>
    }
}

#[component]
fn ErrorCard(message: String) -> impl IntoView {
    view! {
        <div class="error-card">
            <strong>"Could not load track"</strong>
            <p>{message}</p>
        </div>
    }
}
