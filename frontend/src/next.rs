use yew::prelude::*;
use crate::header;
use crate::footer;

#[function_component(Success)]
pub fn success() -> Html {
    html! {
        <div class="flex flex-col min-h-screen justify-center items-center">
            { header() }
            <div class="flex flex-col flex-grow justify-center items-center">
                <h1 class="text-3xl font-serif text-gray-900 mb-4">{ "Submission Successful!" }</h1>
                <p>{ "Your data has been successfully submitted." }</p>
                <a href="/" class="mt-4 bg-emerald-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                    { "Go Back" }
                </a>
            </div>
            { footer() }
        </div>
    }
}
