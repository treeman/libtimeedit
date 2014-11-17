use std::fmt::{ Show, Formatter, FormatError };
use time;
use time::Tm;

#[deriving(Clone, Eq, PartialEq)]
pub struct Event {
    pub start: Tm,
    pub end: Tm,
    pub name: String,
    pub loc: String,
    pub activity: String,
    pub who: String,
    pub groups: Vec<String>
}

impl Show for Event {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let date_format = "%F %R";
        write!(f, "{} - {} {} {} {}",
            time::strftime(date_format, &self.start).unwrap(),
            time::strftime(date_format, &self.end).unwrap(),
            self.name,
            self.activity,
            self.loc
        )
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.start.to_timespec().cmp(&other.start.to_timespec())
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
