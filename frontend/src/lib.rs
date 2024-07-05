// main.rs or lib.rs
mod home;
mod next;

use yew::prelude::*;
use yew_router::prelude::*;
use home::FormModel;
use next::Success;
use wasm_bindgen::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
enum AppRoute {
    #[at("/")]
    Home,
    #[at("/success")]
    Success,
}

// Function to render the header
fn header() -> Html {
    let title = "PlanFinance : Créez votre business plan facilement et gratuitement".to_string();
    html! {
    <header class="border-solid border-b-2 border-orange-200 w-full text-center py-6 bg-zinc-50">
        <h1 class="text-3xl font-serif text-gray-900 mb-2 font-semibold">{ title }</h1>
        <a href="https://github.com/zxfae" class="text-gray-500 hover:text-gray-700" aria-label="My Github">
            <svg width="90" height="90" viewBox="0 0 250 250" class="border-solid border-b-2 border-orange-200" style="fill:#fff; color:rgb(251 146 60); position: absolute; top: 0; border: 0; right: 0;" aria-hidden="true">
                <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
                <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="octo-arm"></path>
                <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,1377 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="octo-body"></path>
            </svg>
        </a>
    </header>
    }
}

// Function to render the footer
fn footer() -> Html {
    let title = "Copyright © 2024 || All Right Reserved || Developed by ZxFae".to_string();
    html! {
        <footer class="border-solid border-t-2 border-orange-200 w-full text-center py-6 bg-zinc-50">
            <h1 class="text-3xl font-serif text-gray-900 mb-2">{ title }</h1>
        </footer>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <FormModel /> },
        AppRoute::Success => html! { <Success /> },
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
