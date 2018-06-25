extern crate ansi_term;
extern crate atty;
extern crate chrono;
extern crate clap;
extern crate diff;
extern crate itertools;
extern crate strsim;

extern crate todo_txt;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod compute_changes;
pub mod display_changes;
pub mod stable_marriage;
