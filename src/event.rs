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

impl Event {
    /// Print time of start and end only.
    pub fn fmt_span_time(&self) -> String {
        self.fmt_span("%R")
    }

    /// Full year-month-day hour:min format.
    pub fn fmt_span_full(&self) -> String {
        self.fmt_span("%F %R")
    }

    /// Format start - end with a given datetime format string.
    pub fn fmt_span(&self, format_str: &str) -> String {
        format!("{} - {}",
            time::strftime(format_str, &self.start).unwrap(),
            time::strftime(format_str, &self.end).unwrap(),
        )
    }

    /// Don't print date.
    pub fn fmt_time_only(&self) -> String {
        format!("{} {} {} {}",
            self.fmt_span_time(),
            self.name,
            self.activity,
            self.loc
        )
    }

    pub fn fmt_full(&self) -> String {
        format!("{} {} {} {}",
            self.fmt_span_full(),
            self.name,
            self.activity,
            self.loc
        )
    }
}

impl Show for Event {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "{}", self.fmt_full())
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
