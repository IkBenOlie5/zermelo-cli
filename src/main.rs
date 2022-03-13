#![feature(drain_filter)]

use colored::Colorize;
use std::process::exit;

use std::env;

mod config;
mod zermelo;
use config::Config;
use zermelo::{Appointment, Zermelo};

fn main() {
    println!("Good morning! 👋");

    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|e| {
        if e == "help" {
            print_help();
            exit(0);
        } else if e == "version" {
            println!("Version: 0.1.0");
            exit(0);
        } else {
            eprintln!("❌ Error parsing arguments: {}", e);
            exit(1);
        }
    });

    let zermelo;
    if let Some(access_token) = config.access_token {
        zermelo = Zermelo::new(&access_token, &config.school);
    } else {
        zermelo = Zermelo::from_code(&config.code.unwrap(), &config.school).unwrap_or_else(|e| {
            eprintln!("❌ Error getting access token: `{}`", e);
            exit(1)
        });

        println!("Access token is: `{}`. You can use this instead of the code with the -a|--access-token flag", zermelo.access_token);
    }

    let appointments = zermelo.get_appointments().unwrap_or_else(|e| {
        eprintln!("Error getting appointments: `{}`", e);
        exit(1)
    });
    print_appointments(&appointments);

    println!("Have a nice day! 🙋");
}

fn print_help() {
    println!("zermelo-cli\n");
    println!("USAGE:");
    println!("   zermelo-cli <FLAG> <CODE OR ACCESS TOKEN> <SCHOOL>\n");
    println!("FLAGS:");
    println!("    -h, --help                           Prints this message");
    println!("    -V, --version                        Prints version information");
    println!(
        "    -c, --code <CODE>                    Uses the provided code to fetch the access token"
    );
    println!("    -a, --access-token <ACCESS_TOKEN>    Uses the provided access token\n");
    println!("ARGS:");
    println!("    <SCHOOL>    The school to get appointments from\n");
    println!("EXAMPLES:");
    println!("    zermelo-cli -c 123456789101 cgu");
    println!("    zermelo-cli -a fajsidu29dj2jdmv0sjsj2jd8d usg");
}

fn print_appointments(appointments: &[Appointment]) {
    println!("APPOINTMENTS:");
    println!(
        "    {} - {} - {} - {} - ({}-{})",
        "SUBJECT(S)".bright_blue(),
        "TEACHER(S)".bright_red(),
        "GROUP(S)".bright_green(),
        "LOCATION(S)".bright_yellow(),
        "START".bright_cyan(),
        "END".bright_magenta()
    );
    for appointment in appointments {
        println!("{}", appointment);
    }
}
