use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use serde::{Serialize, Deserialize};
use reqwasm::http::Request;
use crate::{AppRoute, header, footer};

pub struct StepTwo {
    last_name: String,
    first_name: String,
    entreprise: Option<Entreprise>,
    submitted: bool,
}

pub enum Msg {
    UpdateLastName(String),
    UpdateFirstName(String),
    Submit,
    SubmissionComplete(User),
    LoadEntreprise(Entreprise),
    LoadEntrepriseError,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    id: i32,
    lastname: String,
    firstname: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entreprise {
    id: i32,
    user_id: i32,
    name: String,
    date: String,
    codeape: String,
    status: String,
    jrsttx: i32,
    jrsweek: i16,
    jrsferies: i8,
    jrscp: i8,
    jan: i8,
    fev: i8,
    mar: i8,
    avr: i8,
    mai: i8,
    juin: i8,
    jui: i8,
    aout: i8,
    sept: i8,
    oct: i8,
    nov: i8,
    dec: i8,
}

impl Component for StepTwo {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Récupérer l'ID de l'utilisateur depuis le local storage
        let user_id = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("user_id")
            .ok()
            .flatten()
            .and_then(|id| id.parse::<i32>().ok());

        if let Some(user_id) = user_id {
            // Récupérer les données de l'entreprise depuis l'API
            ctx.link().send_future(async move {
                let url = format!("http://localhost:8080/get_ent?user_id={}", user_id);
                let response = Request::get(&url).send().await.unwrap();
                if response.ok() {
                    let entreprise: Entreprise = response.json().await.unwrap();
                    Msg::LoadEntreprise(entreprise)
                } else {
                    log::error!("Failed to fetch entreprise");
                    Msg::LoadEntrepriseError
                }
            });
        }

        Self {
            last_name: String::new(),
            first_name: String::new(),
            entreprise: None,
            submitted: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateLastName(value) => {
                self.last_name = value;
                true
            }
            Msg::UpdateFirstName(value) => {
                self.first_name = value;
                true
            }
            Msg::Submit => {
                if !self.submitted {
                    let user = User {
                        id: 0,
                        lastname: self.last_name.clone(),
                        firstname: self.first_name.clone(),
                    };
                    let user_json = serde_json::to_string(&user).unwrap();
                    log::info!("Submitting user: {}", user_json);
                    ctx.link().send_future(async {
                        let response = Request::post("http://localhost:8080/add_user")
                            .header("Content-Type", "application/json")
                            .body(user_json)
                            .send()
                            .await
                            .unwrap();

                        if response.ok() {
                            let new_user: User = response.json().await.unwrap();
                            Msg::SubmissionComplete(new_user)
                        } else {
                            Msg::Submit
                        }
                    });
                    self.submitted = true;
                    true
                } else {
                    false
                }
            }
            Msg::SubmissionComplete(new_user) => {
                log::info!("Submission completed.");
                web_sys::window()
                    .unwrap()
                    .local_storage()
                    .unwrap()
                    .unwrap()
                    .set_item("user_id", &new_user.id.to_string())
                    .unwrap();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::FormEntreprise);
                true
            }
            Msg::LoadEntreprise(entreprise) => {
                self.entreprise = Some(entreprise);
                true
            }
            Msg::LoadEntrepriseError => {
                log::error!("Failed to load entreprise");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col min-h-screen">
                { header() }
                <div class="bg-orange-50 flex flex-col flex-grow justify-center items-center">
                    <div class="drop-shadow-md text-center text-gray-600 text-4xl font-semibold mb-20">
                        <h1>{ "Proposer, c'est possible !" }</h1>
                        <div class="text-center text-gray-600 text-2xl font-semibold m-2">
                            <h1>{ "Nous défendons l'idée que chacun peut créer son business plan facilement et gratuitement" }</h1>
                        </div>
                    </div>
                    <div class="w-full max-w-md">
                        <form class="border-solid border-2 border-orange-400 bg-white shadow-[0_35px_60px_-15px_rgba(0,0,0,0.5)] rounded-lg px-8 pt-6 pb-8 mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                            e.prevent_default();
                            Msg::Submit
                        })}>
                            <div class="mb-4">
                                { self.view_box_title() }
                                <label class="block text-orange-500 text-sm font-semibold mb-2" for="last_name">{ "Nom" }</label>
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                    id="last_name"
                                    type="text"
                                    placeholder="Entrez votre nom"
                                    value={self.last_name.clone()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateLastName(input.value())
                                    })}
                                    required=true
                                />
                            </div>
                            <div class="mb-6">
                                <label class="block text-orange-500 text-sm font-semibold mb-2" for="first_name">{ "Prénom" }</label>
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                    id="first_name"
                                    type="text"
                                    placeholder="Entrez votre prénom"
                                    value={self.first_name.clone()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateFirstName(input.value())
                                    })}
                                    required=true
                                />
                            </div>
                            { self.view_entreprise_info() }
                            <div class="flex items-center justify-center">
                                <button
                                    class="bg-emerald-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                                    type="submit"
                                    disabled={self.submitted}
                                >
                                    { "SIMULER MON PROJET" }
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
                { footer() }
            </div>
        }
    }
}

impl StepTwo {
    fn view_box_title(&self) -> Html {
        html! {
            <div class="text-center text-xl font-semibold mb-4">
                <h1 class="text-gray-700">{ "Je simule mon business plan" }</h1>
            </div>
        }
    }

    fn view_entreprise_info(&self) -> Html {
        if let Some(ref entreprise) = self.entreprise {
            html! {
                <div class="mb-4">
                    <h2 class="text-xl font-semibold text-gray-700">{ "Détails de l'entreprise" }</h2>
                    <p class="text-gray-700">{ format!("Nom: {}", entreprise.name) }</p>
                    <p class="text-gray-700">{ format!("Date: {}", entreprise.date) }</p>
                    <p class="text-gray-700">{ format!("Code APE: {}", entreprise.codeape) }</p>
                    <p class="text-gray-700">{ format!("Statut: {}", entreprise.status) }</p>
                    <p class="text-gray-700">{ format!("Jours travaillees: {}", entreprise.jrsttx) }</p>

                </div>
            }
        } else {
            html! { <></> }
        }
    }
}
