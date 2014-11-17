#![allow(unknown_features)]
#![feature(slicing_syntax)]
#![feature(phase)]

extern crate http;
extern crate url;
extern crate time;
extern crate serialize;

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub use search::{ search, course_search, group_search };
pub use info::{ DataId, TypeInfo, Event, Course, Group, Type };
pub use schedule::{ schedule, schedule_from_ids };

pub mod parse;
pub mod info;
pub mod search;
pub mod schedule;

mod request;
