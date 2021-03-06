#[macro_use]
extern crate clap;
use clap::App;

use findomain::{get_subdomains, read_from_file};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if matches.is_present("target") && matches.is_present("output") {
        let target: String = matches.values_of("target").unwrap().collect();
        let with_output = "y";
        let file_format: String = matches.values_of("output").unwrap().collect();
        if matches.is_present("ip") {
            let with_ip = "y";
            get_subdomains(&target, &with_ip, &with_output, &file_format)
        } else {
            let with_ip = "";
            get_subdomains(&target, &with_ip, &with_output, &file_format)
        }
    } else if matches.is_present("target") {
        let target: String = matches.values_of("target").unwrap().collect();
        let with_output = "n";
        let file_format = "n";
        if matches.is_present("ip") {
            let with_ip = "y";
            get_subdomains(&target, &with_ip, &with_output, &file_format)
        } else {
            let with_ip = "";
            get_subdomains(&target, &with_ip, &with_output, &file_format)
        }
    } else if matches.is_present("file") && matches.is_present("output") {
        let with_output = "y";
        let file_format: String = matches.values_of("output").unwrap().collect();
        let file: String = matches.values_of("file").unwrap().collect();
        if matches.is_present("ip") {
            let with_ip = "y";
            read_from_file(&file, &with_ip, &with_output, &file_format)
        } else {
            let with_ip = "";
            read_from_file(&file, &with_ip, &with_output, &file_format)
        }
    } else if matches.is_present("file") {
        let with_output = "n";
        let file_format = "n";
        let file: String = matches.values_of("file").unwrap().collect();
        if matches.is_present("ip") {
            let with_ip = "y";
            read_from_file(&file, &with_ip, &with_output, &file_format)
        } else {
            let with_ip = "";
            read_from_file(&file, &with_ip, &with_output, &file_format)
        }
    }
}
