use request::request;
use parse;
use info::{ Type, TypeInfo, Course, Group };
use config::Config;

/// Search for a match.
///
/// First search for a match in groups and as a fallback
/// search for a matching course.
///
/// Never mix courses with groups in result.
pub fn search(string: &str, conf: &Config) -> (Vec<TypeInfo>, Type) {
    let groups = group_search(string, conf);
    if !groups.is_empty() {
        (groups, Group)
    } else {
        (course_search(string, conf), Course)
    }
}

pub fn course_search(string: &str, conf: &Config) -> Vec<TypeInfo> {
    type_search(string, Course, conf)
}

pub fn group_search(string: &str, conf: &Config) -> Vec<TypeInfo> {
    type_search(string, Group, conf)
}

fn type_search(string: &str, t: Type, conf: &Config) -> Vec<TypeInfo> {
    // TODO use json when searching for things?
    let url = format!("{}/objects.html?max=100&partajax=t&sid=3&search_text={}&types={}",
                      conf.base, string, t.num_id());
    println!("Url: {}", url);
    let txt = request(url[]);

    parse::search_res(txt[], t)
}

#[cfg(test)]
mod tests {
    use super::search;

    use time;
    use std::time::Duration;
    use schedule::schedule;
    use info::{ Course, Group, TypeInfo, DataId };
    use config::Config;

    #[test]
    fn test_search() {
        let from = time::now();
        let to = time::at(from.to_timespec() + Duration::weeks(1));
        let conf = Config::from_file("config_ex.json".to_string());

        let s = "TATA";

        let (types, typ) = search(s, &conf);
        let ts = match typ {
            Course => "courses",
            Group => "groups",
        };
        println!("Found {} {}", types.len(), ts);
        for t in types.iter() {
            println!("{}", t);
        }

        let mut types = Vec::new();
        //types.push(TypeInfo::new("TATA31", "", DataId::new("363733.219")));
        types.push(TypeInfo::new("TATA49", "", DataId::new("363741.219")));
        types.push(TypeInfo::new("FYN1", "", DataId::new("153398.205")));

        let entries = schedule(types, from, to, &conf);

        println!("Found {} entries", entries.len());
        for entry in entries.iter() {
            println!("{}", entry);
        }

        panic!("===o,o===");
    }
}

