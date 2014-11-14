#![allow(unknown_features)]
#![feature(slicing_syntax)]
#![feature(phase)]

extern crate http;
extern crate url;

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub mod parse;
