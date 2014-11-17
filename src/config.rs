use serialize::{ json, Decodable };
use std::io::{ File, Open, Read };

#[deriving(Decodable)]
pub struct Config {
    pub base: String,
}

impl Config {
    pub fn from_file(loc: &str) -> Config {
        let p = Path::new(loc);
        let mut file = match File::open_mode(&p, Open, Read) {
            Ok(f) => f,
            Err(e) => panic!("File error: {}", e),
        };

        let decoded = match file.read_to_string() {
            Ok(f) => f,
            Err(e) => panic!("File error: {}", e),
        };

        let json_object = match json::from_str(decoded[]) {
            Ok(v) => v,
            Err(e) => panic!("Json error: {}", e),
        };
        let mut decoder = json::Decoder::new(json_object);

        match Decodable::decode(&mut decoder) {
            Ok(v) => v,
            Err(e) => panic!("Decoding error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn load_test() {
        let _conf = Config::from_file("config_ex.json");
    }

    #[test]
    #[should_fail]
    fn load_test_fail() {
        let _conf = Config::from_file("missing.json");
    }
}

