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
    //println!("Searching for {}", course);
    // TODO use json when searching for things?
    // https://se.timeedit.net/web/liu/db1/schema/objects.json?max=100&sid=3&search_text=TATA&types=219&fe=132.0&fe=115.20132,20141,20142
    // instead
    // fe is history or something?
    let url = format!("{}/objects.html?max=100&partajax=t&im=f&sid=3&l=sv&search_text={}&types={}", conf.base, string, t.num_id());
    let txt = request(url[]);

    parse::search_res(txt[], t)
}

#[cfg(test)]
mod tests {
    use super::search;

    use time;
    use std::time::Duration;
    use schedule::schedule;
    use info::{ Course, Group };
    use config::Config;

    #[test]
    fn test_search() {
        let from = time::now();
        let to = time::at(from.to_timespec() + Duration::weeks(1));

        let s = "TATA49";

        let conf = Config::from_file("config.json".to_string());

        let (types, typ) = search(s, &conf);
        let ts = match typ {
            Course => "courses",
            Group => "groups",
        };
        println!("Found {} {}", types.len(), ts);
        for t in types.iter() {
            println!("{}", t);
        }

        //let mut types = courses;
        //types.push_all(groups[]);

        //let mut types = Vec::new();
        //types.push(Type {
            //id: "TATA31".to_string(),
            //name: "".to_string(),
            //data_id: "363733.219".to_string()
        //});
        //types.push(Type {
            //id: "TATA49".to_string(),
            //name: "".to_string(),
            //data_id: "363741.219".to_string()
        //});
        // !! Cannot mix courses and groups, it will not return anything then!!
        //types.push(Type {
            //id: "FYN1".to_string(),
            //name: "".to_string(),
            //data_id: "153398.205".to_string()
        //});

        let entries = schedule(types, from, to, &conf);

        println!("Found {} entries", entries.len());
        for entry in entries.iter() {
            println!("{}", entry);
        }

        panic!("===o,o===");
    }
}

