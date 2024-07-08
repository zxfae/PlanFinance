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

//New form
pub struct FormEntreprise{
    name: String,
    date: String,
    codeape: String,
    status: String,
    submitted: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entreprise{
    id: i32,
    name: String,
    date: String,
    codeape: String,
    status: String,
}

//Same logical home.rs
pub enum Msg{
    UpdateName(String),
    UpdateDate(String),
    UpdateCodeApe(String),
    UpdateStatus(String),
    Submit,
    SubmissionComplete(Entreprise),
}
#[function_component(StepOne)]
pub fn stepOne() -> Html {
    let user = use_state(|| None);
    let error = use_state(|| None);
    let has_fetched = use_state(|| false);

    {
        let user = user.clone();
        let error = error.clone();
        let has_fetched = has_fetched.clone();

        use_effect(move || {
            if !*has_fetched {
                let user = user.clone();
                let error = error.clone();
                spawn_local(async move {
                    let user_id_opt = web_sys::window()
                        .unwrap()
                        .local_storage()
                        .unwrap()
                        .unwrap()
                        .get_item("user_id")
                        .unwrap();

                    if let Some(user_id) = user_id_opt {
                        console::log_1(&format!("User ID found: {}", user_id).into());

                        let cached_user_opt = web_sys::window()
                            .unwrap()
                            .local_storage()
                            .unwrap()
                            .unwrap()
                            .get_item(&format!("user_{}", user_id))
                            .unwrap();

                        if let Some(cached_user) = cached_user_opt {
                            let fetched_user: User = serde_json::from_str(&cached_user).unwrap();
                            user.set(Some(fetched_user));
                        } else {
                            let url = format!("http://localhost:8080/get_ent?id={}", user_id);
                            match Request::get(&url).send().await {
                                Ok(response) => {
                                    match response.json::<User>().await {
                                        Ok(fetched_user) => {
                                            web_sys::window()
                                                .unwrap()
                                                .local_storage()
                                                .unwrap()
                                                .unwrap()
                                                .set_item(&format!("user_{}", user_id), &serde_json::to_string(&fetched_user).unwrap())
                                                .unwrap();
                                            user.set(Some(fetched_user));
                                        },
                                        Err(err) => {
                                            console::error_1(&format!("Failed to parse JSON: {:?}", err).into());
                                            error.set(Some(format!("Failed to parse JSON: {:?}", err)));
                                        }
                                    }
                                },
                                Err(err) => {
                                    console::error_1(&format!("Failed to fetch: {:?}", err).into());
                                    error.set(Some(format!("Failed to fetch: {:?}", err)));
                                }
                            }
                        }
                    } else {
                        error.set(Some("User ID not found in local storage".into()));
                    }
                });
                has_fetched.set(true);
            }
            || ()
        });
    }

    html! {
        <div class="flex flex-col min-h-screen justify-center items-center">
            { header() }
            <div class="flex flex-col flex-grow justify-center items-center">
                {
                    if let Some(error) = &*error {
                        html! { <p class="text-red-500">{ format!("Error: {}", error) }</p> }
                    } else if let Some(user) = &*user {
                        html! {
                            <>
                                { formEnt(user) }
                                <p class="text-3xl font-serif text-gray-900 mb-4">{ format!("Bienvenue {} {}", user.lastname, user.firstname) }</p>
                            </>
                        }
                    } else {
                        html! { <p>{ "Chargement des données utilisateur..." }</p> }
                    }
                }
                <p>{ "Your data has been successfully submitted." }</p>
                <a href="/" class="mt-4 bg-emerald-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                    { "Go Back" }
                </a>
            </div>
            { footer() }
        </div>
    }
}

fn formEnt(user: &User) -> Html {
    html! {
        <div class="bg-white shadow-md rounded-lg p-4 m-4">
            <p class="text-2xl font-bold text-center">{ format!("Bienvenue, {} {}!", user.firstname, user.lastname) }</p>
        </div>
    }
}