extern crate pnet;

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

pub fn list_interfaces() {
    let ifs = datalink::interfaces();
    let mut res = Vec::<Interface>::new();

    for it in ifs {
        for ip in &it.ips {
            res.push(Interface::new(&it.name, &it.mac, ip, it.flags));
        }
    }

    display(
        &vec![
            String::from("Name"),
            String::from("MAC"),
            String::from("IP"),
            String::from("Version"),
            String::from("Flags"),
        ],
        &res,
    );
}

pub fn display(header: &Vec<String>, rows: &Vec<Interface>) {
    let mut table = Table::new();

    // Create the header part of the table
    let mut headers_vec = Vec::new();
    for c in header {
        headers_vec.push(Cell::new(&c).with_style(Attr::Bold));
    }
    table.add_row(Row::new(headers_vec));

    for r in rows {
        // For all the interfaces
        let row = r.serialize();
        let mut row_vec = Vec::new();
        for c in row {
            // For all the records the serialize returns.
            row_vec.push(Cell::new(&c));
        }
        table.add_row(Row::new(row_vec));
    }
    table.printstd();
}
