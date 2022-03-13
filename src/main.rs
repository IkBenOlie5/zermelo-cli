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

    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(|e| {
        if e == "help" {
            print_help();
            exit(0);
        } else if e == "version" {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            exit(0);
        } else {
            eprintln!("❌ Error parsing arguments: {}", e);
            exit(1);
        }
    });

    let zermelo;

    if config.use_code {
        zermelo = Zermelo::from_code(&config.auth, &config.school).unwrap_or_else(|e| {
            eprintln!("❌ Error getting access token: `{}`", e);
            exit(1)
        });
        println!(
            "Access token is: `{}`, you can use it instead of the code next time",
            zermelo.access_token
        );
    } else {
        zermelo = Zermelo::new(&config.auth, &config.school);
    }

    let appointments = zermelo.get_appointments().unwrap_or_else(|e| {
        eprintln!("❌ Error getting appointments: `{}`", e);
        exit(1)
    });
    print_appointments(&appointments);

    println!("Have a nice day! 🙋");
}

fn print_help() {
    println!("zermelo-cli\n");
    println!("USAGE:");
    println!("   zermelo-cli [FLAG] [ACCESS TOKEN] <SCHOOL>\n");
    println!("FLAGS:");
    println!("    -h, --help                           Prints this message");
    println!("    -V, --version                        Prints version information");
    println!("    -c, --code <CODE>                    Uses the code to fetch the access token");
    println!("ARGS:");
    println!("    [ACCESS TOKEN]    The access token to get the appointments with (if -c or --code in specified, this is optional)\n");
    println!("    <SCHOOL>          The school to get appointments from\n");
    println!("EXAMPLES:");
    println!("    zermelo-cli -c 123456789101 cgu");
    println!("    zermelo-cli fajsidu29dj2jdmv0sjsj2jd8d usg");
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
