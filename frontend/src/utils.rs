use serde::{Serialize, Deserialize};

//Struct && HomeMsg for HomePage
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub lastname: String,
    pub firstname: String,
}
pub enum HomeMsg {
    UpdateLastName(String),
    UpdateFirstName(String),
    Submit,
    SubmissionComplete(User),
}

//Struct && EntrepriseMsg for nextPage
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entreprise {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub date: String,
    pub codeape: String,
    pub status: String,
    pub jrsttx: i32,
    pub jrsweek: i16,
    pub jrsferies: i8,
    pub jrscp: i8,
    pub jan: i8,
    pub fev: i8,
    pub mar: i8,
    pub avr: i8,
    pub mai: i8,
    pub juin: i8,
    pub jui: i8,
    pub aout: i8,
    pub sept: i8,
    pub oct: i8,
    pub nov: i8,
    pub dec: i8,
}

pub enum EntrepriseMsg {
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