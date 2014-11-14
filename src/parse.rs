#![warn(dead_code)]

use http::client::RequestWriter;
use http::method::Get;
//use http::headers::HeaderEnum;
//use regex::Regex;
use url::Url;
use std::str;

#[deriving(Show)]
struct Course {
    id: String,
    name: String,
    data_id: String,
}

// Split a string on a character, trim and remove empty strings.
fn split<'a>(s: &'a str, c: char) -> Vec<&'a str> {
    s.split(|x: char| -> bool {
        x == c
    }).map(|s: &'a str| -> &'a str {
        s.trim()
    }).filter(|s: &&str| -> bool {
        *s != ""
    }).collect()
}

fn search(course: &str) -> Vec<Course> {
    println!("Searching for {}", course);

    let url = format!("https://se.timeedit.net/web/liu/db1/schema/objects.html?max=15&fr=t&partajax=t&im=f&sid=3&l=sv&search_text={}&types=219&fe=132.0&fe=115.20132,20141,20142", course);
    let txt = request(url[]);

    // Chunk them
    let chunks = regex!(
        r#"(?sm)<div id="objectbasketitem.+?objectfieldsextrawrap"#
    );
    let mut courses = Vec::new();

    // FIXME document a bit
    // TODO move to a parsing library, when someone writes one.
    for cap in chunks.captures_iter(txt[]) {
        let chunk = cap.at(0);

        let course = regex!(r#"<div class="\s*infoboxtitle\s*">\s*(.+?)\s*</div>"#);
        let caps = course.captures(chunk).unwrap();
        let info = caps.at(1);

        // Info is divided in <course-code>, <course name>, <some other things>
        let slice = split(info, ',');
        let id = slice[0];
        let name = slice[1];

        let course = regex!(r#"data-id="([^"]+)""#);
        let caps = course.captures(chunk).unwrap();
        let data_id = caps.at(1);

        courses.push(Course {
            id: id.to_string(),
            name: name.to_string(),
            data_id: data_id.to_string()
        });
    }

    for course in courses.iter() {
        println!("{}", course);
    }

    courses
}

fn schedule(courses: Vec<Course>) {
    let base = "https://se.timeedit.net/web/liu/db1/schema/ri.csv?p=140703-141231&sid=3";
    // FIXME more than 1 course!
    let ref c = courses[0];
    let url = format!("{}&objects={}", base, c.data_id);
    println!("schedule: {}", url);
    let txt = request(url[]);

    let lines = split(txt[], '\n');

    // Header is first 3 lines, skip them.
    let entries = lines.slice_from(3);

    for entry in entries.iter() {
        let split = split(*entry, ',');
        let (startdate, starttime, enddate, endtime) = (split[0], split[1], split[2], split[3]);
        let (name, loc) = (split[4], split[5]);
        println!("{}:{} - {}:{}, {} in {}", startdate, starttime, enddate, endtime, name, loc);
    }
}

fn request(url: &str) -> String {
    let url = Url::parse(url).ok().expect("Invalid URL :-(");
    let request: RequestWriter = RequestWriter::new(Get, url).unwrap();
    //println!("[33;1mRequest[0m");
    //println!("[33;1m=======[0m");
    //println!("");
    //println!("[1mURL:[0m {}", request.url);
    //println!("[1mRemote address:[0m {}", request.remote_addr);
    //println!("[1mMethod:[0m {}", request.method);
    //println!("[1mHeaders:[0m");
    //for header in request.headers.iter() {
        //println!(" - {}: {}", header.header_name(), header.header_value());
    //}
    //println!("");
    //println!("[33;1mResponse[0m");
    //println!("[33;1m========[0m");
    //println!("");
    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_request) => panic!("This example can progress no further with no response :-("),
    };
    //println!("[1mStatus:[0m {}", response.status);
    //println!("[1mHeaders:[0m");
    //for header in response.headers.iter() {
        //println!(" - {}: {}", header.header_name(), header.header_value());
    //}
    println!("[1mBody:[0m");
    let body = match response.read_to_string() {
        Ok(body) => body,
        Err(err) => panic!("Reading response failed: {}", err),
    };
    //let s = str::from_utf8(body[]).expect("Uh oh, response wasn't UTF-8");
    //println(s);

    body
    //s.to_string()
}

#[test]
fn parse() {
    let courses = search("TATA49");
    schedule(courses);
    panic!("===o,o===");
}

