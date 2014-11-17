use time;

use info::{ Type, TypeInfo, Course, Group, Entry, DataId };

// TODO move somewhere?
// Split a string on a character, trim and remove empty strings.
pub fn split<'a>(s: &'a str, c: char) -> Vec<&'a str> {
    s.split(|x: char| -> bool {
        x == c
    }).map(|s: &'a str| -> &'a str {
        s.trim()
    }).filter(|s: &&str| -> bool {
        *s != ""
    }).collect()
}

pub fn search_res(txt: &str, t: Type) -> Vec<TypeInfo> {
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

        // If it's a course it's <course id>, <course name>
        // But if it's a group it's <group description>, <group id>
        let slice = split(info, ',');
        let (id, name) = match t {
            Course => (slice[0], slice[1]),
            Group => (slice[1], slice[0]),
        };

        let re = regex!(r#"data-id="([^"]+)""#);
        let caps = re.captures(chunk).unwrap();
        let data_id = DataId::new(caps.at(1));

        types.push(TypeInfo::new(id, name, data_id));
    }

    types
}

pub fn schedule_res(txt: &str) -> Vec<Entry> {
    let lines = split(txt[], '\n');

    // Header is first 3 lines, skip them.
    let entries = lines.slice_from(3);

    let mut res = Vec::new();
    for entry in entries.iter() {
        //println!("{}", entry);

        // FIXME name can be "<course1, course2>" as well!
        let split = split(*entry, ',');
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

