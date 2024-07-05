use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use serde::Serialize;
use reqwasm::http::Request;

struct FormModel {
    first_name: String,
    last_name: String,
    submitted: bool,
}

enum Msg {
    UpdateFirstName(String),
    UpdateLastName(String),
    Submit,
    SubmissionComplete,
}

#[derive(Serialize)]
struct NewUser {
    firstname: String,
    lastname: String,
}

impl Component for FormModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            first_name: String::new(),
            last_name: String::new(),
            submitted: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateFirstName(value) => {
                self.first_name = value;
                true
            }
            Msg::UpdateLastName(value) => {
                self.last_name = value;
                true
            }
            Msg::Submit => {
                if !self.submitted {
                    let user = NewUser {
                        firstname: self.first_name.clone(),
                        lastname: self.last_name.clone(),
                    };
                    let user_json = serde_json::to_string(&user).unwrap();
                    log::info!("Submitting user: {}", user_json); // Log for debugging
                    ctx.link().send_future(async {
                        let _ = Request::post("http://localhost:8080/add_user")
                            .header("Content-Type", "application/json")
                            .body(user_json)
                            .send()
                            .await;
                        Msg::SubmissionComplete
                    });
                    self.submitted = true;
                    true
                } else {
                    false
                }
            }
            Msg::SubmissionComplete => {
                log::info!("Submission completed.");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col min-h-screen">
                { self.header() }
                <div class="flex flex-grow justify-center items-center">
                    <div class="w-full max-w-md">
                        <form class="bg-white shadow-[0_35px_60px_-15px_rgba(0,0,0,0.5)] rounded-lg px-8 pt-6 pb-8 mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                            e.prevent_default();
                            Msg::Submit
                        })}>
                            <div class="mb-4">
                                <label class="block text-gray-700 text-sm font-semibold mb-2" for="first_name">{ "Nom" }</label>
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                    id="first_name"
                                    type="text"
                                    placeholder="Entrez votre nom"
                                    value={self.first_name.clone()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateFirstName(input.value())
                                    })}
                                />
                            </div>
                            <div class="mb-6">
                                <label class="block text-gray-700 text-sm font-semibold mb-2" for="last_name">{ "Prénom" }</label>
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                    id="last_name"
                                    type="text"
                                    placeholder="Entrez votre Prénom"
                                    value={self.last_name.clone()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateLastName(input.value())
                                    })}
                                />
                            </div>

                            <div class="flex items-center justify-center">
                                <button
                                    class="bg-emerald-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                                    type="submit"
                                    disabled={self.submitted}
                                >
                                    { "Envoyez" }
                                </button>
                            </div>
                        </form>
                        { self.view_result() }
                    </div>
                </div>
                { self.footer() }
            </div>
        }
    }
}

impl FormModel {
    fn header(&self) -> Html {
        let title = "PlanFinance : Créez votre business plan facilement et gratuitement".to_string();
        html! {
            <header class="border-solid border-2 border-zinc-400 w-full text-center py-6 bg-gray-100">
                <h1 class="text-3xl font-serif text-gray-900 mb-2">{ title }</h1>
                <a href="https://github.com/zxfae" class="text-gray-500 hover:text-gray-700 " aria-label="My Github">
                    <svg width="90" height="90" viewBox="0 0 250 250" class="inline-block bg-gray-300" style="fill:#fff; color:#304050; position: absolute; top: 0; border: 0; right: 0;" aria-hidden="true">
                        <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
                        <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="octo-arm"></path>
                        <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,1377 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="octo-body"></path>
                    </svg>
                </a>
            </header>
        }
    }

    fn view_result(&self) -> Html {
        if self.submitted {
            html! {
                <div class="mt-4 p-4 bg-green-100 border border-green-400 text-green-700 rounded w-full">
                    <p>{ format!("Submitted First Name: {}", self.first_name) }</p>
                    <p>{ format!("Submitted Last Name: {}", self.last_name) }</p>
                </div>
            }
        } else {
            html! { <div></div> }
        }
    }

    fn footer(&self) -> Html {
        let title = "Copyright © 2024 || All Right Reserved || Developed by ZxFae".to_string();
        html! {
            <footer class="border-solid border-2 border-zinc-400 w-full text-center py-6 bg-gray-100">
                <h1 class="text-3xl font-serif text-gray-900 mb-2">{ title }</h1>
            </footer>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<FormModel>::new().render();
}
