extern crate getopts;
extern crate serial;

mod file_loader;
mod printer_driver;

use std::env;
use getopts::Options;

use file_loader::FileLoader;
use printer_driver::PrinterDriver;

fn init_options(args: Vec<String>) -> getopts::Matches {
    let mut options = Options::new();

    options.optopt("p", "port", "port", "PORT");
    options.optflag("t", "test", "TEST");

    match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    }
}

fn main() {
    let matches = init_options(env::args().collect::<Vec<String>>());

    let port = matches.opt_str("p");
    let test_flag = matches.opt_present("t");
    let path = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        panic!("File name not found");
    };

    let mut file = FileLoader::new();

    if test_flag {
        file.parse(path);
        for command in file {
            println!("{}", command.to_string());
        }
    } else if port != None {
        let mut driver = PrinterDriver::new(port.unwrap());
        file.parse(path);

        for command in file {
            driver.send(format!("{}\n", command.to_string()));

            'waiting_ok: loop {
                let response = driver.wait_to_response();
                if response.contains("OK") {
                    break 'waiting_ok;
                }
            }
        }
    }
}