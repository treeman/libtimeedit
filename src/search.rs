use request::request;
use parse;
use info::{ Type, TypeInfo, Course, Group };

/// Search for a match.
///
/// First search for a match in groups and as a fallback
/// search for a matching course.
///
/// Never mix courses with groups in result.
pub fn search(string: &str, base: &str) -> (Vec<TypeInfo>, Type) {
    let groups = group_search(string, base);
    if !groups.is_empty() {
        (groups, Group)
    } else {
        (course_search(string, base), Course)
    }
}

pub fn course_search(string: &str, base: &str) -> Vec<TypeInfo> {
    type_search(string, Course, base)
}

pub fn group_search(string: &str, base: &str) -> Vec<TypeInfo> {
    type_search(string, Group, base)
}

fn type_search(string: &str, t: Type, base: &str) -> Vec<TypeInfo> {
    // TODO use json when searching for things?
    let url = format!("{}/objects.html?max=100&partajax=t&sid=3&search_text={}&types={}",
                      base, string, t.num_id());
    let txt = request(url[]);

    parse::search_res(txt[], t)
}

