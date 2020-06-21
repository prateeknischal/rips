extern crate pnet;

use crate::utils::display;
use ipnetwork;
use pnet::datalink;
use pnet::util;
use prettytable::{Attr, Cell, Row, Table};

#[derive(Debug)]
pub struct Interface {
    name: String,
    mac: Option<util::MacAddr>,
    ip: ipnetwork::IpNetwork,
    flags: u32,
}

impl Interface {
    pub fn new(
        name: &String,
        mac: &Option<util::MacAddr>,
        ip: &ipnetwork::IpNetwork,
        flags: u32,
    ) -> Self {
        Interface {
            name: name.clone(),
            mac: *mac,
            ip: *ip,
            flags,
        }
    }

    pub fn serialize(&self) -> Vec<String> {
        let mut res = Vec::<String>::new();
        // The order of the strings
        // name, mac, ip, type, flag
        res.push(self.name.clone());
        res.push(match self.mac {
            None => String::from(""),
            Some(m) => m.to_string(),
        });
        res.push(self.ip.to_string());
        res.push(match self.ip.is_ipv4() {
            true => String::from("v4"),
            false => String::from("v6"),
        });
        res.push(self.flags.to_string());

        res
    }
}

pub fn list_interfaces(raw: bool) {
    let ifs = datalink::interfaces();
    let mut res = Vec::<Interface>::new();

    let mut v = Vec::<Vec<String>>::new();
    for it in ifs {
        for ip in &it.ips {
            let x = Interface::new(&it.name, &it.mac, ip, it.flags);
            v.push(x.serialize());
        }
    }

    display::display(
        &vec![
            String::from("Name"),
            String::from("MAC"),
            String::from("IP"),
            String::from("Version"),
            String::from("Flags"),
        ],
        &v,
        raw,
    );
}
