use time;
use time::Tm;

use request::request;
use parse;
use info::{ Type, Entry };

pub fn schedule(courses: Vec<Type>, from: Tm, to: Tm) -> Vec<Entry> {
    assert!(courses.len() > 0);

    let base = "https://se.timeedit.net/web/liu/db1/schema/ri.csv?sid=3";
    let mut objects = String::new();
    for course in courses.iter() {
        if !objects.is_empty() {
            objects.push('%');
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
    let txt = request(url[]);

    let lines = parse::split(txt[], '\n');

    // Header is first 3 lines, skip them.
    let entries = lines.slice_from(3);

    let mut res = Vec::new();
    for entry in entries.iter() {
        let split = parse::split(*entry, ',');
        let (startdate, starttime, enddate, endtime) = (split[0], split[1], split[2], split[3]);
        let (name, loc) = (split[4], split[5]);

        let start = match time::strptime(format!("{} {}", startdate, starttime)[], "%F %R") {
            Ok(x) => x,
            Err(e) => panic!(e)
        };
        let end = match time::strptime(format!("{} {}", enddate, endtime)[], "%F %R") {
            Ok(x) => x,
            Err(e) => panic!(e)
        };

        res.push(Entry {
            start: start,
            end: end,
            name: name.to_string(),
            loc: loc.to_string()
        });
    }

    res
}
