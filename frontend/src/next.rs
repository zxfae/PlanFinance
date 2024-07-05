use yew::prelude::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

use crate::header;
use crate::footer;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct User {
    id: i32,
    lastname: String,
    firstname: String,
}

#[function_component(Success)]
pub fn success() -> Html {
    let user = use_state(|| None);
    let user_clone = user.clone();

    use_effect(move || {
        let user_clone = user_clone.clone();
        spawn_local(async move {
            // Lire l'ID utilisateur depuis le stockage local
            if let Some(user_id) = web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap()
                .unwrap()
                .get_item("user_id")
                .unwrap()
            {
                let url = format!("http://localhost:8080/get_user?id={}", user_id);
                let fetched_user: Result<User, _> = Request::get(&url).send().await.unwrap().json().await;

                if let Ok(fetched_user) = fetched_user {
                    user_clone.set(Some(fetched_user));
                }
            }
        });

        || ()
    });

    html! {
        <div class="flex flex-col min-h-screen justify-center items-center">
            { header() }
            <div class="flex flex-col flex-grow justify-center items-center">
                <h1 class="text-3xl font-serif text-gray-900 mb-4">{ "Submission Successful!" }</h1>
                <p>{ "Your data has been successfully submitted." }</p>
                {
                    if let Some(user) = &*user {
                        html! {
                            <p>{ format!("Bienvenue {} {}", user.lastname, user.firstname) }</p>
                        }
                    } else {
                        html! {
                            <p>{ "Chargement des données utilisateur..." }</p>
                        }
                    }
                }
                <a href="/" class="mt-4 bg-emerald-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                    { "Go Back" }
                </a>
            </div>
            { footer() }
        </div>
    }
}
