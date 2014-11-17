use time;
use std::str;

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

/// Split at each comma, but ignore commas inside string literals.
///
/// Also prune empty splits and remove opening/trailing "
pub fn string_lit_comma_split(s: &str) -> Vec<&str> {
    enum Mode {
        Comma,
        StringLit,
    }

    // utf8 byte code for , and "
    let comma = 44;
    let strlit = 34;

    let mut at = 0;
    let mut from = 0;
    let mut mode = Comma;
    let mut splits = Vec::new();

    let bytes = s.as_bytes();
    while at < bytes.len() {
        let byte = bytes[at];
        match mode {
            Comma => {
                if byte == comma {
                    splits.push(bytes[from..at]);
                    from = at + 1;
                } else if byte == strlit { // "
                    from = at + 1;
                    mode = StringLit;
                }
            },
            StringLit => {
                if byte == strlit && at != from {
                    splits.push(bytes[from..at]);
                    from = at + 1;
                    mode = Comma;
                }
            }
        }
        at += 1;
    }
    splits.push(bytes[from..]);
    splits.iter().map(|bs| {
        str::from_utf8(*bs).unwrap()
    }).map(|s| {
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

    // Easy empty default.
    fn ind<'a>(split: &Vec<&'a str>, pos: uint) -> &'a str {
        match split[].get(pos) {
            Some(x) => *x,
            None => "",
        }
    }

    let mut res = Vec::new();
    for entry in entries.iter() {
        let xs = string_lit_comma_split(*entry);

        let (startdate, starttime, enddate, endtime) = (xs[0], xs[1], xs[2], xs[3]);
        let (name, loc) = (ind(&xs, 4), ind(&xs, 5));
        let (activity, who, groups) = (ind(&xs, 6), ind(&xs, 7), ind(&xs, 8));

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
            loc: loc.to_string(),
            activity: activity.to_string(),
            who: who.to_string(),
            groups: split(groups, ',').iter().map(|x| x.to_string()).collect(),
        });
    }

    res
}

#[cfg(test)]
mod tests {
    use super::string_lit_comma_split;

    #[test]
    fn string_lit() {
        assert_eq!(string_lit_comma_split(r#"a, b, c, "d, e", f"#),
                                          vec!["a", "b", "c", "d, e", "f"]);
    }

    #[test]
    fn string_lit_utf8() {
        assert_eq!(string_lit_comma_split(r#"å, ä, ö"#),
            vec!["å", "ä", "ö"]);
    }
}

