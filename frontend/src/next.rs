use yew::prelude::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

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
    let error = use_state(|| None);
    let user_clone = user.clone();
    let error_clone = error.clone();

    use_effect(move || {
        let user_clone = user_clone.clone();
        let error_clone = error_clone.clone();
        spawn_local(async move {
            if let Some(user_id) = web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap()
                .unwrap()
                .get_item("user_id")
                .unwrap()
            {
                console::log_1(&format!("User ID found: {}", user_id).into());
                let url = format!("http://localhost:8080/get_user?id={}", user_id);
                match Request::get(&url).send().await {
                    Ok(response) => {
                        match response.json::<User>().await {
                            Ok(fetched_user) => {
                                user_clone.set(Some(fetched_user));
                            },
                            Err(err) => {
                                console::error_1(&format!("Failed to parse JSON: {:?}", err).into());
                                error_clone.set(Some(format!("Failed to parse JSON: {:?}", err)));
                            }
                        }
                    },
                    Err(err) => {
                        console::error_1(&format!("Failed to fetch: {:?}", err).into());
                        error_clone.set(Some(format!("Failed to fetch: {:?}", err)));
                    }
                }
            } else {
                error_clone.set(Some("User ID not found in local storage".into()));
            }
        });
        || ()
    });

    html! {
        <div class="flex flex-col min-h-screen justify-center items-center">
            { header() }
            <div class="flex flex-col flex-grow justify-center items-center">
                <h1 class="text-3xl font-serif text-gray-900 mb-4">{ "Je simule mon business plan" }</h1>
                <p>{ "Your data has been successfully submitted." }</p>
                {
                    if let Some(error) = &*error {
                        html! { <p class="text-red-500">{ format!("Error: {}", error) }</p> }
                    } else if let Some(user) = &*user {
                        html! {
                            <p>{ format!("Bienvenue {} {}", user.lastname, user.firstname) }</p>
                        }
                    } else {
                        html! { <p>{ "Chargements des donnses utilisateur..." }</p> }
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
