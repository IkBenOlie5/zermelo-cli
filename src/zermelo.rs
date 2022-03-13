use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime};
use colored::Colorize;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{error::Error, fmt};

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
    start: i64,
    end: i64,
    subjects: Vec<String>,
    teachers: Vec<String>,
    groups: Vec<String>,
    locations: Vec<String>,
    cancelled: bool,
}

impl fmt::Display for Appointment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let offset = *Local::now().offset();
        let start =
            DateTime::<Local>::from_utc(NaiveDateTime::from_timestamp(self.start, 0), offset);
        let end = DateTime::<Local>::from_utc(NaiveDateTime::from_timestamp(self.end, 0), offset);

        let mut s = format!(
            "    {} - {} - {} - {} - ({}-{})",
            self.subjects.join(", ").bright_blue(),
            self.teachers.join(", ").bright_red(),
            self.groups.join(", ").bright_green(),
            self.locations.join(", ").bright_yellow(),
            start.format("%H:%M").to_string().bright_cyan(),
            end.format("%H:%M").to_string().bright_magenta(),
        );

        if self.cancelled {
            s = s.on_red().to_string();
        }
        write!(f, "{}", s)
    }
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
    pub fn new(access_token: &str, school: &str) -> Self {
        Zermelo {
            url: format!("https://{}.zportal.nl/api/v3", school),
            client: Client::new(),
            access_token: access_token.to_string(),
        }
    }

    pub fn from_code(code: &str, school: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        let url = format!("https://{}.zportal.nl/api/v3", school);

        let res = client
            .post(format!("{}/oauth/token", url))
            .form(&[("grant_type", "authorization_code"), ("code", code)])
            .send()?
            .error_for_status()?;

        Ok(Zermelo {
            url,
            client,
            access_token: res.json::<AuthResponse>()?.access_token,
        })
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

        let mut appointments = res.json::<AppointmentsResponse>()?.response.data;
        appointments.sort_by(|a, b| a.start.cmp(&b.start));
        Ok(appointments)
    }
}
