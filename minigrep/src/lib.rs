#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 3 {
            return Result::Err(String::from("args not enough"));
        }
        // let query = &args[1];
        // let file = &args[2];
        // let query = args[1];
        // let file = args[2];
        let query = args[1].clone();
        let file = args[2].clone();
        let case_sensitive = std::env::var("CASE_SENSITIVE").is_ok();
        Result::Ok(Config {
            query,
            file,
            case_sensitive,
        })
    }
}

pub fn read(conf: &Config) -> Result<String, std::io::Error> {
    std::fs::read_to_string(&conf.file)
}

pub fn search<'a>(conf: &Config, content: &'a str) -> Vec<&'a str> {
    if conf.case_sensitive {
        let mut result = vec![];
        for v in content.lines() {
            if v.contains(&conf.query) {
                result.push(v);
            }
        }
        result
    } else {
        let mut result = vec![];
        let query = conf.query.to_lowercase();
        for v in content.lines() {
            if v.to_lowercase().contains(&query) {
                result.push(v);
            }
        }
        result
    }
}
