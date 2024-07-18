use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{HtmlInputElement};
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
    tva: i8,
    moyprix: f64,
    donttva: f64,
    totalservice: i64,
    totalmoyprix: f64,
    htcanann: f64,
    tvaann: f64,
    ttcann: f64,
    htjours: f64,
}

pub struct StepTwo {
    user_id: Option<i32>,
    production: i32,
    entretien: i32,
    clientele: i32,
    interprofession: i32,
    formation: i32,
    prodjour: i64,
    tva: i8,
    moyprix: f64,
    entreprise: Option<Entreprise>,
    clone_jrsttx: Option<i32>,
    pourcentagejrsent: i32,
    pourcetagenon: i32,
    totalservice: i64,
    donttva: f64,
    totalmoyprix: f64,
    htcanann: f64,
    tvaann: f64,
    ttcann: f64,
    htjours: f64,
    error_percent: Option<String>,
    error_totalstep1: Option<String>,
    total: i32,
    submitted: bool,
}

pub enum Msg {
    UpdateProduction(i32),
    UpdateEntretien(i32),
    UpdateClientele(i32),
    UpdateInterprofession(i32),
    UpdateFormation(i32),
    UpdateProdjour(i64),
    UpdateTva(i8),
    UpdateMoyPrix(f64),
    CalculatePouJTTX,
    CalculePourNon,
    CalculateTotalS1,
    UpdateTotalService,
    CalculateDontTva,
    CalculateMoyTtTva,
    CalculateCaAnnHt,
    CalculcateTvaAnn,
    CalculateTtcAnn,
    CalculateHtJours,
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
    jrsweek: i16,
    jrsferies: i8,
    jrscp: i8,
}

impl Component for StepTwo {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let user_id = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("user_id")
            .ok()
            .flatten()
            .and_then(|id| id.parse::<i32>().ok());

        //get API ent per user
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
            prodjour: 0,
            tva: 0,
            moyprix: 0.0,
            entreprise: None,
            clone_jrsttx: None,
            pourcentagejrsent: 0,
            pourcetagenon: 0,
            total: 0,
            totalservice: 0,
            donttva: 0.0,
            totalmoyprix: 0.0,
            htcanann: 0.0,
            tvaann: 0.0,
            ttcann: 0.0,
            htjours: 0.0,
            error_percent: None,
            error_totalstep1: None,
            submitted: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Mise à jour des valeurs
            Msg::UpdateProduction(value) => {
                self.production = value;
                ctx.link().send_message(Msg::CalculatePouJTTX);
                ctx.link().send_message(Msg::CalculateTotalS1);
                ctx.link().send_message(Msg::CalculateHtJours);
                true
            }
            Msg::UpdateEntretien(value) => {
                self.entretien = value;
                ctx.link().send_message(Msg::CalculePourNon);
                ctx.link().send_message(Msg::CalculateTotalS1);
                true
            }
            Msg::UpdateClientele(value) => {
                self.clientele = value;
                ctx.link().send_message(Msg::CalculePourNon);
                ctx.link().send_message(Msg::CalculateTotalS1);
                true
            }
            Msg::UpdateInterprofession(value) => {
                self.interprofession = value;
                ctx.link().send_message(Msg::CalculePourNon);
                ctx.link().send_message(Msg::CalculateTotalS1);
                true
            }
            Msg::UpdateFormation(value) => {
                self.formation = value;
                ctx.link().send_message(Msg::CalculePourNon);
                ctx.link().send_message(Msg::CalculateTotalS1);
                true
            }
            Msg::UpdateProdjour(value) => {
                self.prodjour = value;
                ctx.link().send_message(Msg::UpdateTotalService);
                true
            }
            Msg::UpdateTva(value) => {
                self.tva = value;
                ctx.link().send_message(Msg::CalculateDontTva);
                true
            }
            Msg::UpdateMoyPrix(value) => {
                self.moyprix = value;
                ctx.link().send_message(Msg::CalculateDontTva);
                ctx.link().send_message(Msg::CalculateMoyTtTva);
                ctx.link().send_message(Msg::CalculateCaAnnHt);
                ctx.link().send_message(Msg::CalculcateTvaAnn);
                ctx.link().send_message(Msg::CalculateTtcAnn);
                ctx.link().send_message(Msg::CalculateHtJours);
                true
            }

