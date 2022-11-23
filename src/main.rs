use crate::errors::exit_with_ret_code;
use std::process::exit;

#[macro_use]
extern crate scan_fmt;

mod child;
mod cli;
mod config;
mod container;
mod errors;
mod ipc;

fn main() {
    let args = cli::parse_args();
    match args {
        Ok(ar) => {
            log::info!("{:?}", ar);
            exit_with_ret_code(container::start(ar));
        }
        Err(e) => {
            log::error!("Error while parsing arguments: \n\t{}", e);
            exit(e.get_ret_code());
        }
    }
}
