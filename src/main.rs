use async_std::net::Ipv4Addr;
use futures::executor::block_on;
use clap::{
    App,
    Arg,
};
use crate::{
    fs::ReadLines,
    net::Ipv4Subnet,
    scanner::Scanner,
};
use std::{
    io,
    fs::File,
    io::Write,
    str::FromStr,
    time::Duration,
};

mod mc;
mod net;
mod scanner;
mod fs;

fn main() {
    let matches = App::new("Scanolotl")
        .arg(Arg::with_name("i")
            .short("i")
            .takes_value(true)
            .required(true)
            .help("Sets the input file path"))
        .arg(Arg::with_name("o")
            .short("o")
            .takes_value(true)
            .default_value("./output.txt")
            .help("Sets the output file path"))
        .arg(Arg::with_name("b")
            .short("b")
            .takes_value(true)
            .default_value("3000")
            .help("Sets the batch size"))
        .arg(Arg::with_name("t")
            .short("t")
            .takes_value(true)
            .default_value("500")
            .help("Sets the timeout in millis"))
        .arg(Arg::with_name("c")
            .short("c")
            .help("Preform a server list ping on every open port"))
        .get_matches();

    let input_path = matches.value_of("i").unwrap();
    let output_path = matches.value_of("o").unwrap();

    let batch_size = match matches.value_of("b").unwrap().parse::<u16>() {
        Ok(batch_size) => batch_size,
        Err(e) => panic!("Failed to parse batch_size '{}'", e),
    };

    let timeout = match matches.value_of("t").unwrap().parse::<u64>() {
        Ok(timeout) => timeout,
        Err(e) => panic!("[!] Failed to parse timeout '{}'", e),
    };

    let check = matches.occurrences_of("c") == 1;

    let ips = match read_input_file(input_path) {
        Ok(ips) => ips,
        Err(e) => panic!("[!] Failed to read input file '{}'", e),
    };

    println!("[>] Loaded {} ips", ips.len());

    let mut scanner = Scanner::new(&ips, batch_size, Duration::from_millis(timeout));

    let mut output_file = match File::create(output_path) {
        Ok(file) => file,
        Err(e) => panic!("[!] Failed to crate output file '{}'", e),
    };

    if check {
        let valid_servers = block_on(scanner.check());
        let servers_json = match serde_json::to_string(&valid_servers) {
            Ok(json) => json,
            Err(e) => panic!("[!] Failed to map vector to json array '{}'", e),
        };

        if let Err(e) = write!(output_file, "{}", servers_json) {
            panic!("[!] Failed to write json to file '{}'", e);
        }
    } else {
        let valid_ips = block_on(scanner.scan());
        for ip in valid_ips {
            if let Err(e) = writeln!(output_file, "{}", ip) {
                panic!("[!] Failed to write ip to file '{}'", e);
            }
        }
    }
}

fn read_input_file(path: &str) -> io::Result<Vec<Ipv4Addr>> {
    let file = File::open(path)?;
    let lines = file.read_lines()?;

    let mut ips = Vec::new();
    for line in lines.iter() {
        if line.starts_with("#") {
            continue;
        }

        if let Ok(subnet) = Ipv4Subnet::from_str(line) {
            ips.append(&mut subnet.hosts());
        } else if let Ok(ip) = Ipv4Addr::from_str(line) {
            ips.push(ip);
        }
    }

    Ok(ips)
}
