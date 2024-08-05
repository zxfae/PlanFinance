use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{HtmlInputElement};
use reqwasm::http::Request;
use crate::{AppRoute, header, footer};
use crate::modals::{Entreprise, Activities, FormActivities, ActivitiesMsg};

impl Component for FormActivities {
    type Message = ActivitiesMsg;
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
                    ActivitiesMsg::LoadEntreprise(entreprise)
                } else {
                    log::error!("Failed to fetch entreprise");
                    ActivitiesMsg::LoadEntrepriseError
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
            tva: -1.0,
            moyprix: 0.0,
            entreprise: None,
            clone_jrsttx: None,
            pourcentagejrsent: 0.0,
            pourcetagenon: 0.0,
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
            error_tva: None,
            submitted: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Mise à jour des valeurs
            ActivitiesMsg::UpdateProduction(value) => {
                self.production = value;
                ctx.link().send_message(ActivitiesMsg::CalculatePouJTTX);
                ctx.link().send_message(ActivitiesMsg::CalculateTotalS1);
                ctx.link().send_message(ActivitiesMsg::CalculateHtJours);
                true
            }
            ActivitiesMsg::UpdateEntretien(value) => {
                self.entretien = value;
                ctx.link().send_message(ActivitiesMsg::CalculePourNon);
                ctx.link().send_message(ActivitiesMsg::CalculateTotalS1);
                true
            }
            ActivitiesMsg::UpdateClientele(value) => {
                self.clientele = value;
                ctx.link().send_message(ActivitiesMsg::CalculePourNon);
                ctx.link().send_message(ActivitiesMsg::CalculateTotalS1);
                true
            }
            ActivitiesMsg::UpdateInterprofession(value) => {
                self.interprofession = value;
                ctx.link().send_message(ActivitiesMsg::CalculePourNon);
                ctx.link().send_message(ActivitiesMsg::CalculateTotalS1);
                true
            }
            ActivitiesMsg::UpdateFormation(value) => {
                self.formation = value;
                ctx.link().send_message(ActivitiesMsg::CalculePourNon);
                ctx.link().send_message(ActivitiesMsg::CalculateTotalS1);
                true
            }
            ActivitiesMsg::UpdateProdjour(value) => {
                self.prodjour = value;
                ctx.link().send_message(ActivitiesMsg::UpdateTotalService);
                true
            }
            ActivitiesMsg::UpdateTva(value) => {
                self.tva = value;
                ctx.link().send_message(ActivitiesMsg::CalculateDontTva);
                true
            }
            ActivitiesMsg::UpdateMoyPrix(value) => {
                self.moyprix = value;
                ctx.link().send_message(ActivitiesMsg::CalculateDontTva);
                ctx.link().send_message(ActivitiesMsg::CalculateMoyTtTva);
                ctx.link().send_message(ActivitiesMsg::CalculateCaAnnHt);
                ctx.link().send_message(ActivitiesMsg::CalculcateTvaAnn);
                ctx.link().send_message(ActivitiesMsg::CalculateTtcAnn);
                ctx.link().send_message(ActivitiesMsg::CalculateHtJours);
                true
            }

            ActivitiesMsg::CalculatePouJTTX => {
                if let Some(clone_jrsttx) = self.clone_jrsttx {
                    if clone_jrsttx > 0 {
                        if let Some(entreprise) = &self.entreprise {
                            let denominator = clone_jrsttx as f32 - entreprise.jrsweek as f32 - entreprise.jrscp as f32 - entreprise.jrsferies as f32;
                            log::info!("Calculating percentage: production = {}, denominator = {}", self.production, denominator);
                            if denominator > 0.0 {
                                self.pourcentagejrsent = ((self.production as f32 * 100.0) / denominator * 100.0).round() / 100.0;
                                log::info!("Updated percentage: {:.2}", self.pourcentagejrsent);
                            } else {
                                log::warn!("Denominator is zero or negative, cannot compute percentage");
                                self.pourcentagejrsent = 0.0;
                            }
                        } else {
                            log::warn!("entreprise data is None");
                            self.pourcentagejrsent = 0.0;
                        }
                    } else {
                        log::warn!("clone_jrsttx is zero or negative");
                        self.pourcentagejrsent = 0.0;
                    }
                } else {
                    log::warn!("clone_jrsttx is None");
                    self.pourcentagejrsent = 0.0;
                }
                true
            }
            ActivitiesMsg::CalculePourNon => {
                if let Some(clone_jrsttx) = self.clone_jrsttx {
                    if clone_jrsttx > 0 {
                        if let Some(entreprise) = &self.entreprise {
                            let denominator = clone_jrsttx as f32 - entreprise.jrsweek as f32 - entreprise.jrscp as f32 - entreprise.jrsferies as f32;
                            log::info!("Calculating percentage for non-productive days: total = {}, denominator = {}", self.entretien + self.clientele + self.interprofession + self.formation, denominator);
                            if denominator > 0.0 {
                                self.pourcetagenon = (((self.entretien + self.clientele + self.interprofession + self.formation) as f32 * 100.0) / denominator * 100.0).round() / 100.0;
                                log::info!("Updated non-productive percentage: {:.2}", self.pourcetagenon);
                            } else {
                                log::warn!("Denominator is zero or negative, cannot compute non-productive percentage");
                                self.pourcetagenon = 0.0;
                            }
                        } else {
                            log::warn!("entreprise data is None");
                            self.pourcetagenon = 0.0;
                        }
                    } else {
                        log::warn!("clone_jrsttx is zero or negative");
                        self.pourcetagenon = 0.0;
                    }
                } else {
                    log::warn!("clone_jrsttx is None");
                    self.pourcetagenon = 0.0;
                }
                true
            }
            ActivitiesMsg::CalculateTotalS1 => {
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
            ActivitiesMsg::UpdateTotalService => {
                self.totalservice = self.production as i64 * self.prodjour;
                log::info!("Total service updated: {}", self.totalservice);
                if self.totalservice <= 0 {
                    self.totalservice = 0;
                }
                true
            }
            ActivitiesMsg::CalculateDontTva => {
                self.donttva = (self.moyprix * self.tva as f64) / 100.0;
                if self.donttva <= 0.0{
                    self.donttva = 0.0;
                }
                true
            }
            ActivitiesMsg::CalculateMoyTtTva => {
                self.totalmoyprix = self.moyprix + self.donttva;
                true
            }
            ActivitiesMsg::CalculateCaAnnHt => {
                self.htcanann = self.totalservice as f64 * self.moyprix;
                true
            }
            ActivitiesMsg::CalculcateTvaAnn => {
                self.tvaann = (self.totalservice as f64 * self.moyprix) * self.tva as f64 / 100.0;
                if self.tvaann <= 0.0{
                    self.tvaann = 0.0;
                }
                true
            }
            ActivitiesMsg::CalculateTtcAnn => {
                self.ttcann = (self.totalservice as f64 * self.moyprix) + self.tvaann;
                true
            }
            ActivitiesMsg::CalculateHtJours => {
                self.htjours = (self.totalservice as f64 * self.moyprix) / self.production as f64;
                true
            }
            ActivitiesMsg::Submit => {
                if !self.submitted {
                    if self.pourcetagenon + self.pourcentagejrsent  > 100.0 {
                        self.error_percent = Some("Il n'est pas autorisé de dépasser le nombre de jours à positionner".to_string());
                        true
                    } else if self.total != 0 {
                        self.error_totalstep1 = Some("Mauvais positionnement".to_string());
                        true
                    } else if self.tva == -1.0{
                        self.error_tva = Some("Mettez à jour votre TVA".to_string());
                        true
                    } else if self.moyprix == 0.0 {
                        true
                    }else {
                        self.error_percent = None;
                        self.error_totalstep1 =None;
                        self.error_tva = None;
                        let activities = Activities {
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
                                        let new_activities: Result<Activities, _> = resp.json().await;
                                        match new_activities {
                                            Ok(new_activities) => ActivitiesMsg::SubmissionComplete(new_activities),
                                            Err(e) => {
                                                log::error!("Failed to parse response: {:?}", e);
                                                ActivitiesMsg::Submit
                                            }
                                        }
                                    } else {
                                        log::error!("Failed to submit activities: {}", resp.status());
                                        ActivitiesMsg::Submit
                                    }
                                }
                                Err(e) => {
                                    log::error!("Request failed: {:?}", e);
                                    ActivitiesMsg::Submit
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
            ActivitiesMsg::SubmissionComplete(new_activities) => {
                log::info!("Submission completed.");
                web_sys::window()
                    .unwrap()
                    .local_storage()
                    .unwrap()
                    .unwrap()
                    .set_item("user_id", &new_activities.id.to_string())
                    .unwrap();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::RecapOne);
                true
            }
            ActivitiesMsg::LoadEntreprise(entreprise) => {
                self.clone_jrsttx = Some(entreprise.jrsttx);
                self.entreprise = Some(entreprise);
                ctx.link().send_message(ActivitiesMsg::CalculateTotalS1);
                true
            }
            ActivitiesMsg::LoadEntrepriseError => {
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
                <div class="drop-shadow-md text-center text-gray-600 text-2xl font-semibold m-5">
                    <h1>{ "Répartition Temps de Travail / d'activité" }</h1>
                    <div class="text-center text-gray-600 text-2xl font-semibold m-2">
                        <h1>{ "Nous défendons l'idée que chacun peut créer son business plan facilement et gratuitement" }</h1>
                    </div>
                </div>
                <table class="table-auto mb-2 border-collapse border-separate border-2 border-orange-400 w-2/4">
                    <thead>
                        <tr class="bg-orange-100">
                            <th class="border-solid border-2 px-4 py-2 text-gray-700 font-semibold text-lg">{ "Répartition temps d'activité" }</th>
                            <th class="px-4 py-2 border-solid border-2 text-gray-700 font-semibold text-lg">{ "Nombre de jours" }</th>
                            <th class="px-4 py-2 border-solid border-2 text-gray-700 font-semibold text-lg">{ "Jours en Entreprise" }</th>
                            <th class="px-4 py-2 border-solid border-2 text-gray-700 font-semibold text-lg">{ "Pourcentage" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "Production - vente = CA" }</td>
                            <td class="border-solid border-2 text-right text-zinc-600 text-base font-semibold px-4 py-2">
                                <input
                                    class="shadow border rounded w-full py-2 px-3 text-gray-700 focus:outline-none focus:shadow-outline"
                                    type="text"
                                    value={self.production.to_string()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        match input.value().parse::<i32>() {
                                            Ok(value) => ActivitiesMsg::UpdateProduction(value),
                                            Err(_) => ActivitiesMsg::UpdateProduction(0),
                                        }
                                    })}
                                required=true
                                />
                            </td>
                            <td class="border-solid border-2 bg-white/50 text-right text-zinc-600 text-base font-semibold px-4 py-2">{ self.production }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">{ format!("{:.2}%", self.pourcentagejrsent) }</td>
                        </tr>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "Rentrée d'argent positive" }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">{""}</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">{""}</td>
                            <td class="border-solid border-2 bg-white/50 text-right text-emerald-600 text-base font-semibold px-4 py-2">{ format!("{:.2}%", self.pourcentagejrsent) }</td>
                        </tr>
                    </tbody>
                </table>
                <table class="table-auto mb-2 border-collapse border-separate border-2 border-orange-400 w-2/4">
                    <thead>
                        <tr class="bg-orange-100">
                            <th class="border-solid border-2 px-4 py-2 text-gray-700 font-semibold text-lg">{ "Répartition temps d'activité" }</th>
                            <th class="px-4 py-2 border-solid border-2 text-gray-700 font-semibold text-lg">{ "Nombre de jours" }</th>
                            <th class="px-4 py-2 border-solid border-2 text-gray-700 font-semibold text-lg">{ "Jours en Entreprise" }</th>
                            <th class="px-4 py-2 border-solid border-2 text-gray-700 font-semibold text-lg">{ "Pourcentage" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "Entretien / Maintenance" }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                    type="text"
                                    value={self.entretien.to_string()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        match input.value().parse::<i32>() {
                                            Ok(value) => ActivitiesMsg::UpdateEntretien(value),
                                            Err(_) => ActivitiesMsg::UpdateEntretien(0),
                                        }
                                    })}
                                    required=true
                                />
                            </td>
                            <td class="border-solid border-2 bg-white/50 text-right text-zinc-600 text-base font-semibold px-4 py-2">{ self.entretien }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">{""}</td>
                        </tr>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "Gestion, Devis, Facture" }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                    type="text"
                                    value={self.clientele.to_string()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        match input.value().parse::<i32>() {
                                            Ok(value) => ActivitiesMsg::UpdateClientele(value),
                                            Err(_) => ActivitiesMsg::UpdateClientele(0),
                                        }
                                    })}
                                    required=true
                                />
                            </td>
                            <td class="border-solid border-2 bg-white/50 text-right text-zinc-600 text-base font-semibold px-4 py-2">{ self.clientele }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">{""}</td>
                        </tr>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "Interprofession" }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                    type="text"
                                    value={self.interprofession.to_string()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        match input.value().parse::<i32>() {
                                            Ok(value) => ActivitiesMsg::UpdateInterprofession(value),
                                            Err(_) => ActivitiesMsg::UpdateInterprofession(0),
                                        }
                                    })}
                                    required=true
                                />
                            </td>
                            <td class="border-solid border-2 bg-white/50 text-right text-zinc-600 text-base font-semibold px-4 py-2">{ self.interprofession }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">{""}</td>
                        </tr>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "Formation" }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                    type="text"
                                    value={self.formation.to_string()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        match input.value().parse::<i32>() {
                                            Ok(value) => ActivitiesMsg::UpdateFormation(value),
                                            Err(_) => ActivitiesMsg::UpdateFormation(0),
                                        }
                                    })}
                                    required=true
                                />
                            </td>
                            <td class="border-solid border-2 bg-white/50 text-right text-zinc-600 text-base font-semibold px-4 py-2">{ self.formation }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">{""}</td>
                        </tr>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "Rentrée d'argent nulle" }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">{""}</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">{""}</td>
                            <td class="border-solid border-2 bg-white/50 text-right text-red-600 text-base font-semibold px-4 py-2">{ format!("{:.2}%", self.pourcetagenon) }</td>
                        </tr>
                        <tr>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2"></td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">
                                {
                                    if let Some(ref message) = self.error_totalstep1 {
                                        html! {
                                            <div class="text-center text-sm font-semibold text-red-500">
                                                { message }
                                            </div>
                                        }
                                    } else {
                                        html! { <></> }
                                    }
                                }
                            </td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">{ self.view_total_form() }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">
                        {
                        if let Some(ref message) = self.error_percent {
                            html! {
                                <div class="text-center text-sm font-semibold text-red-500">
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
                <table class="table-auto mb-2 border-collapse border-separate border-2 border-orange-400 w-2/4">
                    <thead>
                        <tr class="bg-orange-100">
                            <th class="border-solid border-2 px-4 py-2 text-gray-700 font-semibold text-lg">{ "Prestation" }</th>
                            <th class="px-4 py-2 border-solid border-2 text-gray-700 font-semibold text-lg">{ "Production/Encaissement" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "Production - Service - Vente / jour" }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                    type="text"
                                    value={self.prodjour.to_string()}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        match input.value().parse::<i64>() {
                                            Ok(value) => ActivitiesMsg::UpdateProdjour(value),
                                            Err(_) => ActivitiesMsg::UpdateProdjour(0),
                                        }
                                    })}
                                />
                            </td>
                        </tr>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "Production - Service - Vente / an" }</td>
                            <td class="border-solid border-2 bg-white/50 text-right  text-zinc-600 text-base font-semibold px-4 py-2">{self.totalservice}</td>
                        </tr>
                    </tbody>
                </table>
                <table class="table-auto mb-4 border-collapse border-separate border-2 border-orange-400 w-2/4">
                    <thead>
                        <tr class="bg-orange-100">
                            <th class="border-solid border-2 px-4 py-2 text-gray-700 font-semibold text-lg">{ "Moyenne prix de vente" }</th>
                            <th class="border-solid border-2 px-4 py-2 text-gray-700 font-semibold text-lg">{ "HT" }</th>
                            <th class="border-solid border-2 px-4 py-2 text-gray-700 font-semibold text-lg">{ "TVA" }</th>
                            <th class="border-solid border-2 px-4 py-2 text-gray-700 font-semibold text-lg">{ "TTC" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "TVA applicable" }</td>
                            <td class="border px-4 py-2">
                                {
                                    if let Some(ref message) = self.error_tva {
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
                            <td class="border px-4 py-2">
                             <div class="relative inline-block w-full">
                                <select
                                    class="block appearance-none w-full bg-white border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline"
                                    value={self.tva.to_string()}
                                    onchange={ctx.link().callback(|e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        match input.value().parse::<f32>() {
                                            Ok(value) => ActivitiesMsg::UpdateTva(value),
                                            Err(_) => ActivitiesMsg::UpdateTva(0.0),
                                        }
                                    })}
                                >
                                    <option value="5.5">{ "5.5%" }</option>
                                    <option value="10">{ "10%" }</option>
                                    <option value="20">{ "20%" }</option>
                                    <option value="-1.0">{"Choix TVA"}</option>
                                </select>
                                <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                                    <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M10 12l-5-5h10l-5 5z"/></svg>
                                </div>
                            </div>
                            </td>
                            <td class="border px-4 py-2">{ "" }</td>
                        </tr>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "Moyenne prix de vente" }</td>
                            <td class="border-solid border-2 text-zinc-600 text-base font-semibold px-4 py-2">
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                    type="text"
                                    value={format!("{:.2}", self.moyprix).replace('.', ",")}
                                    oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        let value_str = input.value().replace(',', ".");
                                        match value_str.parse::<f64>() {
                                            Ok(value) => ActivitiesMsg::UpdateMoyPrix(value),
                                            Err(_) => ActivitiesMsg::UpdateMoyPrix(0.0),
                                        }
                                    })}
                                />
                            </td>
                            <td class="border-solid border-2 bg-white/50 text-right text-zinc-600 text-base font-semibold px-4 py-2">{ self.donttva.to_string() }</td>
                            <td class="border-solid border-2 bg-white/50 text-right text-zinc-600 text-base font-semibold px-4 py-2">{ self.totalmoyprix.to_string() }</td>
                        </tr>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "CA journalier" }</td>
                            <td class="border-solid border-2 bg-white/50 text-right text-zinc-600 text-base font-semibold px-4 py-2">{ self.htjours.to_string() }</td>
                            <td class="border px-4 py-2">{ "" }</td>
                            <td class="border px-4 py-2">{ "" }</td>
                        </tr>
                        <tr>
                            <td class="border-solid border-2 bg-white text-zinc-600 text-base font-semibold px-4 py-2">{ "CA annuel" }</td>
                            <td class="border-solid border-2 text-right bg-white/50 text-zinc-600 text-base font-semibold px-4 py-2">{ self.htcanann.to_string() }</td>
                            <td class="border-solid border-2 text-right bg-white/50 text-zinc-600 text-base font-semibold px-4 py-2">{ self.tvaann.to_string() }</td>
                            <td class="border-solid border-2 text-right bg-white/50 text-zinc-600 text-base font-semibold px-4 py-2">{ self.ttcann.to_string() }</td>
                        </tr>
                    </tbody>
                </table>

                <div class="w-full max-w-md">
                    <form class="mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                        e.prevent_default();
                        ActivitiesMsg::Submit
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

impl FormActivities {

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
