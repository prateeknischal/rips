extern crate clap;

mod host;
mod ip;
mod utils;

use host::hostip;
use ip::ipengine;

use std::process;

use clap::{App, Arg, SubCommand};

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

    let cli = App::new("rips")
        .version("0.1.0")
        .author("github @prateeknischal")
        .subcommand(expand)
        .subcommand(net)
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
    /*
        hostip::list_interfaces(false);
        ipengine::expand_subnet("10.57.162.96/25", true);
        ipengine::find_parent(
            vec![
                "1.1.1.1",
                "10.15.1.0/23",
                "10.15.0.0/24",
                "10.57.162.124",
                "10.57.162.125",
                "10.57.162.168",
                "10.57.162.169",
            ],
            true,
        );
    */
}
