// todo.rs

use leptos::{prelude::*, reactive::spawn_local};

#[server]
pub async fn add_todo(title: String) -> Result<String, ServerFnError> {
    println!("Add_todo");
    Ok("ok".to_string())
}

#[component]
pub fn BusyButton() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async {
                let a = add_todo("So much to do!".to_string()).await;
                if let Ok(s) = a {
                    println!("{s}");
                }
            });
        }>"Add Todo"</button>
    }
}
