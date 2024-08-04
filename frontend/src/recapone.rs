use std::fmt::format;
use log::log;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{HtmlInputElement, HtmlCanvasElement, console};
use serde::{Serialize, Deserialize};
use reqwasm::http::Request;
use crate::{AppRoute, header, footer};
use crate::utils::{Entreprise, User};
use plotters::prelude::*;
use plotters::style::full_palette::{GREY_A700, ORANGE_100, ORANGE_200, ORANGE_50, ORANGE_500};
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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


pub enum Msg {
    LoadActivites(Activites),
    LoadActivitesError,
    LoadEntreprise(Entreprise),
    LoadEntrepriseError,
    LoadUsers(User),
    LoadUsersError,
    DrawChart,
}

pub struct RecapOne {
    user_id: Option<i32>,
    current_step: usize,
    activites: Option<Activites>,
    entreprise: Option<Entreprise>,
    users: Option<User>,
}

impl Component for RecapOne {
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

        if let Some(user_id) = user_id {
            log::info!("User ID found: {}", user_id);

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

            ctx.link().send_future(async move {
                let url = format!("http://localhost:8080/get_user?user_id={}", user_id);
                log::info!("Requesting users from: {}", url);
                let response = Request::get(&url).send().await.unwrap();
                if response.ok() {
                    log::info!("Successfully fetched users");
                    let user: User = response.json().await.unwrap();
                    Msg::LoadUsers(user)
                } else {
                    log::error!("Failed to fetch users with status: {}", response.status());
                    Msg::LoadUsersError
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
            users: None,
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
            Msg::LoadUsers(users) => {
                self.users = Some(users);
                true
            }
            Msg::LoadUsersError => {
                self.users = None;
                true
            }
            Msg::DrawChart => {
                if let Some(ref entreprise) = self.entreprise {
                    self.draw_chart(entreprise);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            if let Some(entreprise) = &self.entreprise{
                <div class="flex flex-col min-h-screen">
                {header()}
                <div class="bg-orange-50 flex flex-col flex-grow justify-center items-center">
                    <div class="flex flex-row w-full justify-center">
                        <h1>{format!("donnees de l'entreprise {}", entreprise.name)}</h1>
                        {self.view_activites()}
                        {self.view_entreprise()}
                        {self.view_users()}
                        <canvas id="chart" width=800 height=600></canvas>
                    </div>
                    <div>
                        <button onclick={ctx.link().callback(|_| Msg::DrawChart)}>{ "Jours Travaillés" }</button>
                    </div>
                </div>
                {footer()}
            </div>
            }
        }
    }
}

impl RecapOne {
    fn view_activites(&self) -> Html {
        if let Some(activites) = &self.activites {
            html! {
                <div>
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
                    <h1>{format!("Donnees de l'entreprise: {}",entreprise.name)}</h1>
                    <p>{ format!("Name: {}", entreprise.name) }</p>
                    <p>{ format!("Date: {}", entreprise.date) }</p>
                    <p>{ format!("Code APE: {}", entreprise.codeape) }</p>
                    <p>{format!("Status: {}", entreprise.status)}</p>
                    <p>{ format!("Jours travaillés: {}", entreprise.jrsttx) }</p>
                    <p>{ format!("Jours week-end: {}", entreprise.jrsweek) }</p>
                    <p>{ format!("Jours fériés: {}", entreprise.jrsferies) }</p>
                    <p>{ format!("Jours CP: {}", entreprise.jrscp) }</p>
                </div>
            }
        } else {
            html! {
                <p>{ "Loading entreprises..." }</p>
            }
        }
    }

    fn view_users(&self) -> Html {
        if let Some(users) = &self.users {
            html! {
                <div>
                    <p>{ format!("Lastname: {}", users.lastname) }</p>
                    <p>{ format!("Firstname: {}", users.firstname) }</p>
                </div>
            }
        } else {
            html! {
                <p>{ "Loading users..." }</p>
            }
        }
    }

    fn draw_chart(&self, entreprise: &Entreprise) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("chart")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();

        root.fill(&WHITE).unwrap();

        let data = vec![
            (0, entreprise.jan as i32),
            (1, entreprise.fev as i32),
            (2, entreprise.mar as i32),
            (3, entreprise.avr as i32),
            (4, entreprise.mai as i32),
            (5, entreprise.juin as i32),
            (6, entreprise.jui as i32),
            (7, entreprise.aout as i32),
            (8, entreprise.sept as i32),
            (9, entreprise.oct as i32),
            (10, entreprise.nov as i32),
            (11, entreprise.dec as i32),
        ];

        let max_value = data.iter().map(|&(_, y)| y).max().unwrap_or(0);
        let months = [
            "Jan", "Fev", "Mar", "Avr", "Mai", "Jui",
            "Jui", "Aou", "Sep", "Oct", "Nov", "Dec"
        ];

        let mut chart = ChartBuilder::on(&root)
            .caption("Jours Travaillés par Mois", ("sans-serif", 20).into_font())
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..11, 0..max_value)
            .unwrap();

        chart.configure_mesh()
            .x_labels(12)
            .x_label_formatter(&|&x| {
                if x < 12 {
                    months[x as usize].to_string()
                } else {
                    "".to_string()
                }
            })
            .x_desc("Mois")
            .y_desc("N jours")
            .draw()
            .unwrap();

        chart
            .draw_series(
                AreaSeries::new(
                    data.iter().map(|&(x,y)|(x,y)),
                    0,
                    &ORANGE_100.mix(0.4),
                )
                    .border_style(&ORANGE_500),
            ).unwrap();

        chart
            .draw_series(
                data.iter()
                    .map(|&(x, y)| Circle::new((x, y), 5, ORANGE_500.filled())),
            )
            .unwrap();
        root.present().unwrap();
    }
}
