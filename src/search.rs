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

