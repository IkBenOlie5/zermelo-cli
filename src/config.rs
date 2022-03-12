pub struct Config {
    pub code: Option<String>,
    pub access_token: Option<String>,
    pub school: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        match args.next() {
            Some(flag) => {
                if flag == "-c" || flag == "--code" {
                    let code = match args.next() {
                        Some(code) => code,
                        None => return Err("No code provided"),
                    };
                    let school = match args.next() {
                        Some(school) => school,
                        None => return Err("No school provided"),
                    };
                    Ok(Config {
                        code: Some(code),
                        access_token: None,
                        school,
                    })
                } else if flag == "-a" || flag == "--access-token" {
                    let access_token = match args.next() {
                        Some(access_token) => access_token,
                        None => return Err("No access token provided"),
                    };
                    let school = match args.next() {
                        Some(school) => school,
                        None => return Err("No school provided"),
                    };

                    Ok(Config {
                        code: None,
                        access_token: access_token.into(),
                        school,
                    })
                } else if flag == "-h" || flag == "--help" {
                    Err("help")
                } else if flag == "-V" || flag == "--version" {
                    Err("version")
                } else {
                    Err("Invalid flag")
                }
            }
            None => Err("help"),
        }
    }
}
