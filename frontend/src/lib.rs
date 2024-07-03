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
        html! {
            <div class="container">
                <form class="form" onsubmit={_ctx.link().callback(|e: SubmitEvent| {
                    e.prevent_default();
                    Msg::Submit
                })}>
                    <div class="form-group">
                        <label for="first_name">{ "First Name: " }</label>
                        <input
                            id="first_name"
                            type="text"
                            value={self.first_name.clone()}
                            oninput={_ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateFirstName(input.value())
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label for="last_name">{ "Last Name: " }</label>
                        <input
                            id="last_name"
                            type="text"
                            value={self.last_name.clone()}
                            oninput={_ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateLastName(input.value())
                            })}
                        />
                    </div>
                    <button type="submit" class="submit-button">{ "Submit" }</button>
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
                <div class="result">
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