            Msg::CalculatePouJTTX => {
                if let Some(clone_jrsttx) = self.clone_jrsttx {
                    if clone_jrsttx != 0 {
                        if let Some(entreprise) = &self.entreprise {
                            self.pourcentagejrsent = ((self.production as f64) * 100.0 / (clone_jrsttx as f64 - entreprise.jrsweek as f64 - entreprise.jrscp as f64 - entreprise.jrsferies as f64)).round() as i32;
                        }
                    } else {
                        log::warn!("Pas de division possible, jrsttx == 0");
                        self.pourcentagejrsent = 0;
                    }
                } else {
                    log::warn!("clone_jrsttx == none");
                    self.pourcentagejrsent = 0;
                }
                true
            }
            Msg::CalculePourNon => {
                if let Some(clone_jrsttx) = self.clone_jrsttx {
                    if clone_jrsttx != 0 {
                        if let Some(entreprise) = &self.entreprise {
                            self.pourcetagenon = (((self.entretien + self.clientele + self.interprofession + self.formation) as f64) * 100.0 / (clone_jrsttx as f64 - entreprise.jrsweek as f64 - entreprise.jrscp as f64 - entreprise.jrsferies as f64)).round() as i32;
                        }
                    } else {
                        log::warn!("Pas de division possible, jrsttx == 0");
                        self.pourcetagenon = 0;
                    }
                } else {
                    log::warn!("clone_jrsttx == none");
                    self.pourcetagenon = 0;
                }
                true
            }
            Msg::CalculateTotalS1 => {
                if let Some(clone_jrsttx) = self.clone_jrsttx {
                    if let Some(entreprise) = &self.entreprise {
                        self.total = clone_jrsttx -
                            (entreprise.jrsweek as i32) -
                            (entreprise.jrscp as i32) -
                            (entreprise.jrsferies as i32) -
                            self.production -
                            self.entretien -
                            self.clientele -
                            self.interprofession -
                            self.formation;
                    }
                }
                true
            }
            // Calcul du total service
            Msg::UpdateTotalService => {
                if let Some(clone_jrsttx) = self.clone_jrsttx {
                    if let Some(entreprise) = &self.entreprise {
                        self.totalservice = self.production as i64 * self.prodjour;
                        log::info!("Total service updated: {}", self.totalservice);
                        if self.totalservice <= 0 {
                            self.totalservice = 0;
                        }
                    }
                }
                true
            }
            Msg::CalculateDontTva => {
                self.donttva = (self.moyprix * self.tva as f64) / 100.0;
                //log?
                log::info!("Dont TVA calculé: {}", self.donttva);
                true
            }
            Msg::CalculateMoyTtTva => {
                self.totalmoyprix = self.moyprix + self.donttva;
                true
            }
            Msg::CalculateCaAnnHt => {
                self.htcanann = self.totalservice as f64 * self.moyprix;
                true
            }
            Msg::CalculcateTvaAnn => {
                self.tvaann = (self.totalservice as f64 * self.moyprix) * self.tva as f64 / 100.0;
                true
            }
            Msg::CalculateTtcAnn => {
                self.ttcann = (self.totalservice as f64 * self.moyprix) + self.tvaann;
                true
            }
            Msg::CalculateHtJours => {
                self.htjours = (self.totalservice as f64 * self.moyprix) / self.production as f64;
                true
            }
            Msg::Submit => {
                if !self.submitted {
                    if self.pourcetagenon + self.pourcentagejrsent > 100 {
                        self.error_percent = Some("Erreur : Il n'est pas autorisé de dépasser le nombre de jours à positionner".to_string());
                        true
                    } else if self.total != 0 {
                        self.error_totalstep1 = Some("Erreur : Il vous reste des jours à positionner".to_string());
                        true
                    } else {
                        self.error_totalstep1 = None;
                        self.error_percent = None;
                        let activities = StepTwoo {
                            id: 0,
                            user_id: self.user_id.unwrap_or_default(),
                            production: self.production,
                            entretien: self.entretien,
                            clientele: self.clientele,
                            interprofession: self.interprofession,
                            formation: self.formation,
                            prodjour: self.prodjour,
                            tva: self.tva,
                            moyprix: self.moyprix,
                            donttva: self.donttva,
                            totalservice: self.totalservice,
                            totalmoyprix: self.totalmoyprix,
                            htcanann: self.htcanann,
                            tvaann: self.tvaann,
                            ttcann: self.ttcann,
                            htjours: self.htjours,
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
                    }
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
                ctx.link().send_message(Msg::CalculateTotalS1);
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
                                <th class="px-4 py-2 text-gray-700 font-semibold">{ "Répartition temps d'activité" }</th>
                                <th class="px-4 py-2 text-gray-700 font-semibold">{ "Nombre de jours" }</th>
                                <th class="px-4 py-2 text-gray-700 font-semibold">{ "Jours en Entreprise" }</th>
                                <th class="px-4 py-2 text-gray-700 font-semibold">{ "Pourcentage" }</th>
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
                                <td class="border px-4 py-2">{ self.production }</td>
                                <td class="border px-4 py-2">{""}</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Rentrée d'argent positive" }</td>
                                <td class="border px-4 py-2">{""}</td>
                                <td class="border px-4 py-2">{""}</td>
                                <td class="border px-4 py-2 text-emerald-600">{ format!("{}%", self.pourcentagejrsent) }</td>
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
                                <td class="border px-4 py-2">{ self.entretien }</td>
                                <td class="border px-4 py-2">{ "" }</td>
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
                                <td class="border px-4 py-2">{ self.clientele }</td>
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
                                <td class="border px-4 py-2">{ self.interprofession }</td>
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
                                <td class="border px-4 py-2">{ self.formation }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Rentrée d'argent nulle" }</td>
                                <td class="border px-4 py-2">{""}</td>
                                <td class="border px-4 py-2">{""}</td>
                                <td class="border px-4 py-2 text-red-600">{ format!("{}%", self.pourcetagenon) }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "" }</td>
                                <td class="border px-4 py-2">
                                    {
                                        if let Some(ref message) = self.error_totalstep1 {
                                            html! {
                                                <div class="mb-2 text-center text-sm font-semibold text-red-500">
                                                    { message }
                                                </div>
                                            }
                                        } else {
                                            html! { <></> }
                                        }
                                    }
                                </td>
                                <td class="border px-4 py-2">{ self.view_total_form() }</td>
                                <td class="border px-4 py-2">{
                            if let Some(ref message) = self.error_percent {
                                html! {
                                    <div class="mb-2 text-center text-sm font-semibold text-red-500">
                                        { message }
                                    </div>
                                }
                            } else {
                                html! { <></> }
                            }
                        }</td>
                            </tr>
                        </tbody>
                    </table>

                    <hr class="my-1 border-t-2 border-orange-400 w-2/4" />

                    <table class="table-auto mb-4 border-collapse border-separate border border-gray-900 w-2/4">
                        <thead>
                            <tr class="bg-orange-100">
                                <th class="px-4 py-2 text-gray-700 font-semibold">{ "Prestation" }</th>
                                <th class="px-4 py-2 text-gray-700 font-semibold">{ "Production/Encaissement" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="border px-4 py-2">{ "Production - Service - Vente / jour" }</td>
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
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "Production - Service - Vente / an" }</td>
                                <td class="border px-4 py-2 text-emerald-600 text-right">{format!("{}", self.totalservice)}</td>
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
                                            let input: HtmlInputElement = e.target_unchecked_into();
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
                                        value={format!("{:.2}", self.moyprix).replace('.', ",")}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            let value_str = input.value().replace(',', ".");
                                            match value_str.parse::<f64>() {
                                                Ok(value) => Msg::UpdateMoyPrix(value),
                                                Err(_) => Msg::UpdateMoyPrix(0.0),
                                            }
                                        })}
                                    />
                                </td>
                                <td class="border px-4 py-2">{ self.donttva.to_string() }</td>
                                <td class="border px-4 py-2">{ self.totalmoyprix.to_string() }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "CA journalier" }</td>
                                <td class="border px-4 py-2">{ self.htjours.to_string() }</td>
                                <td class="border px-4 py-2">{ "" }</td>
                                <td class="border px-4 py-2">{ "" }</td>
                            </tr>
                            <tr>
                                <td class="border px-4 py-2">{ "CA annuel" }</td>
                                <td class="border px-4 py-2">{ self.htcanann.to_string() }</td>
                                <td class="border px-4 py-2">{ self.tvaann.to_string() }</td>
                                <td class="border px-4 py-2 text-orange-400">{ self.ttcann.to_string() }</td>
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
            if let Some(ref entreprise) = self.entreprise {
                html! {
                    <p class="text-gray-700">
                        { format!(
                            "Jours travaillés: {}",
                            cloned_jrsttx - (entreprise.jrsweek as i32) - (entreprise.jrscp as i32) - (entreprise.jrsferies as i32)
                        ) }
                    </p>
                }
            } else {
                html! { <p class="text-gray-700">{ "Loading entreprise data..." }</p> }
            }
        } else {
            html! { <p class="text-gray-700">{ "No cloned jrsttx available" }</p> }
        }
    }

    fn view_total_form(&self) -> Html{
        html!{
            <div class="mb-2 text-center text-sm font-semibold text-gray-700">
                            { "Il vous reste " }
                            <div class="text-red-500">
                                { self.total }
                            </div>
                            <div class="mb-2 text-center text-sm font-semibold text-gray-700">
                                {"jours à positionner"}
                            </div>
                        </div>
        }
    }
}
