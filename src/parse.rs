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

