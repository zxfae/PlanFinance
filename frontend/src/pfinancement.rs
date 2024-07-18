use yew::prelude::*;

pub struct PlanFinancement;

impl Component for PlanFinancement {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col min-h-screen">
                <div class="bg-orange-50 flex flex-col flex-grow justify-center items-center">
                    <h1 class="text-gray-600 text-4xl font-semibold">{ "SALUT" }</h1>
                </div>
            </div>
        }
    }
}