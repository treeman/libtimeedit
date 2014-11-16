use url::Url;
use time;
use std::time::Duration;

use request::request;
use schedule::schedule;
use parse;
use info::{ Type, TypeInfo, Course, Group };

/// Search for a match.
///
/// First search for a match in groups and as a fallback
/// search for a matching course.
///
/// Never mix courses with groups in result.
pub fn search(string: &str) -> (Vec<TypeInfo>, Type) {
    let groups = group_search(string);
    if !groups.is_empty() {
        (groups, Group)
    } else {
        (course_search(string), Course)
    }
}

pub fn course_search(string: &str) -> Vec<TypeInfo> {
    type_search(string, Course)
}

pub fn group_search(string: &str) -> Vec<TypeInfo> {
    type_search(string, Group)
}

fn type_search(string: &str, t: Type) -> Vec<TypeInfo> {
    //println!("Searching for {}", course);
    // TODO use json when searching for things?
    // https://se.timeedit.net/web/liu/db1/schema/objects.json?max=100&sid=3&search_text=TATA&types=219&fe=132.0&fe=115.20132,20141,20142
    // instead
    // fe is history or something?
    let url = format!("https://se.timeedit.net/web/liu/db1/schema/objects.html?max=100&partajax=t&im=f&sid=3&l=sv&search_text={}&types={}", string, t.num_id());
    let txt = request(url[]);

    parse::search_res(txt[], t)
}

#[test]
fn test_search() {
    //let from = time::now();
    //let to = time::at(from.to_timespec() + Duration::weeks(1));

    //let s = "TATA49";

    //let (types, typ) = search(s);
    //let ts = match typ {
        //Course => "courses",
        //Group => "groups",
    //};
    //println!("Found {} {}", types.len(), ts);
    //for t in types.iter() {
        //println!("{}", t);
    //}

    //let courses = course_search(s);
    //let groups = group_search(s);

    //println!("Found {} courses matching {}", courses.len(), s);
    //for course in courses.iter() {
        //println!("{}", course);
    //}

    //println!("Found {} groups matching {}", groups.len(), s);
    //for group in groups.iter() {
        //println!("{}", group);
    //}

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

    //let entries = schedule(types, from, to);

    //println!("Found {} entries", entries.len());
    //for entry in entries.iter() {
        //println!("{}", entry);
    //}

    //panic!("===o,o===");
}

