use log::log;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{HtmlInputElement, console};
use serde::{Serialize, Deserialize};
use reqwasm::http::Request;
use crate::{AppRoute, header, footer};

// Définition des structures pour les activités et les entreprises
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Activites {
    id: i32,
    user_id: i32,
    production: i32,
    entretien: i32,
    clientele: i32,
    interprofession: i32,
    formation: i32,
    prodjour: i64,
    tva: f32,
    moyprix: f64,
    donttva: f64,
    totalservice: i64,
    totalmoyprix: f64,
    htcanann: f64,
    tvaann: f64,
    ttcann: f64,
    htjours: f64,
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
    jan: i8,
    fev: i8,
    mar: i8,
    avr: i8,
    mai: i8,
    juin: i8,
    jui: i8,
    aout: i8,
    sept: i8,
    oct: i8,
    nov: i8,
    dec: i8,
}

// Enumération des messages pour la gestion des états
pub enum Msg {
    LoadActivites(Activites),
    LoadActivitesError,
    LoadEntreprise(Entreprise),
    LoadEntrepriseError,
}

// Définition du composant recap
pub struct RecapOne {
    user_id: Option<i32>,
    current_step: usize,
    activites: Option<Activites>,
    entreprise: Option<Entreprise>,
}

impl Component for RecapOne {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Récupération de l'ID utilisateur depuis le stockage local
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
            log::info!("User ID found: {}", user_id);

            // Requête pour les activités
            ctx.link().send_future(async move {
                let url = format!("http://localhost:8080/get_act?user_id={}", user_id);
                log::info!("Requesting activities from: {}", url);
                let response = Request::get(&url).send().await.unwrap();
                if response.ok() {
                    log::info!("Successfully fetched activities");
                    let activites: Activites = response.json().await.unwrap();
                    Msg::LoadActivites(activites)
                } else {
                    log::error!("Failed to fetch activities with status: {}", response.status());
                    Msg::LoadActivitesError
                }
            });

            // Requête pour les entreprises
            ctx.link().send_future(async move {
                let url = format!("http://localhost:8080/get_ent?user_id={}", user_id);
                log::info!("Requesting entreprises from: {}", url);
                let response = Request::get(&url).send().await.unwrap();
                if response.ok() {
                    log::info!("Successfully fetched entreprises");
                    let entreprise: Entreprise = response.json().await.unwrap();
                    Msg::LoadEntreprise(entreprise)
                } else {
                    log::error!("Failed to fetch entreprises with status: {}", response.status());
                    Msg::LoadEntrepriseError
                }
            });
        } else {
            log::error!("No user ID found in local storage");
        }

        Self {
            user_id,
            current_step: 1,
            activites: None,
            entreprise: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadActivites(activites) => {
                self.activites = Some(activites);
                true
            }
            Msg::LoadActivitesError => {
                self.activites = None;
                true
            }
            Msg::LoadEntreprise(entreprise) => {
                self.entreprise = Some(entreprise);
                true
            }
            Msg::LoadEntrepriseError => {
                self.entreprise = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col min-h-screen">
                {header()}
                <div class="bg-orange-50 flex flex-col flex-grow justify-center items-center">
                    <div class="flex flex-row w-full justify-center">
                        {self.view_activites()}
                        {self.view_entreprise()}
                    </div>
                </div>
                {footer()}
            </div>
        }
    }
}

impl RecapOne {
    fn view_activites(&self) -> Html {
        if let Some(activites) = &self.activites {
            html! {
                <div>
                    <p>{ format!("ID: {}", activites.id) }</p>
                    <p>{ format!("User ID: {}", activites.user_id) }</p>
                    <p>{ format!("Production: {}", activites.production) }</p>
                    <p>{ format!("Entretien: {}", activites.entretien) }</p>
                    <p>{ format!("Clientele: {}", activites.clientele) }</p>
                    <p>{ format!("Interprofession: {}", activites.interprofession) }</p>
                    <p>{ format!("Formation: {}", activites.formation) }</p>
                    <p>{ format!("Prodjour: {}", activites.prodjour) }</p>
                    <p>{ format!("TVA: {}", activites.tva) }</p>
                    <p>{ format!("Moyprix: {}", activites.moyprix) }</p>
                    <p>{ format!("Donttva: {}", activites.donttva) }</p>
                    <p>{ format!("Totalservice: {}", activites.totalservice) }</p>
                    <p>{ format!("Totalmoyprix: {}", activites.totalmoyprix) }</p>
                    <p>{ format!("Htcanann: {}", activites.htcanann) }</p>
                    <p>{ format!("Tvaann: {}", activites.tvaann) }</p>
                    <p>{ format!("Ttcann: {}", activites.ttcann) }</p>
                    <p>{ format!("Htjours: {}", activites.htjours) }</p>
                </div>
            }
        } else {
            html! {
                <p>{ "Loading activities..." }</p>
            }
        }
    }

    fn view_entreprise(&self) -> Html {
        if let Some(entreprise) = &self.entreprise {
            html! {
                <div>
                    <p>{ format!("ID: {}", entreprise.id) }</p>
                    <p>{ format!("User ID: {}", entreprise.user_id) }</p>
                    <p>{ format!("Name: {}", entreprise.name) }</p>
                    <p>{ format!("Date: {}", entreprise.date) }</p>
                    <p>{ format!("Code APE: {}", entreprise.codeape) }</p>
                    <p>{ format!("Jours travaillés: {}", entreprise.jrsttx) }</p>
                    <p>{ format!("Jours semaine: {}", entreprise.jrsweek) }</p>
                    <p>{ format!("Jours fériés: {}", entreprise.jrsferies) }</p>
                    <p>{ format!("Jours CP: {}", entreprise.jrscp) }</p>
                    <p>{ format!("Janvier: {}", entreprise.jan) }</p>
                    <p>{ format!("Février: {}", entreprise.fev) }</p>
                    <p>{ format!("Mars: {}", entreprise.mar) }</p>
                    <p>{ format!("Avril: {}", entreprise.avr) }</p>
                    <p>{ format!("Mai: {}", entreprise.mai) }</p>
                    <p>{ format!("Juin: {}", entreprise.juin) }</p>
                    <p>{ format!("Juillet: {}", entreprise.jui) }</p>
                    <p>{ format!("Août: {}", entreprise.aout) }</p>
                    <p>{ format!("Septembre: {}", entreprise.sept) }</p>
                    <p>{ format!("Octobre: {}", entreprise.oct) }</p>
                    <p>{ format!("Novembre: {}", entreprise.nov) }</p>
                    <p>{ format!("Décembre: {}", entreprise.dec) }</p>
                </div>
            }
        } else {
            html! {
                <p>{ "Loading entreprises..." }</p>
            }
        }
    }
}
