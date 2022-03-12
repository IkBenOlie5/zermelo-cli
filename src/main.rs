#![feature(drain_filter)]

use std::process::exit;
use colored::Colorize;
use std::fmt;
use chrono::{Local, DateTime, NaiveDateTime};

use std::env;

mod config;
mod zermelo;
use config::Config;
use zermelo::{Zermelo, Appointment};

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
        write!(
            f,
            "{}",
            s
        )
    }
}

fn main() {
    println!("Good morning! 👋");
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|e| {

        if e == "help" {
            println!("zermelo-cli\n");
            println!("USAGE:");
            println!("   zermelo-cli <FLAG> <CODE OR ACCESS TOKEN> <SCHOOL>\n");
            println!("FLAGS:");
            println!("    -h, --help                           Prints this message");
            println!("    -V, --version                        Prints version information");
            println!("    -c, --code <CODE>                    Uses the provided code to fetch the access token");
            println!("    -a, --access-token <ACCESS_TOKEN>    Uses the provided access token\n");
            println!("ARGS:");
            println!("    <SCHOOL>    The school to get appointments from\n");
            println!("EXAMPLES:");
            println!("    zermelo-cli -c 123456789101 cgu");
            println!("    zermelo-cli -a fajsidu29dj2jdmv0sjsj2jd8d usg");
            exit(0);
        } else if e == "version" {
            println!("Version: 0.1.0");
            exit(0);
        }
        else {
            eprintln!("❌ Error parsing arguments: {}", e);
            exit(1);
        }
        
    });

    let zermelo;
    if let Some(code) = config.code {
        zermelo = Zermelo::from_code(&code, &config.school).unwrap_or_else(|e| {
            eprintln!("❌ Error getting access token: `{}`", e);
            exit(1)
        });

        println!("Access token is: `{}`. You can use this instead of the code with the -a|--access-token flag", zermelo.access_token);
    } else {
        zermelo = Zermelo::from_access_token(&config.access_token.unwrap(), &config.school);
    }

    let mut appointments = zermelo.get_appointments().unwrap_or_else(|e| {eprintln!("Error getting appointments: `{}`", e); exit(1)});
    appointments.sort_by(|a, b| a.start.cmp(&b.start));

    println!("APPOINTMENTS:");
    println!("    {} - {} - {} - {} - ({}-{})", "SUBJECT(S)".bright_blue(), 
    "TEACHER(S)".bright_red(), "GROUP(S)".bright_green(), "LOCATION(S)".bright_yellow(), "START".bright_cyan(), "END".bright_magenta());
    for appointment in appointments {
        println!("{}", appointment);
    }
    println!("Have a nice day! 🙋");
}
