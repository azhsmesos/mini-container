use std::process::exit;
use crate::errors::exit_with_ret_code;

mod cli;
mod errors;
mod config;

fn main() {
    let args = cli::parse_args();
    match args {
        Ok(ar) => {
            log::info!("{:?}", ar);
            exit_with_ret_code(Ok(()));
        }
        Err(e) => {
            log::error!("Error while parsing arguments: \n\t{}", e);
            exit(e.get_ret_code());
        }
    }
}
