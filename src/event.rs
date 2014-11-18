use std::fmt::{ Show, Formatter, FormatError };
use time;
use time::Tm;
use std::time::Duration;

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
        format!("{} {} {} {} {}",
            self.fmt_span_time(),
            self.name,
            self.activity,
            self.loc,
            self.fmt_groups(),
        )
    }

    /// Don't print date or groups.
    pub fn fmt_short(&self) -> String {
        format!("{} {} {} {}",
            self.fmt_span_time(),
            self.name,
            self.activity,
            self.loc
        )
    }

    /// Fully print.
    pub fn fmt_full(&self) -> String {
        format!("{} {} {} {} {}",
            self.fmt_span_full(),
            self.name,
            self.activity,
            self.loc,
            self.fmt_groups()
        )
    }

    pub fn fmt_pretty(&self) -> String {
        if self.same_day() {
            self.fmt_same_day()
        } else {
            self.fmt_full()
        }
    }

    fn fmt_same_day(&self) -> String {
        let date = if self.starts_today() {
            "Today".to_string()
        } else if self.starts_tomorrow() {
            "Tomorrow".to_string()
        } else if self.starts_in_one_week() {
            time::strftime("%a", &self.start).unwrap()
        } else {
            time::strftime("%e %b", &self.start).unwrap()
        };
        format!("{} {} {} {} {} {}",
            date,
            self.fmt_span_time(),
            self.name,
            self.activity,
            self.loc,
            self.fmt_groups(),
        )
    }

    fn fmt_groups(&self) -> String {
        if self.groups.is_empty() {
            String::new()
        } else {
            format!("({})", join(&self.groups, ", "))
        }
    }

    /// Is the event during a single day?
    pub fn same_day(&self) -> bool {
        same_day(&self.start, &self.end)
    }

    /// Does the event start today?
    pub fn starts_today(&self) -> bool {
        let today = time::now();
        same_day(&self.start, &today)
    }

    /// Does the event start tomorrow?
    pub fn starts_tomorrow(&self) -> bool {
        let tomorrow = time::at(time::get_time() + Duration::days(1));
        same_day(&self.start, &tomorrow)
    }

    /// Does the event start in 1 week?
    pub fn starts_in_one_week(&self) -> bool {
        let a_week = time::get_time() + Duration::weeks(1);
        self.start.to_timespec() < a_week
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

pub fn filter_upcoming(events: Vec<Event>) -> Vec<Event> {
    let now = time::get_time();
    events.into_iter().skip_while(|x| {
        x.start.to_timespec() < now
    }).collect()
}

pub fn filter_today(events: Vec<Event>) -> Vec<Event> {
    events.into_iter().filter(|x| {
        x.starts_today()
    }).collect()
}

pub fn filter_tomorrow(events: Vec<Event>) -> Vec<Event> {
    events.into_iter().filter(|x| {
        x.starts_tomorrow()
    }).collect()
}

fn same_day(a: &Tm, b: &Tm) -> bool {
    a.tm_mday == b.tm_mday &&
    a.tm_mon == b.tm_mon &&
    a.tm_year == b.tm_year
}

// TODO move away
pub fn join(xs: &Vec<String>, between: &str) -> String {
    let mut res = String::new();
    for x in xs.iter() {
        if !res.is_empty() {
            res.push_str(between);
        }
        res.push_str(x[]);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::{ Event, filter_upcoming, filter_today, filter_tomorrow };
    use regex::Regex;
    use time;
    use time::Tm;
    use std::time::Duration;

    fn simple_event(start: Tm, id: &str) -> Event {
        let end = time::at(start.to_timespec() + Duration::hours(1));
        Event {
            start: start,
            end: end,
            name: id.to_string(),
            loc: "Location".to_string(),
            activity: "Activity".to_string(),
            who: "Who".to_string(),
            groups: vec!["Group.1".to_string(), "Group.2".to_string()],
        }
    }

    #[test]
    fn filtering() {
        let now = time::now();
        let mut hour_ago = time::get_time();
        hour_ago.sec = hour_ago.sec - 3600;
        let hour_from_now = time::at(time::get_time() + Duration::hours(1));
        let tomorrow = time::at(time::get_time() + Duration::days(1));
        let week_from_now = time::at(time::get_time() + Duration::weeks(1));

        // Yeah these will not always pass...
        let _1h = simple_event(time::at(hour_ago), "-1h");
        let now = simple_event(now, "now");
        let p1h = simple_event(hour_from_now, "+1h");
        let p1d = simple_event(tomorrow, "+1d");
        let p1w = simple_event(week_from_now, "+1w");

        let events = vec![_1h.clone(), now.clone(), p1h.clone(), p1d.clone(), p1w.clone()];

        assert_eq!(filter_upcoming(events.clone()), vec![p1h.clone(), p1d.clone(), p1w.clone()]);
        assert_eq!(filter_today(events.clone()), vec![_1h.clone(), now.clone(), p1h.clone()]);
        assert_eq!(filter_tomorrow(events.clone()), vec![p1d.clone()]);
    }

    fn matches(got: String, re: Regex) {
        if !re.is_match(got[]) {
            println!("`{}` != `{}`", got, re);
            assert!(false);
        }
    }

    #[test]
    fn printing() {
        let today = simple_event(time::now(), "0d");
        let tomorrow = simple_event(time::at(time::get_time() + Duration::days(1)), "1d");
        let three_days = simple_event(time::at(time::get_time() + Duration::days(3)), "3d");
        let two_weeks = simple_event(time::at(time::get_time() + Duration::weeks(2)), "2w");

        assert!(today.starts_today());
        assert!(!today.starts_tomorrow());
        assert!(today.starts_in_one_week());

        assert!(!tomorrow.starts_today());
        assert!(tomorrow.starts_tomorrow());
        assert!(tomorrow.starts_in_one_week());

        assert!(!three_days.starts_today());
        assert!(!three_days.starts_tomorrow());
        assert!(three_days.starts_in_one_week());

        assert!(!two_weeks.starts_today());
        assert!(!two_weeks.starts_tomorrow());
        assert!(!two_weeks.starts_in_one_week());

        matches(today.fmt_span_time(),
            regex!(r"^\d{2}:\d{2} - \d{2}:\d{2}$"));

        matches(today.fmt_pretty(),
            regex!(r"^Today \d{2}:\d{2} - \d{2}:\d{2} 0d Activity Location \(Group\.1, Group\.2\)$"));
        matches(tomorrow.fmt_pretty(),
            regex!(r"^Tomorrow \d{2}:\d{2} - \d{2}:\d{2} 1d Activity Location \(Group\.1, Group\.2\)$"));
        matches(three_days.fmt_pretty(),
            regex!(r"^... \d{2}:\d{2} - \d{2}:\d{2} 3d Activity Location \(Group\.1, Group\.2\)$"));
        matches(two_weeks.fmt_pretty(),
            regex!(r"^.. ... \d{2}:\d{2} - \d{2}:\d{2} 2w Activity Location \(Group\.1, Group\.2\)$"));
    }
}
