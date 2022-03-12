use chrono::{DateTime, Datelike, Local, NaiveDate};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct AppointmentsResponse {
    response: AppointmentsData,
}

#[derive(Deserialize)]
struct AppointmentsData {
    data: Vec<Appointment>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Appointment {
    pub start: i64,
    pub end: i64,
    pub subjects: Vec<String>,
    pub teachers: Vec<String>,
    pub groups: Vec<String>,
    pub locations: Vec<String>,
    pub cancelled: bool,
}



#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
}

pub struct Zermelo {
    url: String,
    client: Client,
    pub access_token: String,
}

impl Zermelo {
    pub fn from_code(code: &str, school: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        let url = format!("https://{}.zportal.nl/api/v3", school);

        let res = client
            .post(format!("{}/oauth/token", url))
            .form(&[("grant_type", "authorization_code"), ("code", code)])
            .send()
            .unwrap()
            .error_for_status()
            .unwrap();

        Ok(Zermelo {
            url,
            client,
            access_token: res.json::<AuthResponse>()?.access_token,
        })
    }

    pub fn from_access_token(access_token: &str, school: &str) -> Self {
        Zermelo {
            url: format!("https://{}.zportal.nl/api/v3", school),
            client: Client::new(),
            access_token: access_token.to_string(),
        }
    }

    pub fn get_appointments(&self) -> Result<Vec<Appointment>, Box<dyn Error>> {
        let dt = Local::now();

        let start = DateTime::<Local>::from_utc(
            NaiveDate::from_ymd(dt.year(), dt.month(), dt.day()).and_hms(0, 0, 0),
            *dt.offset(),
        )
        .timestamp();
        let end = start + 24 * 60 * 60;

        let res = self
            .client
            .get(format!(
                "{}/appointments?user=~me&start={}&end={}&access_token={}&valid=true",
                self.url, start, end, self.access_token
            ))
            .send()?
            .error_for_status()?;

        Ok(res.json::<AppointmentsResponse>()?.response.data)
    }
}
