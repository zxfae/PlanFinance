use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use serde::{Serialize, Deserialize};
use reqwasm::http::Request;
use crate::{AppRoute, header, footer};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StepTwoo {
    id: i32,
    user_id: i32,
    production: i32,
    entretien: i32,
    clientele: i32,
    interprofesion: i32,
    formation: i32,
}

pub struct StepTwo {
    user_id: Option<i32>,
    production: i32,
    entretien: i32,
    clientele: i32,
    interprofesion: i32,
    formation: i32,
    entreprise: Option<Entreprise>,
    clone_jrsttx: Option<i32>,
    submitted: bool,
}

pub enum Msg {
    UpdateProduction(i32),
    UpdateEntretien(i32),
    UpdateClientele(i32),
    UpdateInterprofession(i32),
    UpdateFormation(i32),
    Submit,
    SubmissionComplete(StepTwoo),
    LoadEntreprise(Entreprise),
    LoadEntrepriseError,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entreprise {
    id: i32,
    user_id: i32,
    name: String,
    date: String,
    codeape: String,
    status: String,
    jrsttx: i32,
}

impl Component for StepTwo {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Récupérer l'ID de l'utilisateur depuis le local storage
        let user_id = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("user_id")
            .ok()
            .flatten()
            .and_then(|id| id.parse::<i32>().ok());

        if let Some(user_id) = user_id {
            ctx.link().send_future(async move {
                let url = format!("http://localhost:8080/get_ent?user_id={}", user_id);
                let response = Request::get(&url).send().await.unwrap();
                if response.ok() {
                    let entreprise: Entreprise = response.json().await.unwrap();
                    Msg::LoadEntreprise(entreprise)
                } else {
                    log::error!("Failed to fetch entreprise");
                    Msg::LoadEntrepriseError
                }
            });
        }

        Self {
            user_id,
            production: 0,
            entretien: 0,
            clientele: 0,
            interprofesion: 0,
            formation: 0,
            entreprise: None,
            clone_jrsttx: None,
            submitted: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateProduction(value) => {
                self.production = value;
                true
            }
            Msg::UpdateEntretien(value) => {
                self.entretien = value;
                true
            }
            Msg::UpdateClientele(value) => {
                self.clientele = value;
                true
            }
            Msg::UpdateInterprofession(value) => {
                self.interprofesion = value;
                true
            }
            Msg::UpdateFormation(value) => {
                self.formation = value;
                true
            }
            Msg::Submit => {
                if !self.submitted {
                    let activities = StepTwoo {
                        id: 0,
                        user_id: self.user_id.unwrap_or_default(),
                        production: self.production,
                        entretien: self.entretien,
                        clientele: self.clientele,
                        interprofesion: self.interprofesion,
                        formation: self.formation,
                    };
                    let activities_json = serde_json::to_string(&activities).unwrap();
                    log::info!("Submitting activities: {}", activities_json);
                    ctx.link().send_future(async {
                        let response = Request::post("http://localhost:8080/add_act")
                            .header("Content-Type", "application/json")
                            .body(activities_json)
                            .send()
                            .await;

                        match response {
                            Ok(resp) => {
                                if resp.ok() {
                                    let new_activities: Result<StepTwoo, _> = resp.json().await;
                                    match new_activities {
                                        Ok(new_activities) => Msg::SubmissionComplete(new_activities),
                                        Err(e) => {
                                            log::error!("Failed to parse response: {:?}", e);
                                            Msg::Submit
                                        }
                                    }
                                } else {
                                    log::error!("Failed to submit activities: {}", resp.status());
                                    Msg::Submit
                                }
                            }
                            Err(e) => {
                                log::error!("Request failed: {:?}", e);
                                Msg::Submit
                            }
                        }
                    });
                    self.submitted = true;
                    true
                } else {
                    false
                }
            }
            Msg::SubmissionComplete(new_activities) => {
                log::info!("Submission completed.");
                web_sys::window()
                    .unwrap()
                    .local_storage()
                    .unwrap()
                    .unwrap()
                    .set_item("user_id", &new_activities.id.to_string())
                    .unwrap();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::FormEntreprise);
                true
            }
            Msg::LoadEntreprise(entreprise) => {
                self.clone_jrsttx = Some(entreprise.jrsttx);
                self.entreprise = Some(entreprise);
                true
            }
            Msg::LoadEntrepriseError => {
                log::error!("Failed to load entreprise");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col min-h-screen">
                { header() }
                <div class="bg-orange-50 flex flex-col flex-grow justify-center items-center">
                    <div class="drop-shadow-md text-center text-gray-600 text-4xl font-semibold mb-20">
                        <h1>{ "Répartition Temps de Travail / d'activité" }</h1>
                        <div class="text-center text-gray-600 text-2xl font-semibold m-2">
                            <h1>{ "Nous défendons l'idée que chacun peut créer son business plan facilement et gratuitement" }</h1>
                        </div>
                    </div>
                    <table class="table-auto mb-4 border-collapse border-separate border border-gray-900">
                        <thead>
                            <tr class="bg-orange-100">
                                <th class="px-4 py-2">{ "Répartition temps d'activité" }</th>
                                <th class="px-4 py-2">{ "Nombre de jours" }</th>
                                <th class="px-4 py-2">{ "Jours travaillés" }</th>
                                <th class="px-4 py-2">{ "Pourcentage" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="border px-4 py-2">{ "Production - vente = CA" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="number"
                                        value={self.production.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateProduction(input.value().parse().unwrap_or(0))
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "..." }</td>
                                <td class="border px-4 py-2">{ "..." }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Entretien / Maintenance ..." }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="number"
                                        value={self.entretien.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateEntretien(input.value().parse().unwrap_or(0))
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "..." }</td>
                                <td class="border px-4 py-2">{ "..." }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Gestion clients, Devis, Facture..." }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="number"
                                        value={self.clientele.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateClientele(input.value().parse().unwrap_or(0))
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "..." }</td>
                                <td class="border px-4 py-2">{ "..." }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Interprofession" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="number"
                                        value={self.interprofesion.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateInterprofession(input.value().parse().unwrap_or(0))
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "..." }</td>
                                <td class="border px-4 py-2">{ "..." }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Formation" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="number"
                                        value={self.formation.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateFormation(input.value().parse().unwrap_or(0))
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "..." }</td>
                                <td class="border px-4 py-2">{ "..." }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "" }</td>
                                <td class="border px-4 py-2">
                                    { self.view_cloned_jrsttx() }
                                </td>
                                <td class="border px-4 py-2">{ "..." }</td>
                                <td class="border px-4 py-2">{ "..." }</td>
                            </tr>
                        </tbody>
                    </table>
                    <div class="w-full max-w-md">
                        <form class="border-solid border-2 border-orange-400 bg-white shadow-[0_35px_60px_-15px_rgba(0,0,0,0.5)] rounded-lg px-8 pt-6 pb-8 mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                            e.prevent_default();
                            Msg::Submit
                        })}>
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

impl StepTwo {
    fn view_cloned_jrsttx(&self) -> Html {
        if let Some(cloned_jrsttx) = self.clone_jrsttx {
            html! { <p class="text-gray-700">{ format!("Jours travaillés clonés: {}", cloned_jrsttx) }</p> }
        } else {
            html! { <></> }
        }
    }
}
