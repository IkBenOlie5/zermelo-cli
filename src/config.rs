use std::cmp::Ordering;

pub struct Config {
    pub school: String,
    pub use_code: bool,
    pub auth: String, // this can be the code or the access token
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() == 1 {
            // no arguments
            return Err("help");
        }

        match args[1].as_str() {
            "-h" | "--help" => Err("help"),
            "-V" | "--version" => Err("version"),
            "-c" | "--code" => match args.len().cmp(&4) {
                Ordering::Less => Err("not enough arguments"),
                Ordering::Greater => Err("too many arguments"),
                Ordering::Equal => Ok(Config {
                    school: args[3].clone(),
                    use_code: true,
                    auth: args[2].clone(),
                }),
            },
            _ => match args.len().cmp(&3) {
                Ordering::Less => Err("not enough arguments"),
                Ordering::Greater => Err("too many arguments"),
                Ordering::Equal => Ok(Config {
                    school: args[2].clone(),
                    use_code: false,
                    auth: args[1].clone(),
                }),
            },
        }
    }
}
