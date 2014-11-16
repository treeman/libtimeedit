#![allow(unknown_features)]
#![feature(slicing_syntax)]
#![feature(phase)]

extern crate http;
extern crate url;
extern crate time;

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub mod parse;
pub mod info;
pub mod search;
pub mod schedule;

mod request;
