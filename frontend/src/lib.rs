use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

struct FormModel {
    first_name: String,
    last_name: String,
    submitted: bool,
}

enum Msg {
    UpdateFirstName(String),
    UpdateLastName(String),
    Submit,
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                self.submitted = true;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let title = "Formulaire avec Tailwind CSS".to_string();
        html! {
            <div class="container mx-auto p-4">
                <h1 class="text-3xl font-bold mb-4">{ title }</h1>
                <form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4" onsubmit={_ctx.link().callback(|e: SubmitEvent| {
                    e.prevent_default();
                    Msg::Submit
                })}>
                    <div class="mb-4">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="first_name">{ "First Name:" }</label>
                        <input
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="first_name"
                            type="text"
                            value={self.first_name.clone()}
                            oninput={_ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateFirstName(input.value())
                            })}
                        />
                    </div>
                    <div class="mb-6">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="last_name">{ "Last Name:" }</label>
                        <input
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="last_name"
                            type="text"
                            value={self.last_name.clone()}
                            oninput={_ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateLastName(input.value())
                            })}
                        />
                    </div>
                    <div class="flex items-center justify-between">
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            type="submit"
                        >
                            { "Submit" }
                        </button>
                    </div>
                </form>
                { self.view_result() }
            </div>
        }
    }
}

impl FormModel {
    fn view_result(&self) -> Html {
        if self.submitted {
            html! {
                <div class="mt-4 p-4 bg-green-100 border border-green-400 text-green-700 rounded">
                    <p>{ format!("Submitted First Name: {}", self.first_name) }</p>
                    <p>{ format!("Submitted Last Name: {}", self.last_name) }</p>
                </div>
            }
        } else {
            html! { <div></div> }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<FormModel>::new().render();
}
