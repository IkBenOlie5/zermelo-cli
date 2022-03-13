pub struct Config {
    pub school: String,
    pub use_code: bool,
    pub auth: String, // this can be the code or the access token
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        match args[1].as_str() {
            "-h" | "--help" => Err("help"),
            "-V" | "--version" => Err("version"),
            "-c" | "--code" => {
                if args.len() < 4 {
                    Err("not enough arguments")
                } else if args.len() > 4{
                    Err("too many arguments")
                }
                
                else {
                    Ok(Config {
                        school: args[3].clone(),
                        use_code: true,
                        auth: args[2].clone(),
                    })
                }
            }
            _ => {
                if args.len() < 3 {
                    Err("not enough arguments")
                } else if args.len() > 3 {
                    Err("too many arguments")
                }
                
                else {
                    Ok(Config {
                        school: args[2].clone(),
                        use_code: false,
                        auth: args[1].clone(),
                    })
                }
            }
        }
    }
}
