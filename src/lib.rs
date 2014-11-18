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

pub use search::{ multi_search, search, course_search, group_search };
pub use typeinfo::{ DataId, TypeInfo, Type };
pub use event::{ Event };
pub use schedule::{ schedule, schedule_from_ids };
pub use parse::{ string_lit_comma_split };

mod parse;
mod search;
mod schedule;
mod typeinfo;
mod event;
mod request;
