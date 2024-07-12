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
    interprofession: i32,
    formation: i32,
    prodjour: i64,
    prodan: i64,
    tva: i8,
    moyprix: i64,
    cajour: i64,
    caann: i64,
}

pub struct StepTwo {
    user_id: Option<i32>,
    production: i32,
    entretien: i32,
    clientele: i32,
    interprofession: i32,
    formation: i32,
    prodjour: i64,
    prodan: i64,
    tva: i8,
    moyprix: i64,
    cajour: i64,
    caann: i64,
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
    UpdateProdjour(i64),
    UpdateProdan(i64),
    UpdateTva(i8),
    UpdateMoyPrix(i64),
    UpdateCaJour(i64),
    UpdateCaAnn(i64),
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
            interprofession: 0,
            formation: 0,
            prodjour:0,
            prodan:0,
            tva:0,
            moyprix:0,
            cajour:0,
            caann:0,
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
                self.interprofession = value;
                true
            }
            Msg::UpdateFormation(value) => {
                self.formation = value;
                true
            }
            Msg::UpdateProdjour(value)=>{
                self.prodjour = value;
                true
            }
            Msg::UpdateProdan(value)=>{
                self.prodan = value;
                true
            }
            Msg::UpdateTva(value)=>{
                self.tva = value;
                true
            }
            Msg::UpdateMoyPrix(value)=>{
                self.moyprix= value;
                true
            }
            Msg::UpdateCaJour(value)=>{
                self.cajour = value;
                true
            }
            Msg::UpdateCaAnn(value)=>{
                self.caann = value;
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
                        interprofession: self.interprofession,
                        formation: self.formation,
                        prodjour:self.prodjour,
                        prodan:self.prodan,
                        tva:self.tva,
                        moyprix:self.moyprix,
                        cajour:self.cajour,
                        caann:self.caann,
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
                    <table class=" mb-4 border-collapse border-separate border border-gray-900 w-2/4">
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
                                        type="text"
                                        value={self.production.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i32>() {
                                                    Ok(value) => Msg::UpdateProduction(value),
                                                    Err(_) => Msg::UpdateProduction(0),
                                            }
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
                                        type="text"
                                        value={self.entretien.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i32>() {
                                                    Ok(value) => Msg::UpdateEntretien(value),
                                                    Err(_) => Msg::UpdateEntretien(0),
                                            }
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
                                        type="text"
                                        value={self.clientele.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i32>() {
                                                    Ok(value) => Msg::UpdateClientele(value),
                                                    Err(_) => Msg::UpdateClientele(0),
                                            }
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "..." }</td>
                                <td class="border px-4 py-2">{ "" }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Interprofession" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.interprofession.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i32>() {
                                                    Ok(value) => Msg::UpdateInterprofession(value),
                                                    Err(_) => Msg::UpdateInterprofession(0),
                                            }
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "..." }</td>
                                <td class="border px-4 py-2">{ "" }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Formation" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.formation.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i32>() {
                                                    Ok(value) => Msg::UpdateFormation(value),
                                                    Err(_) => Msg::UpdateFormation(0),
                                            }
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "..." }</td>
                                <td class="border px-4 py-2">{ "auto" }</td>
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

                    <hr class="my-1 border-t-2 border-orange-400 w-2/4" />

                    <table class="table-auto mb-4 border-collapse border-separate border border-gray-900 w-2/4">
                        <thead>
                            <tr class="bg-orange-100">
                                <th class="px-4 py-2">{ "Prestation" }</th>
                                <th class="px-4 py-2">{ "Production/Encaissement" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="border px-4 py-2">{ "Production - Encaissement / jour" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.prodjour.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i64>() {
                                                    Ok(value) => Msg::UpdateProdjour(value),
                                                    Err(_) => Msg::UpdateProdjour(0),
                                            }
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "" }</td>
                                <td class="border px-4 py-2">{ "" }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Production - Encaissement / an" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.prodan.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i64>() {
                                                    Ok(value) => Msg::UpdateProdan(value),
                                                    Err(_) => Msg::UpdateProdan(0),
                                            }
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "" }</td>
                                <td class="border px-4 py-2">{ "" }</td>
                            </tr>
                        </tbody>
                    </table>

                    <hr class="my-1 border-t-2 border-orange-400 w-2/4" />

                    <table class="table-auto mb-4 border-collapse border-separate border border-gray-900 w-2/4">
                        <thead>
                            <tr class="bg-orange-100">
                                <th class="px-4 py-2">{ "Moyenne prix de vente" }</th>
                                <th class="px-4 py-2">{ "HT" }</th>
                                <th class="px-4 py-2">{ "TVA" }</th>
                                <th class="px-4 py-2">{ "TTC" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="border px-4 py-2">{ "TVA applicable" }</td>
                                <td class="border px-4 py-2">{ "" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.tva.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let     input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateTva(value),
                                                    Err(_) => Msg::UpdateTva(0),
                                            }
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "" }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Moyenne prix de vente" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.moyprix.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i64>() {
                                                    Ok(value) => Msg::UpdateMoyPrix(value),
                                                    Err(_) => Msg::UpdateMoyPrix(0),
                                            }
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "auto" }</td>
                                <td class="border px-4 py-2">{ "auto" }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "CA journalier" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.cajour.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i64>() {
                                                    Ok(value) => Msg::UpdateCaJour(value),
                                                    Err(_) => Msg::UpdateCaJour(0),
                                            }
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "" }</td>
                                <td class="border px-4 py-2">{ "" }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "CA annuel" }</td>
                                <td class="border px-4 py-2">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.caann.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i64>() {
                                                    Ok(value) => Msg::UpdateCaAnn(value),
                                                    Err(_) => Msg::UpdateCaAnn(0),
                                            }
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ "auto" }</td>
                                <td class="border px-4 py-2">{ "auto" }</td>
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
