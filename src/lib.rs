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
pub use typeinfo::{ DataId, TypeInfo, Course, Group, Type };
pub use event::{ Event };
pub use schedule::{ schedule, schedule_from_ids };

mod parse;
mod search;
mod schedule;
mod typeinfo;
mod event;
mod request;
