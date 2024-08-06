use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{HtmlInputElement};
use web_sys::HtmlSelectElement;
use reqwasm::http::Request;
use web_sys::console;
use crate::{AppRoute, header, footer};
use crate::modals::{Entreprise, EntrepriseMsg, FormEntreprise};
use crate::utils::{auto_distribute, date_test};

impl Component for FormEntreprise {
    type Message = EntrepriseMsg;
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
            status: String::from("NULL"),
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
            err_status: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EntrepriseMsg::UpdateName(value) => {
                self.name = value;
                true
            }
            EntrepriseMsg::UpdateDate(value) => {
                self.date = value;
                true
            }
            EntrepriseMsg::UpdateCodeApe(value) => {
                self.codeape = value;
                true
            }
            EntrepriseMsg::UpdateStatus(value) => {
                self.status = value;
                true
            }
            EntrepriseMsg::UpdateJrsTTX(value) => {
                self.jrsttx = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateJrsWeek(value) => {
                self.jrsweek = value;
                ctx.link().send_message(EntrepriseMsg::CalculateDecompte);
                true
            }
            EntrepriseMsg::UpdateJrsFeries(value) => {
                self.jrsferies = value;
                ctx.link().send_message(EntrepriseMsg::CalculateDecompte);
                true
            }
            EntrepriseMsg::UpdateJrsCp(value) => {
                self.jrscp = value;
                ctx.link().send_message(EntrepriseMsg::CalculateDecompte);
                true
            }
            EntrepriseMsg::UpdateJan(value) => {
                self.jan = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateFev(value) => {
                self.fev = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateMar(value) => {
                self.mar = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateAvr(value) => {
                self.avr = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateMai(value) => {
                self.mai = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateJuin(value) => {
                self.juin = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateJui(value) => {
                self.jui = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateAout(value) => {
                self.aout = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateSept(value) => {
                self.sept = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateOct(value) => {
                self.oct = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateNov(value) => {
                self.nov = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::UpdateDec(value) => {
                self.dec = value;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::CalculateDecompte => {
                self.decompte = self.jrsweek as i32 +
                    self.jrsferies as i32 +
                    self.jrscp as i32;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::CalculateTotal => {
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
            EntrepriseMsg::AutoDistribution => {
                let (jan, fev, mar, avr, mai, juin, jui, aout, sept, oct, nov, dec) = auto_distribute(self.total);
                self.jan = jan;
                self.fev = fev;
                self.mar = mar;
                self.avr = avr;
                self.mai = mai;
                self.juin = juin;
                self.jui = jui;
                self.aout = aout;
                self.sept = sept;
                self.oct = oct;
                self.nov = nov;
                self.dec = dec;
                ctx.link().send_message(EntrepriseMsg::CalculateTotal);
                true
            }
            EntrepriseMsg::Submit => {
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
                    } else if self.status == "NULL"{
                        self.err_status = Some("Mettez à jour votre statut d'entreprise".to_string());
                        true
                    }else {
                        if self.current_step < 3 {
                            self.current_step += 1;
                            //Clear errMsg when ok
                            self.date_err = None;
                            self.oth_err = None;
                            self.error_msg = None;
                            self.err_status = None;
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
                                    EntrepriseMsg::SubmissionComplete(new_ent)
                                } else {
                                    log::error!("Failed to submit entreprise");
                                    EntrepriseMsg::Submit
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
            EntrepriseMsg::SubmissionComplete(new_ent) => {
                log::info!("Submission completed. Entreprise ID: {}", new_ent.id);
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::StepTwo);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

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
            "w-full max-w-xl"
        } else {
            "w-full max-w-md opacity-50"
        };

        html! {
            <div class="flex justify-center items-center h-screen">
                <div class={class}>
                    <div class="mb-10 text-center text-gray-600 text-4xl font-semibold">
                        <h1>{ "Étape 1" }</h1>
                        <div class="mb-3 text-center text-gray-600 text-2xl font-semibold m-2">
                            <h1>{ "Veuillez renseigner votre future situation :" }</h1>
                        </div>
                    </div>
                    <form class="border-solid border-2 border-orange-400 bg-white shadow-[0_35px_60px_-15px_rgba(0,0,0,0.5)] rounded-lg px-8 pt-6 pb-8 mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                        e.prevent_default();
                        EntrepriseMsg::Submit
                    })}>
                        <div class="mb-4">
                            { self.view_box_title() }
                            <label class="block text-orange-500 text-m text-center font-semibold mb-2" for="name">{ "Nom de votre entreprise" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="name"
                                type="text"
                                placeholder="Entrez le nom de votre entreprise"
                                value={self.name.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    EntrepriseMsg::UpdateName(input.value())
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-m text-center font-semibold mb-2" for="date">{ "Date" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="date"
                                type="text"
                                placeholder="Date potentielle ouverture (JJ-MM-AAAA)"
                                value={self.date.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    EntrepriseMsg::UpdateDate(input.value())
                                })}
                                required=true
                            />
                        {
                            if let Some(ref message) = self.date_err {
                                html! {
                                    <div class="mb-2 text-m text-center font-semibold text-red-500">
                                        { message }
                                    </div>
                                }
                            } else {
                                html! { <></> }
                            }
                        }
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-m text-center font-semibold mb-2" for="codeape">{ "Code APE *" }</label>
                            <a href="https://entreprendre.service-public.fr/vosdroits/F33050" class="text-red-500 text-sm text-center mb-2 font-semibold">{"*Plus d'informations"}</a>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline placeholder-gray-700"
                                id="codeape"
                                type="text"
                                placeholder="Code APE"
                                value={self.codeape.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    EntrepriseMsg::UpdateCodeApe(input.value())
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-6">
                                <label class="block text-orange-500 text-m text-center font-semibold mb-2" for="status">{ "Statut" }</label>
                                <div class="relative inline-block w-full">
                                    <select
                                        class="block appearance-none w-full bg-white border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline"
                                        id="status"
                                        value={self.status.clone()}
                                        onchange={ctx.link().callback(|e: Event| {
                                            let input: HtmlSelectElement = e.target_unchecked_into();
                                            EntrepriseMsg::UpdateStatus(input.value())
                                        })}
                                    >

                                        <option value="MC">{ "Micro Entreprise (MC)" }</option>
                                        <option value="EI">{ "Entreprise Individuelle (EI)" }</option>
                                        <option value="EIRL">{ "Entreprise Individuelle Responsabilité Limitée (EIRL)" }</option>
                                        <option value="SARL">{ "Société Responsabilité Limitée (SARL)" }</option>
                                        <option value="SASU">{ "Société Actions Simplifiée Unipersonnelle (SASU)" }</option>
                                        <option value="SAS">{ "Société Actions Simplifiée (SAS)" }</option>
                                        <option value="NULL">{ "Choisissez votre statut d'entreprise" }</option>
                                    </select>
                                    <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                                        <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M10 12l-5-5h10l-5 5z"/></svg>
                                    </div>
                                </div>
                                <div>
                                {
                                    if let Some(ref message) = self.err_status{
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
            </div>
        }
    }

    fn render_step2(&self, ctx: &Context<Self>, disabled: bool) -> Html {
        let class = if self.current_step == 2 {
            "w-full max-w-xl"
        } else {
            "w-full max-w-md opacity-50"
        };
        html! {
            <div class="flex justify-center items-center h-screen">
                <div class={class}>
                    <div class="mb-10 text-center text-gray-600 text-4xl font-semibold">
                        <h1>{ "Étape 2" }</h1>
                        <div class="mb-3 text-center text-gray-600 text-2xl font-semibold m-2">
                            <h1>{ "Cette section calcule les jours travaillés et non travaillés :" }</h1>
                        </div>
                    </div>
                    <form class="border-solid border-2 border-orange-400 bg-white shadow-[0_35px_60px_-15px_rgba(0,0,0,0.5)] rounded-lg px-8 pt-6 pb-8 mb-4" onsubmit={ctx.link().callback(|e: SubmitEvent| {
                        e.prevent_default();
                        EntrepriseMsg::Submit
                    })}>
                        <div class="mb-6">
                            { self.view_form_deux() }
                            <label class="block text-orange-500 text-m text-center font-semibold mb-2" for="jrsttx">{ "Jours travaillés dans l'année" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                id="jrsttx"
                                type="text"
                                value={self.jrsttx.to_string()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    match input.value().parse::<i32>() {
                                        Ok(value) => EntrepriseMsg::UpdateJrsTTX(value),
                                        Err(_) => EntrepriseMsg::UpdateJrsTTX(0),
                                    }
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-m text-center font-semibold mb-2" for="jrsweek">{ "Jours week-end" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                id="jrsweek"
                                type="text"
                                value={self.jrsweek.to_string()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    match input.value().parse::<i16>() {
                                        Ok(value) => EntrepriseMsg::UpdateJrsWeek(value),
                                        Err(_) => EntrepriseMsg::UpdateJrsWeek(0),
                                    }
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-m text-center font-semibold mb-2" for="jrsferies">{ "Jours fériés" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                id="jrsferies"
                                type="text"
                                value={self.jrsferies.to_string()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    match input.value().parse::<i8>() {
                                        Ok(value) => EntrepriseMsg::UpdateJrsFeries(value),
                                        Err(_) => EntrepriseMsg::UpdateJrsFeries(0),
                                    }
                                })}
                                required=true
                            />
                        </div>
                        <div class="mb-6">
                            <label class="block text-orange-500 text-m text-center font-semibold mb-2" for="jrscp">{ "Jours congés payés" }</label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                id="jrscp"
                                type="text"
                                value={self.jrscp.to_string()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    match input.value().parse::<i8>() {
                                        Ok(value) => EntrepriseMsg::UpdateJrsCp(value),
                                        Err(_) => EntrepriseMsg::UpdateJrsCp(0),
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
                                    <div class="mb-2 text-center text-m text-center font-semibold text-red-500">
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
            </div>
        }
    }

    fn render_step3(&self, ctx: &Context<Self>, disabled: bool) -> Html {
        html! {
        <div class="flex justify-center items-center h-screen">
            <div class="w-full max-w-lg">
                <div class="text-center text-gray-600 text-4xl font-semibold">
                    <h1>{ "Étape 3" }</h1>
                </div>
                <div class="mb-10 text-center text-gray-600 text-2xl font-semibold m-2">
                    <h1>{ "Cette section calcule les jours travaillés et non travaillés :" }</h1>
                </div>
                <form class="border-solid border-2 border-orange-400 bg-white shadow-lg rounded-lg px-4 pt-4 pb-4 mb-4"
                    onsubmit={ctx.link().callback(|e: SubmitEvent| {
                        e.prevent_default();
                        EntrepriseMsg::Submit
                    })}>
                    <div class="mb-2 mt-6">
                        {self.view_form_trois()}
                    </div>
                    <div class="flex items-center justify-center">
                        <button class="text-gray-700 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                                type="button"
                                onclick={ctx.link().callback(|_| EntrepriseMsg::AutoDistribution)}
                        >
                            { "Répartition Automatique" }
                        </button>
                    </div>
                    <div class="flex justify-center mb-6">
                    <table class="table-auto text-xl">
                        <thead>
                            <tr class="bg-orange-100">
                                <th class="px-2 py-1">{ "Mois" }</th>
                                <th class="px-2 py-1">{ "Nombre de jours" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Janvier" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.jan.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateJan(value),
                                                Err(_) => EntrepriseMsg::UpdateJan(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Février" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.fev.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateFev(value),
                                                Err(_) => EntrepriseMsg::UpdateFev(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Mars" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.mar.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateMar(value),
                                                Err(_) => EntrepriseMsg::UpdateMar(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Avril" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.avr.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateAvr(value),
                                                Err(_) => EntrepriseMsg::UpdateAvr(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Mai" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.mai.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateMai(value),
                                                Err(_) => EntrepriseMsg::UpdateMai(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Juin" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.juin.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateJuin(value),
                                                Err(_) => EntrepriseMsg::UpdateJuin(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Juillet" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.jui.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateJui(value),
                                                Err(_) => EntrepriseMsg::UpdateJui(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Août" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.aout.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateAout(value),
                                                Err(_) => EntrepriseMsg::UpdateAout(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Septembre" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.sept.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateSept(value),
                                                Err(_) => EntrepriseMsg::UpdateSept(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Octobre" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.oct.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateOct(value),
                                                Err(_) => EntrepriseMsg::UpdateOct(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Novembre" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.nov.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateNov(value),
                                                Err(_) => EntrepriseMsg::UpdateNov(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td class="text-orange-500 text-lg font-medium border px-2 py-1">{ "Décembre" }</td>
                                <td class="border px-2 py-1">
                                    <input
                                        class="shadow appearance-none border rounded w-full py-1 px-2 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                        type="text"
                                        value={self.dec.to_string()}
                                        oninput={ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            match input.value().parse::<i8>() {
                                                Ok(value) => EntrepriseMsg::UpdateDec(value),
                                                Err(_) => EntrepriseMsg::UpdateDec(0),
                                            }
                                        })}
                                    />
                                </td>
                            </tr>
                        </tbody>
                    </table>
                    </div>
                    <div class="mb-4 text-center text-sm font-semibold text-gray-700">
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
                    <div class="mb-10 flex items-center justify-center">
                        <button class="bg-orange-400 hover:bg-orange-500 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                                type="submit"
                                disabled={self.submitted || disabled}
                        >
                            { "Soumettre" }
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
    }

    fn view_box_title(&self) -> Html {
        html! {
            <div class="text-center text-2xl font-medium mb-4">
                { "Formulaire d'entreprise" }
            </div>
        }
    }

    fn view_form_deux(&self) -> Html {
        html! {
            <div class="text-center text-2xl font-medium mb-4">
                { "Décompte des Jours Travaillés et Non Travaillés" }
            </div>
        }
    }

    fn view_form_trois(&self) -> Html {
        html! {
            <div class="text-center text-2xl font-medium mb-4">
                { "Répartition mensuelle des jours de travail" }
            </div>
        }
    }
}
