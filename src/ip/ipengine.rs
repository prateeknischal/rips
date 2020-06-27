extern crate ipnetwork;

use crate::utils::display;
use ipnetwork::IpNetwork;
use std::collections::VecDeque;

/// Check if all the given IPs/subnets belong to a given subnet or not.
/// Depending on the flag invert, print included or excluded IPs.
/// invert = true prints the matching IPs.
pub fn belongs(parent: &str, child: Vec<&str>, invert: bool) -> bool {
    let parent_ip = parent.parse();
    if parent_ip.is_err() {
        return false;
    };
    let parent_ip: IpNetwork = parent_ip.unwrap();
    let mut ok = true;
    for c in child {
        match c.parse::<IpNetwork>() {
            Ok(ip) => {
                let v = parent_ip.contains(ip.ip());
                if v == invert {
                    println!("{}", c);
                }

                ok = ok & v;
            }
            Err(_e) => {
                println!("Failed to parse {}", c);
                ok = false;
            }
        }
    }

    ok
}

/// Expand the subnet to all it's child IPs and print it out in a
/// tabular format.
pub fn _expand_subnet(parent: &str) -> Vec<Vec<String>> {
    let parent_ip = parent.parse();
    if parent_ip.is_err() {
        eprintln!("Invalid IP or subnet");
        return Vec::<Vec<String>>::new();
    }

    let parent_ip: IpNetwork = parent_ip.unwrap();
    let v: Vec<Vec<String>> = parent_ip
        .iter()
        .map(|x| {
            let s = x.to_string();
            vec![s]
        })
        .collect();
    // let sz: u128 = match parent_ip.size() {
    //    NetworkSize::V4(v4) => v4 as u128,
    //    NetworkSize::V6(v6) => v6,
    // };

    return v;
}

pub fn expand_subnet(parent: &str, raw: bool) {
    let v = _expand_subnet(parent);
    display::display(
        &vec![String::from(format!("Children ({})", v.len()))],
        &v,
        raw,
    );
}

/// Find the lowest common ancestor(s) for the list of IP/subnets
/// and print. The function will try to condense all the IPs within
/// the /16 range and print the list of IPs that contain all the
/// supplied IPs. eg: for "10.0.1.15/31" and "10.0.1.32/27" the parent
/// is "10.0.1.15/26" as it contains both of them.
fn _find_parent(children: Vec<&str>) -> Vec<String> {
    let mut ips = Vec::<IpNetwork>::new();
    for c in children {
        let ip = c.parse();
        if ip.is_err() {
            continue;
        }

        ips.push(ip.unwrap());
    }

    ips.sort();
    let mut st = VecDeque::<IpNetwork>::new();
    let mut top = 0;
    for &ip in ips.iter() {
        let l = st.len();
        if l == 0 {
            st.push_back(ip);
            continue;
        }

        let x = st.pop_back().unwrap();
        match closest_parent(x, ip, 16u8) {
            Some(p) => {
                st.push_back(p);
            }
            None => {
                st.push_back(x);
                st.push_back(ip);
            }
        }
    }

    let mut v: Vec<String> = st.into_iter().map(|x| x.to_string()).collect();

    return v;
}

pub fn find_parent(children: Vec<&str>, raw: bool) {
    let v: Vec<String> = _find_parent(children);
    let mut rows: Vec<Vec<String>> = Vec::<Vec<String>>::new();

    for t in v {
        rows.push(vec![t]);
    }
    display::display(&vec!["Parents".to_string()], &rows, raw);
}

/// Find the closest ancestor of 2 IPs upto oc number of prefix bits..
/// eg, if oc = 0, it will search within /32, if oc=2, it will search
/// till /30. Right now, it only supports IPv4.
/// This function assumes that a <= b.
fn closest_parent(a: IpNetwork, b: IpNetwork, oc: u8) -> Option<IpNetwork> {
    if !a.is_ipv4() || !b.is_ipv4() {
        return None;
    }

    if a.contains(b.ip()) {
        return Some(a);
    }

    if a.prefix() < oc || b.prefix() < oc {
        return None;
    }

    let mut pr = a.prefix();
    let mut par = IpNetwork::new(a.ip(), a.prefix()).unwrap();
    while pr >= oc {
        if par.contains(a.ip()) && par.contains(b.ip()) {
            break;
        }
        pr -= 1u8;
        par = IpNetwork::new(par.ip(), pr).unwrap();
    }

    if par.contains(a.ip()) && par.contains(b.ip()) {
        return Some(par);
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ip_does_belong() {
        assert!(belongs("10.36.214.32/27", vec!["10.36.214.38"]));
    }

    #[test]
    fn ip_does_not_belong() {
        assert!(!belongs("10.36.214.32/27", vec!["1.1.1.1"]));
    }

    #[test]
    fn ip_err_check() {
        assert!(!belongs("a.1.4.z", vec![]));
    }

    #[test]
    fn ip_expand() {
        let x = _expand_subnet("10.49.129.32/28");
        assert_eq!(16usize, x.len());
    }

    #[test]
    fn ip_merge() {
        let x = _find_parent(vec![
            "10.57.162.124/32",
            "10.57.162.142/24",
            "10.36.214.32/27",
            "10.36.214.40/28",
        ]);

        assert_eq!(
            vec![
                "10.36.214.32/27".to_string(),
                "10.57.162.124/24".to_string(),
            ],
            x
        );
    }

    #[test]
    fn ip_closest_parent() {
        let x = closest_parent(
            "10.36.214.32/27".parse().unwrap(),
            "10.36.214.40/28".parse().unwrap(),
            16,
        )
        .unwrap();

        assert_eq!(x, "10.36.214.32/27".parse().unwrap());
    }

    #[test]
    fn ip_closest_parent2() {
        let x = closest_parent(
            "10.0.1.15/31".parse().unwrap(),
            "10.0.1.32/27".parse().unwrap(),
            16,
        )
        .unwrap();

        assert_eq!(x.ip(), "10.0.1.15/26".parse::<IpNetwork>().unwrap().ip());
    }

    #[test]
    fn ip_closest_error() {
        let x = closest_parent(
            "10.0.1.15/31".parse().unwrap(),
            "1.1.1.1/32".parse().unwrap(),
            16,
        );

        match x {
            None => assert!(true),
            Some(x) => assert!(false),
        }
    }
}
