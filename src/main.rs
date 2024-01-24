use date_clock::*;
use std::{
    fmt,
    io::{self, prelude::*},
};

#[derive(fcla::FromArgs)]
struct Args {}

fn main() -> fcla::MainResult<RuntimeError> {
    let Args {} = fcla::parse_cla()?.args;
    writeln!(io::stdout(), "{}", Time::now()).map_err(RuntimeError)?;
    Ok(())
}

struct RuntimeError(io::Error);

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;

        fmt::Display::fmt(inner, f)
    }
}
