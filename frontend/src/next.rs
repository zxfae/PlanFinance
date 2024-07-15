use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
#[warn(unused_imports)]
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use crate::{AppRoute, header, footer};
extern crate regex;
use regex::Regex;

//Rust.doc (-) currentFormat
pub fn date_test(date: &str) -> bool {
    let date_regex = Regex::new(r"^\d{2}-\d{2}-\d{4}$").unwrap();
    date_regex.is_match(date)
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

pub struct FormEntreprise {
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
    submitted: bool,
    current_step: usize,
    decompte: i32,
    total: i32,
    error_msg: Option<String>,
    date_err:Option<String>,
    oth_err: Option<String>,
}

pub enum Msg {
    UpdateName(String),
    UpdateDate(String),
    UpdateCodeApe(String),
    UpdateStatus(String),
    UpdateJrsTTX(i32),
    UpdateJrsWeek(i16),
    UpdateJrsFeries(i8),
    UpdateJrsCp(i8),
    UpdateJan(i8),
    UpdateFev(i8),
    UpdateMar(i8),
    UpdateAvr(i8),
    UpdateMai(i8),
    UpdateJuin(i8),
    UpdateJui(i8),
    UpdateAout(i8),
    UpdateSept(i8),
    UpdateOct(i8),
    UpdateNov(i8),
    UpdateDec(i8),
    CalculateDecompte,
    CalculateTotal,
    Submit,
    SubmissionComplete(Entreprise),
}

impl Component for FormEntreprise {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let user_id = match web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("user_id") {
            Ok(Some(id)) => id.parse::<i32>().unwrap_or_else(|_| {
                console::error_1(&"Failed to parse user_id".into());
                0
            }),
            Ok(None) | Err(_) => {
                console::error_1(&"Failed to get user_id from local storage".into());
                0
            }
        };

        Self {
            user_id,
            name: String::new(),
            date: String::new(),
            codeape: String::new(),
            status: String::new(),
            jrsttx: 0,
            jrsweek: 0,
            jrsferies: 0,
            jrscp: 0,
            jan: 0,
            fev: 0,
            mar: 0,
            avr: 0,
            mai: 0,
            juin: 0,
            jui: 0,
            aout: 0,
            sept: 0,
            oct: 0,
            nov: 0,
            dec: 0,
            submitted: false,
            current_step: 1,
            decompte: 0,
            total: 0,
            error_msg: None,
            date_err: None,
            oth_err: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateName(value) => {
                self.name = value;
                true
            }
            Msg::UpdateDate(value) => {
                self.date = value;
                true
            }
            Msg::UpdateCodeApe(value) => {
                self.codeape = value;
                true
            }
            Msg::UpdateStatus(value) => {
                self.status = value;
                true
            }
            Msg::UpdateJrsTTX(value) => {
                self.jrsttx = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateJrsWeek(value) => {
                self.jrsweek = value;
                ctx.link().send_message(Msg::CalculateDecompte);
                true
            }
            Msg::UpdateJrsFeries(value) => {
                self.jrsferies = value;
                ctx.link().send_message(Msg::CalculateDecompte);
                true
            }
            Msg::UpdateJrsCp(value) => {
                self.jrscp = value;
                ctx.link().send_message(Msg::CalculateDecompte);
                true
            }
            Msg::UpdateJan(value) => {
                self.jan = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateFev(value) => {
                self.fev = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateMar(value) => {
                self.mar = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateAvr(value) => {
                self.avr = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateMai(value) => {
                self.mai = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateJuin(value) => {
                self.juin = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateJui(value) => {
                self.jui = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateAout(value) => {
                self.aout = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateSept(value) => {
                self.sept = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateOct(value) => {
                self.oct = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateNov(value) => {
                self.nov = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::UpdateDec(value) => {
                self.dec = value;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::CalculateDecompte => {
                self.decompte = self.jrsweek as i32 +
                    self.jrsferies as i32 +
                    self.jrscp as i32;
                ctx.link().send_message(Msg::CalculateTotal);
                true
            }
            Msg::CalculateTotal => {
                self.total = self.jrsttx -
                    self.decompte -
                    self.jan as i32 -
                    self.fev as i32 -
                    self.mar as i32 -
                    self.avr as i32 -
                    self.mai as i32 -
                    self.juin as i32 -
                    self.jui as i32 -
                    self.aout as i32 -
                    self.sept as i32 -
                    self.oct as i32 -
                    self.nov as i32 -
                    self.dec as i32;
                true
            }
            Msg::Submit => {
                if !self.submitted {
                    if self.current_step == 2 && self.total == 0 && self.current_step != 3 || self.current_step == 2 && self.total < 0 && self.current_step != 3 {
                        self.error_msg = Some("Erreur : Aucun jours travaillés".to_string());
                        true
                    } else if self.current_step == 3 && self.total != 0 && self.current_step != 2 || self.current_step == 3 && self.total < 0 && self.current_step != 2 {
                        self.oth_err = Some("Mauvais positionnement, recommencez".to_string());
                        true
                    } else if self.current_step == 1 && !date_test(&self.date){
                        //regexpDate ok
                        self.date_err = Some("Format incorrect (JJ-MM-AAAA)".to_string());
                        true
                    }else {
                        if self.current_step < 3 {
                            self.current_step += 1;
                            //Clear errMsg when ok
                            self.date_err = None;
                            self.oth_err = None;
                            self.error_msg = None;
                            true
                        } else {
                            let entreprise = Entreprise {
                                id: 0,
                                user_id: self.user_id,
                                name: self.name.clone(),
                                date: self.date.clone(),
                                codeape: self.codeape.clone(),
                                status: self.status.clone(),
                                jrsttx: self.jrsttx,
                                jrsweek: self.jrsweek,
                                jrsferies: self.jrsferies,
                                jrscp: self.jrscp,
                                jan: self.jan,
                                fev: self.fev,
                                mar: self.mar,
                                avr: self.avr,
                                mai: self.mai,
                                juin: self.juin,
                                jui: self.jui,
                                aout: self.aout,
                                sept: self.sept,
                                oct: self.oct,
                                nov: self.nov,
                                dec: self.dec,
                            };
                            let entreprise_json = serde_json::to_string(&entreprise).unwrap();
                            log::info!("Submitting entreprise: {}", entreprise_json);
                            ctx.link().send_future(async {
                                let response = Request::post("http://localhost:8080/add_ent")
                                    .header("Content-Type", "application/json")
                                    .body(entreprise_json)
                                    .send()
                                    .await
                                    .unwrap();

                                if response.ok() {
                                    let new_ent: Entreprise = response.json().await.unwrap();
                                    log::info!("Entreprise created: {:?}", new_ent);
                                    Msg::SubmissionComplete(new_ent)
                                } else {
                                    log::error!("Failed to submit entreprise");
                                    Msg::Submit
                                }
                            });
                            self.submitted = true;
                            true
                        }
                    }
                } else {
                    false
                }
            }
            Msg::SubmissionComplete(new_ent) => {
                log::info!("Submission completed. Entreprise ID: {}", new_ent.id);
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::StepTwo);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::info!("Rendering view. Submitted: {}", self.submitted);

            html! {
                <div class="flex flex-col min-h-screen">
                    { header() }
                    <div class="bg-orange-50 flex flex-col flex-grow justify-center items-center">
                        <div class="flex flex-row w-full justify-center">
                            {
                                match self.current_step {
                                    1 => self.render_step1(ctx, false),
                                    2 => html! {
                                        <>
                                            <div class="mr-8">
                                                { self.render_step1(ctx, true) }
                                            </div>
                                            <div>
                                                { self.render_step2(ctx, false) }
                                            </div>
                                        </>
                                    },
                                    3 => html! {
                                        <>
                                            <div class="mr-8">
                                                { self.render_step1(ctx, true) }
                                            </div>
                                            <div class="mr-8">
                                                { self.render_step2(ctx, true) }
                                            </div>
                                            <div>
                                                { self.render_step3(ctx, false) }
                                            </div>
                                        </>
                                    },
                                    _ => html! {},
                                }
                            }
                        </div>
                    </div>
                    { footer() }
                </div>
            }
    }
}

impl FormEntreprise {
    fn render_step1(&self, ctx: &Context<Self>, disabled: bool) -> Html {
        let class = if self.current_step == 1 {
            "w-full max-w-md"
        } else {
            "w-full max-w-md opacity-50"
        };

        html! {
            <>
                <div class={class}>
                    <div class="mb-10 text-center text-gray-600 text-4xl font-semibold">
                        <h1>{ "Étape 1" }</h1>
                        <div class="mb-3 text-center text-gray-600 text-2xl font-semibold m-2">
                            <h1>{ "Afin de commencer votre simulation, veuillez renseigner votre future situation :" }</h1>
                        </div>
                    </div>
                    <form class="border-solid border-2 border-orange-400 bg-white shadow-[0_35px_60px_-15px_rgba(0,0,0,0.5)] rounded-lg px-8 pt-6 pb-8 mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                        e.prevent_default();
                        Msg::Submit
                    })}>
                        <div class="mb-4">
                            { self.view_box_title() }
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="name">{ "Nom de votre entreprise" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="name"
                                type="text"
                                placeholder="Entrez le nom de votre entreprise"
                                value={self.name.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateName(input.value())
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="date">{ "Date" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="date"
                                type="text"
                                placeholder="Date potentielle ouverture (JJ-MM-AAAA)"
                                value={self.date.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateDate(input.value())
                                })}
                                required=true
                            />
                        {
                            if let Some(ref message) = self.date_err {
                                html! {
                                    <div class="mb-2 text-center text-sm font-semibold text-red-500">
                                        { message }
                                    </div>
                                }
                            } else {
                                html! { <></> }
                            }
                        }
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="codeape">{ "Code APE" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="codeape"
                                type="text"
                                placeholder="Code APE"
                                value={self.codeape.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateCodeApe(input.value())
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="status">{ "Statut" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="status"
                                type="text"
                                placeholder="Statut de votre entreprise"
                                value={self.status.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateStatus(input.value())
                                })}
                                required=true
                            />
                        </div>
                        <div class="flex items-center justify-center">
                            <button
                                class="bg-emerald-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                                type="submit"
                                disabled={self.submitted || disabled}
                            >
                                { "SUIVANT" }
                            </button>
                        </div>
                    </form>
                </div>
            </>
        }
    }

    fn render_step2(&self, ctx: &Context<Self>, disabled: bool) -> Html {
        let class = if self.current_step == 2 {
            "w-full max-w-md"
        } else {
            "w-full max-w-md opacity-50"
        };
        html! {
            <>
                <div class={class}>
                    <div class="mb-10 text-center text-gray-600 text-4xl font-semibold">
                        <h1>{ "Étape 2" }</h1>
                        <div class="mb-3 text-center text-gray-600 text-2xl font-semibold m-2">
                            <h1>{ "Cette section calcule les jours travaillés et non travaillés :" }</h1>
                        </div>
                    </div>
                    <form class="border-solid border-2 border-orange-400 bg-white shadow-[0_35px_60px_-15px_rgba(0,0,0,0.5)] rounded-lg px-8 pt-6 pb-8 mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                        e.prevent_default();
                        Msg::Submit
                    })}>
                        <div class="mb-6">
                            { self.view_form_deux() }
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="jrsttx">{ "Jours travaillés dans l'année" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                id="jrsttx"
                                type="text"
                                value={self.jrsttx.to_string()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    match input.value().parse::<i32>() {
                                        Ok(value) => Msg::UpdateJrsTTX(value),
                                        Err(_) => Msg::UpdateJrsTTX(0),
                                    }
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="jrsweek">{ "Jours week-end" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                id="jrsweek"
                                type="text"
                                value={self.jrsweek.to_string()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    match input.value().parse::<i16>() {
                                        Ok(value) => Msg::UpdateJrsWeek(value),
                                        Err(_) => Msg::UpdateJrsWeek(0),
                                    }
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="jrsferies">{ "Jours fériés" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                id="jrsferies"
                                type="text"
                                value={self.jrsferies.to_string()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    match input.value().parse::<i8>() {
                                        Ok(value) => Msg::UpdateJrsFeries(value),
                                        Err(_) => Msg::UpdateJrsFeries(0),
                                    }
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-sm font-semibold mb-2" for="jrscp">{ "Jours congés payés" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                id="jrscp"
                                type="text"
                                value={self.jrscp.to_string()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    match input.value().parse::<i8>() {
                                        Ok(value) => Msg::UpdateJrsCp(value),
                                        Err(_) => Msg::UpdateJrsCp(0),
                                    }
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-2 text-center text-sm font-semibold text-gray-700">{ "Décompte jours non travaillés: " }<div class="mb-2 text-center text-sm font-semibold text-red-500">{ self.decompte }</div></div>
                        <div class="mb-2 text-center text-sm font-semibold text-gray-700">{ "Total jours travaillés: " }<div class="text-red-500">{ self.total }</div></div>
                        {
                            if let Some(ref message) = self.error_msg {
                                html! {
                                    <div class="mb-2 text-center text-sm font-semibold text-red-500">
                                        { message }
                                    </div>
                                }
                            } else {
                                html! { <></> }
                            }
                        }
                        <div class="flex items-center justify-center">
                            <button
                                class="bg-emerald-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                                type="submit"
                                disabled={self.submitted || disabled}
                            >
                                { "SUIVANT" }
                            </button>
                        </div>
                    </form>
                </div>
            </>
        }
    }

    fn render_step3(&self, ctx: &Context<Self>, disabled: bool) -> Html {
        html! {
            <>
                <div class="w-full max-w-md mx-auto">
                    <div class="text-center text-gray-600 text-4xl font-semibold">
                        <h1>{ "Étape 3" }</h1>
                    </div>
                    <div class="text-center text-gray-600 text-2xl font-semibold m-2">
                        <h1>{ "Cette section calcule les jours travaillés et non travaillés :" }</h1>
                    </div>
                    <form class="border-solid border-2 border-orange-400 bg-white shadow-lg rounded-lg px-8 pt-6 pb-8 mb-4"
                        onsubmit={ctx.link().callback(|e: SubmitEvent| {
                            e.prevent_default();
                            Msg::Submit
                        })}>
                        <table class="table-auto mb-4">
                            <thead>
                                <tr class="bg-orange-100">
                                    <th class="px-4 py-2">{ "Mois" }</th>
                                    <th class="px-4 py-2">{ "Nombre de jours" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td class="border px-4 py-2">{ "Janvier" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.jan.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateJan(value),
                                                    Err(_) => Msg::UpdateJan(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Février" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.fev.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateFev(value),
                                                    Err(_) => Msg::UpdateFev(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Mars" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.mar.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateMar(value),
                                                    Err(_) => Msg::UpdateMar(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Avril" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.avr.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateAvr(value),
                                                    Err(_) => Msg::UpdateAvr(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Mai" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.mai.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateMai(value),
                                                    Err(_) => Msg::UpdateMai(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Juin" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.juin.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateJuin(value),
                                                    Err(_) => Msg::UpdateJuin(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Juillet" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.jui.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateJui(value),
                                                    Err(_) => Msg::UpdateJui(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Août" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.aout.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateAout(value),
                                                    Err(_) => Msg::UpdateAout(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Septembre" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.sept.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateSept(value),
                                                    Err(_) => Msg::UpdateSept(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Octobre" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.oct.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateOct(value),
                                                    Err(_) => Msg::UpdateOct(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Novembre" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.nov.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateNov(value),
                                                    Err(_) => Msg::UpdateNov(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <td class="border px-4 py-2">{ "Décembre" }</td>
                                    <td class="border px-4 py-2">
                                        <input
                                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                            type="text"
                                            value={self.dec.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                match input.value().parse::<i8>() {
                                                    Ok(value) => Msg::UpdateDec(value),
                                                    Err(_) => Msg::UpdateDec(0),
                                                }
                                            })}
                                        />
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                        <div class="mb-2 text-center text-sm font-semibold text-gray-700">
                            { "Il vous reste " }
                            <div class="text-red-500">
                                { self.total }
                            </div>
                            <div class="mb-2 text-center text-sm font-semibold text-gray-700">
                                {"jours à positionner"}
                            </div>
                        </div>
                        {
                            if let Some(ref message) = self.oth_err {
                                html! {
                                    <div class="mb-2 text-center text-sm font-semibold text-red-500">
                                        { message }
                                    </div>
                                }
                            } else {
                                html! { <></> }
                            }
                        }
                        <div class="flex items-center justify-center">
                            <button class="bg-orange-400 hover:bg-orange-500 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                                    type="submit"
                                    disabled={self.submitted || disabled}
                            >
                                { "Soumettre" }
                            </button>
                        </div>
                    </form>
                </div>
            </>
        }
    }

    fn view_box_title(&self) -> Html {
        html! {
            <div class="mb-4 text-xl font-bold text-center text-gray-700">
                { "Formulaire d'entreprise" }
            </div>
        }
    }

    fn view_form_deux(&self) -> Html {
        html! {
            <div class="mb-4 text-xl font-bold text-center text-gray-700">
                { "Décompte des Jours Travaillés et Non Travaillés" }
            </div>
        }
    }

}
