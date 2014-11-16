use time;
use time::Tm;

use request::request;
use parse;
use info::{ Type, TypeInfo, Entry };

pub fn schedule(courses: Vec<TypeInfo>, from: Tm, to: Tm) -> Vec<Entry> {
    assert!(courses.len() > 0);

    let base = "https://se.timeedit.net/web/liu/db1/schema/ri.csv?sid=3";
    let mut objects = String::new();
    for course in courses.iter() {
        if !objects.is_empty() {
            objects.push_str("%2C");
        }
        objects.push_str(course.data_id[]);
    }
    //println!("objects: {}", objects);
    let date_format = "%y%m%d";
    let url = format!("{}&p={}-{}&objects={}",
                base,
                time::strftime(date_format, &from).unwrap(),
                time::strftime(date_format, &to).unwrap(),
                objects);
    println!("Requesting url: {}", url);
    let txt = request(url[]);

    parse::schedule_res(txt[])
}

