use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use crate::{AppRoute, header, footer};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entreprise {
    id: i32,
    user_id: i32,
    name: String,
    date: String,
    codeape: String,
    status: String,
}

pub struct FormEntreprise {
    user_id: i32,
    name: String,
    date: String,
    codeape: String,
    status: String,
    submitted: bool,
}

pub enum Msg {
    UpdateName(String),
    UpdateDate(String),
    UpdateCodeApe(String),
    UpdateStatus(String),
    Submit,
    SubmissionComplete(Entreprise),
}

impl Component for FormEntreprise {
    type Message = Msg;
    type Properties = ();

    //Get user_id local_storage
    fn create(_ctx: &Context<Self>) -> Self {
        let user_id = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("user_id")
            .unwrap()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        Self {
            user_id,
            name: String::new(),
            date: String::new(),
            codeape: String::new(),
            status: String::new(),
            submitted: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateName(value) => {
                self.name = value;
                true
            }
            Msg::UpdateDate(value) => {
                self.date = value;
                true
            }
            Msg::UpdateCodeApe(value) => {
                self.codeape = value;
                true
            }
            Msg::UpdateStatus(value) => {
                self.status = value;
                true
            }
            Msg::Submit => {
                if !self.submitted {
                    let entreprise = Entreprise {
                        id: 0,
                        user_id: self.user_id,
                        name: self.name.clone(),
                        date: self.date.clone(),
                        codeape: self.codeape.clone(),
                        status: self.status.clone(),
                    };
                    let entreprise_json = serde_json::to_string(&entreprise).unwrap();
                    log::info!("Submitting entreprise: {}", entreprise_json);
                    ctx.link().send_future(async {
                        let response = Request::post("http://localhost:8080/add_ent")
                            .header("Content-Type", "application/json")
                            .body(entreprise_json)
                            .send()
                            .await
                            .unwrap();

                        if response.ok() {
                            let new_ent: Entreprise = response.json().await.unwrap();
                            log::info!("Entreprise created: {:?}", new_ent);
                            Msg::SubmissionComplete(new_ent)
                        } else {
                            log::error!("Failed to submit entreprise");
                            Msg::Submit
                        }
                    });
                    self.submitted = true;
                    true
                } else {
                    false
                }
            }
            Msg::SubmissionComplete(new_ent) => {
                log::info!("Submission completed. Entreprise ID: {}", new_ent.id);
                web_sys::window()
                    .unwrap()
                    .local_storage()
                    .unwrap()
                    .unwrap()
                    .set_item("ent_id", &new_ent.id.to_string())
                    .unwrap();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::info!("Rendering view. Submitted: {}", self.submitted);
        if self.submitted {
            self.success(ctx)
        } else {
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
                                <label class="block text-orange-500 text-sm font-semibold mb-2" for="name">{ "Nom de votre entreprise" }</label>
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                    id="name"
                                    type="text"
                                    placeholder="Entrez le nom de votre entreprise"
                                    value={self.name.clone()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateName(input.value())
                                    })}
                                required=true
                                />
                            </div>
                            <div class="mb-6">
                                <label class="block text-orange-500 text-sm font-semibold mb-2" for="date">{ "Date" }</label>
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                    id="date"
                                    type="text"
                                    placeholder="Date potentielle ouverture"
                                    value={self.date.clone()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateDate(input.value())
                                    })}
                                required=true
                                />
                            </div>
                            <div class="mb-6">
                                <label class="block text-orange-500 text-sm font-semibold mb-2" for="codeape">{ "Code APE" }</label>
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                    id="codeape"
                                    type="text"
                                    placeholder="Code APE"
                                    value={self.codeape.clone()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateCodeApe(input.value())
                                    })}
                                required=true
                                />
                            </div>
                            <div class="mb-6">
                                <label class="block text-orange-500 text-sm font-semibold mb-2" for="status">{ "Statut" }</label>
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                    id="status"
                                    type="text"
                                    placeholder="Statut de votre entreprise"
                                    value={self.status.clone()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateStatus(input.value())
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
                                    { "SUIVANT" }
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
}

impl FormEntreprise {
    fn view_box_title(&self) -> Html {
        html! {
            <div class="mb-4 text-xl font-bold text-center text-gray-700">
                { "Formulaire d'entreprise" }
            </div>
        }
    }

    fn success(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col min-h-screen justify-center items-center">
                { header() }
                <div class="flex flex-col flex-grow justify-center items-center">
                    <p class="text-3xl font-serif text-gray-900 mb-4">{ "Votre projet a été soumis avec succès." }</p>
                    <a href="/" class="mt-4 bg-emerald-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                        { "Retour" }
                    </a>
                </div>
                { footer() }
            </div>
        }
    }
}