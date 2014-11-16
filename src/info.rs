use std::fmt::{Show, Formatter, FormatError};
use time;
use time::Tm;

// TODO name?
#[deriving(Show, Clone)]
pub struct Type {
    pub id: String,
    pub name: String,
    pub data_id: String,
}

pub struct Entry {
    pub start: Tm,
    pub end: Tm,
    pub name: String,
    pub loc: String,
}

impl Show for Entry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let date_format = "%F %R";
        write!(f, "{} - {} {}",
            time::strftime(date_format, &self.start).unwrap(),
            time::strftime(date_format, &self.end).unwrap(),
            self.name
        )
    }
}

