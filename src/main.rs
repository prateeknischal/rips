extern crate clap;

mod host;
mod ip;
mod utils;

use host::hostip;
use ip::ipengine;

use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let raw = Arg::with_name("raw")
        .short("r")
        .long("raw")
        .help("Print output in raw form");

    let expand = SubCommand::with_name("expand")
        .about("Expand an IP or a subnet")
        .arg(Arg::with_name("subnet").help("A single IP or subnet to expand"))
        .arg(raw.clone());

    let net = SubCommand::with_name("net")
        .about("List all network interfaces")
        .arg(raw.clone());

    let belong = SubCommand::with_name("belong")
        .arg(
            Arg::with_name("in")
                .help("Check if a list of IPs belong to a parent")
                .takes_value(true)
                .value_name("IPs")
                .min_values(2),
        )
        .arg(raw.clone())
        .arg(
            Arg::with_name("invert")
                .help("Print missing IPs if invert is true")
                .short("i")
                .long("invert")
                .takes_value(false),
        );

    let cli = App::new("rips")
        .version("0.1.0")
        .author("github @prateeknischal")
        .subcommand(expand)
        .subcommand(net)
        .subcommand(belong)
        .arg(raw.clone());

    let matches = cli.get_matches();

    if let Some(exp) = matches.subcommand_matches("expand") {
        let mut raw = false;
        raw = matches.is_present("raw") || exp.is_present("raw");
        if let Some(snet) = exp.value_of("subnet") {
            ipengine::expand_subnet(snet, raw);
        }
    }

    if let Some(nt) = matches.subcommand_matches("net") {
        let mut raw = false;
        raw = matches.is_present("row") || nt.is_present("raw");

        hostip::list_interfaces(raw);
    }

    if let Some(bl) = matches.subcommand_matches("belong") {
        let invert = bl.is_present("invert");
        if let Some(ips) = bl.values_of("in") {
            let iplist = ips.collect::<Vec<&str>>();
            let par = iplist[0];
            let ok = ipengine::belongs(par, iplist, invert);
            process::exit(!ok as i32);
        }
    }
}
