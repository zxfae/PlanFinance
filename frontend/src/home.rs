use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use serde::{Serialize, Deserialize};
use reqwasm::http::Request;
use crate::{AppRoute, header, footer};
use crate::utils::{HomeMsg, User, FormHome};


impl Component for FormHome {
    type Message = HomeMsg;
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
            HomeMsg::UpdateLastName(value) => {
                self.last_name = value;
                true
            }
            HomeMsg::UpdateFirstName(value) => {
                self.first_name = value;
                true
            }
            HomeMsg::Submit => {
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
                            HomeMsg::SubmissionComplete(new_user)
                        } else {
                            HomeMsg::Submit
                        }
                    });
                    self.submitted = true;
                    true
                } else {
                    false
                }
            }
            HomeMsg::SubmissionComplete(new_user) => {
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col min-h-screen">
                { header() }
                <div class="bg-orange-50 flex flex-col flex-grow pt-10">
                    <div class="drop-shadow-md text-center text-gray-600 text-4xl font-semibold">
                        <h1>{ "Proposer, c'est possible !" }</h1>
                        <div class="text-center text-gray-600 text-4xl font-medium mb-6">
                            <h1>
                                { "Nous défendons l'idée que chacun peut " }
                                <span class="underline decoration-2">{ "créer son business plan facilement" }</span>
                                { " et gratuitement" }
                            </h1>
                        </div>
                        <div class="text-black text-center text-2xl font-medium mb-10">
                            <p>{"Plan Finance vous accompagne dans la génération de votre plan de financement, bilan prévisionnel et bilan de trésorerie. Présentez votre dossier de financement aux banques en toute confiance !"}</p>
                        </div>
                    </div>
                    <div class="w-full max-w-xl mx-auto">
                        <form class="border-solid border-2 border-orange-400 bg-white rounded-lg px-8 pt-6 pb-8 mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                            e.prevent_default();
                            HomeMsg::Submit
                        })}>
                            <div class="mb-2">
                                {self.view_box_title()}
                                <label class="block text-orange-500 text-center text-medium font-semibold mb-2" for="last_name">{ "Nom" }</label>
                                <input
                                    class="mb-6 shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                    id="last_name"
                                    type="text"
                                    placeholder="Entrez votre nom"
                                    value={self.last_name.clone()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        HomeMsg::UpdateLastName(input.value())
                                    })}
                                required=true
                                />
                            </div>
                            <div class="mb-6">
                                <label class="block text-orange-500 text-center text-medium font-semibold mb-2" for="first_name">{ "Prénom" }</label>
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                    id="first_name"
                                    type="text"
                                    placeholder="Entrez votre prénom"
                                    value={self.first_name.clone()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        HomeMsg::UpdateFirstName(input.value())
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
                    <div class="w-full max-w-xl mx-auto pt-9 flex">
                        <div class="flex space-x-30 flex-col items-center justify-center">
                            <img src="https://www.svgrepo.com/show/483373/free-whiteboard-7.svg" alt="whiteboard" width="70" height="70"/>
                            <p class="text-center pl-4 text-2xl font-serif font-medium text-gray-700">{"Générez votre business plan"}</p>
                        </div>
                        <div class="flex flex-col items-center justify-center pl-4">
                            <img src="https://www.svgrepo.com/show/507651/document-download.svg" alt="document download" width="70" height="70"/>
                            <p class="text-center text-2xl font-serif font-medium text-gray-700">{"Téléchargez votre dossier en PDF"}</p>
                        </div>
                        <div class="flex flex-col items-center justify-center pl-4">
                            <img src="https://www.svgrepo.com/show/477567/free-5.svg" alt="free" width="70" height="70"/>
                            <p class="text-center pl-4 text-2xl font-serif font-medium text-gray-700">{"Entièrement gratuit"}</p>
                        </div>
                    </div>
                </div>
                { footer() }
            </div>
        }
    }
}

impl FormHome {
    fn view_box_title(&self) -> Html {
        html! {
            <div class="text-center text-4xl font-medium mb-4">
                <h1 class="text-gray-700">{"Je crée mon business plan"}</h1>
            </div>
        }
    }
}
