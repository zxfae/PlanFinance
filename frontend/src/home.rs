use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use serde::{Serialize, Deserialize};
use reqwasm::http::Request;
use crate::{AppRoute, header, footer};

pub struct FormModel {
    last_name: String,
    first_name: String,
    submitted: bool,
}

pub enum Msg {
    UpdateLastName(String),
    UpdateFirstName(String),
    Submit,
    SubmissionComplete(User),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    id: i32,
    lastname: String,
    firstname: String,
}
impl Component for FormModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            last_name: String::new(),
            first_name: String::new(),
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
                // Navigate to FormEntreprise route
                navigator.push(&AppRoute::FormEntreprise);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="flex flex-col min-h-screen">
            { header() }
            <div class="bg-orange-50 flex flex-col flex-grow justify-center items-center">
                <div class="text-center text-gray-600 text-4xl font-semibold mb-2">
                    <h1>{ "Proposer, c'est possible !" }</h1>
                <div class="text-center text-gray-600 text-2xl font-semibold m-2">
                    <h1>{"Nous défendons l'idée que chacun peut créer son business plan facilement et gratuitement"}</h1>
                </div>
                </div>
                <div class="w-full max-w-md">
                    <form class="border-solid border-2 border-orange-400 bg-white shadow-[0_35px_60px_-15px_rgba(0,0,0,0.5)] rounded-lg px-8 pt-6 pb-8 mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                        e.prevent_default();
                        Msg::Submit
                    })}>
                        <div class="mb-4">
                            {self.view_box_title()}
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

impl FormModel {
    fn view_box_title(&self) -> Html{
        let title = "Je simule mon business plan".to_string();
        html!{
            <div class="text-center text-grey-600 text-xl font-semibold mb-4">
                <h1>{title}</h1>
            </div>
        }
    }
}