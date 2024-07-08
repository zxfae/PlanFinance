use yew::prelude::*;
use web_sys::HtmlInputElement;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use crate::header;
use crate::footer;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entreprise {
    id: i32,
    user_id: i32,
    name: String,
    date: String,
    codeape: String,
    status: String,
    jrsttx: String,
    jrsweek: String,
    jrsferies: String,
    jrscp: String,
}

pub struct FormEntreprise {
    user_id: i32,
    name: String,
    date: String,
    codeape: String,
    status: String,
    jrsttx: String,
    jrsweek: String,
    jrsferies: String,
    jrscp: String,
    submitted: bool,
    current_step: usize,
    decompte: u32,
    total: u32,
}

pub enum Msg {
    UpdateName(String),
    UpdateDate(String),
    UpdateCodeApe(String),
    UpdateStatus(String),
    UpdateJrsTTX(String),
    UpdateJrsWeek(String),
    UpdateJrsFeries(String),
    UpdateJrsCp(String),
    CalculateDecompte,
    CalculateTotal,
    Submit,
    SubmissionComplete(Entreprise),
}

impl Component for FormEntreprise {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let user_id = match web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("user_id") {
            Ok(Some(id)) => id.parse::<i32>().unwrap_or_else(|_| {
                console::error_1(&"Failed to parse user_id".into());
                0
            }),
            Ok(None) | Err(_) => {
                console::error_1(&"Failed to get user_id from local storage".into());
                0
            }
        };

        Self {
            user_id,
            name: String::new(),
            date: String::new(),
            codeape: String::new(),
            status: String::new(),
            jrsttx: String::new(),
            jrsweek: String::new(),
            jrsferies: String::new(),
            jrscp: String::new(),
            submitted: false,
            current_step: 1,
            decompte: 0,
            total: 0,
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
            Msg::UpdateJrsTTX(value) => {
                self.jrsttx = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateJrsWeek(value) => {
                self.jrsweek = value;
                ctx.link().send_message(Msg::CalculateDecompte);
                true
            }
            Msg::UpdateJrsFeries(value) => {
                self.jrsferies = value;
                ctx.link().send_message(Msg::CalculateDecompte);
                true
            }
            Msg::UpdateJrsCp(value) => {
                self.jrscp = value;
                ctx.link().send_message(Msg::CalculateDecompte);
                true
            }
            //qubit project inspiration
            Msg::CalculateDecompte => {
                self.decompte = self.jrsweek.parse::<u32>().unwrap_or(0) +
                    self.jrsferies.parse::<u32>().unwrap_or(0) +
                    self.jrscp.parse::<u32>().unwrap_or(0);
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::CalculateTotal => {
                self.total = self.jrsttx.parse::<u32>().unwrap_or(0) -
                    self.decompte;
                true
            }

            Msg::Submit => {
                if !self.submitted {
                    if self.current_step == 1 {
                        self.current_step += 1;
                        true
                    } else {
                        let entreprise = Entreprise {
                            id: 0,
                            user_id: self.user_id,
                            name: self.name.clone(),
                            date: self.date.clone(),
                            codeape: self.codeape.clone(),
                            status: self.status.clone(),
                            jrsttx: self.jrsttx.clone(),
                            jrsweek: self.jrsweek.clone(),
                            jrsferies: self.jrsferies.clone(),
                            jrscp: self.jrscp.clone(),
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
                    }
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
                    <div class="flex flex-row w-full justify-center">
                        {
                            if self.current_step == 1 {
                                self.render_step1(ctx)
                            } else {
                                html! {
                                    <>
                                        <div class="mr-8">
                                            { self.render_step1(ctx) }
                                        </div>
                                        <div>
                                            { self.render_step2(ctx) }
                                        </div>
                                    </>
                                }
                            }
                        }
                    </div>
                </div>
                { footer() }
            </div>
            }
        }
    }
}

impl FormEntreprise {
    fn render_step1(&self, ctx: &Context<Self>) -> Html {
        let class = if self.current_step == 1 {
            "w-full max-w-md"
        } else {
            "w-full max-w-md opacity-50"
        };

        html! {
            <>
                <div class={class}>
                    <div class="mb-10 text-center text-gray-600 text-4xl font-semibold">
                        <h1>{ "Étape 1" }</h1>
                    <div class="mb-3 text-center text-gray-600 text-2xl font-semibold m-2">
                        <h1>{"Afin de commencer votre simulation, veuillez renseigner votre future situation :"}</h1>
                    </div>
                    </div>
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
                                placeholder="Date potentielle ouverture (JJ:MM:AAAA)"
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
            </>
        }
    }

    fn render_step2(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="w-full max-w-md">
                    <div class="mb-10 text-center text-gray-600 text-4xl font-semibold">
                        <h1>{ "Étape 2" }</h1>
                    <div class="mb-3 text-center text-gray-600 text-2xl font-semibold m-2">
                        <h1>{"Cette section calcule les jours travaillés et non travaillés :"}</h1>
                    </div>
                    </div>
                    <form class="border-solid border-2 border-orange-400 bg-white shadow-[0_35px_60px_-15px_rgba(0,0,0,0.5)] rounded-lg px-8 pt-6 pb-8 mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                        e.prevent_default();
                        Msg::Submit
                    })}>
                        <div class="mb-6">
                            {self.view_form_deux()}
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="jrsttx">{ "Jours travaillés dans l'année" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="jrsttx"
                                type="text"
                                placeholder="Année pleine : 365 jours"
                                value={self.jrsttx.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateJrsTTX(input.value())
                                })}
                            required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="jrsweek">{ "Jours week-end" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="jrsweek"
                                type="text"
                                placeholder="En moyenne 104 jours"
                                value={self.jrsweek.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateJrsWeek(input.value())
                                })}
                            required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="jrsferies">{ "Jours fériés" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="jrsferies"
                                type="text"
                                placeholder="En moyenne 11 jours"
                                value={self.jrsferies.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateJrsFeries(input.value())
                                })}
                            required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="jrscp">{ "Jours congés payés" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="jrscp"
                                type="text"
                                placeholder="En moyenne 25 jours"
                                value={self.jrscp.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateJrsCp(input.value())
                                })}
                            required=true
                            />
                        </div>
                        <div class="mb-2 text-center text-sm font-semibold text-gray-700">{ "Décompte jours non travaillés: " }<div class="mb-2 text-center text-sm font-semibold text-red-500">{ self.decompte }</div></div>
                        <div class="mb-2 text-center text-sm font-semibold text-gray-700">{ "Total jours travaillés: " }<div class="text-red-500">{ self.total }</div></div>
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
            </>
        }
    }

    fn view_box_title(&self) -> Html {
        html! {
            <div class="mb-4 text-xl font-bold text-center text-gray-700">
                { "Formulaire d'entreprise" }
            </div>
        }
    }

    fn view_form_deux(&self) -> Html{
        html!{
            <div class="mb-4 text-xl font-bold text-center text-gray-700">
                {"Décompte des Jours Travaillés et Non Travaillés"}
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
