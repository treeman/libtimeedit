use url::Url;
use time;
use std::time::Duration;

use request::request;
use schedule::schedule;
use parse;
use info::{ Type };

// TODO
//fn search(string: &str) -> Vec<Type> {

//}

fn course_search(string: &str) -> Vec<Type> {
    type_search(string, 219)
}

fn group_search(string: &str) -> Vec<Type> {
    type_search(string, 205)
}

fn type_search(string: &str, typ: int) -> Vec<Type> {
    //println!("Searching for {}", course);
    // TODO use json when searching for things?
    // https://se.timeedit.net/web/liu/db1/schema/objects.json?max=100&sid=3&search_text=TATA&types=219&fe=132.0&fe=115.20132,20141,20142
    // instead
    // fe is history or something?
    let url = format!("https://se.timeedit.net/web/liu/db1/schema/objects.html?max=100&partajax=t&im=f&sid=3&l=sv&search_text={}&types={}", string, typ);
    let txt = request(url[]);

    // Chunk them
    let chunks = regex!(
        r#"(?sm)<div id="objectbasketitem.+?objectfieldsextrawrap"#
    );
    let mut types = Vec::new();

    // FIXME document a bit
    // TODO move to a parsing library, when someone writes one.
    for cap in chunks.captures_iter(txt[]) {
        let chunk = cap.at(0);

        let course = regex!(r#"<div class="\s*infoboxtitle\s*">\s*(.+?)\s*</div>"#);
        let caps = course.captures(chunk).unwrap();
        let info = caps.at(1);

        // Info is divided in <course-code>, <course name>, <some other things>
        let slice = parse::split(info, ',');
        let id = slice[0];
        let name = slice[1];

        let re = regex!(r#"data-id="([^"]+)""#);
        let caps = re.captures(chunk).unwrap();
        let data_id = caps.at(1);

        types.push(Type {
            id: id.to_string(),
            name: name.to_string(),
            data_id: data_id.to_string()
        });
    }

    types
}

#[test]
fn search() {
    let from = time::now();
    let to = time::at(from.to_timespec() + Duration::weeks(1));

    let s = "TATA";
    let courses = course_search(s);
    let groups = group_search(s);

    println!("Found {} courses matching {}", courses.len(), s);
    for course in courses.iter() {
        println!("{}", course);
    }

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

    panic!("===o,o===");
}

