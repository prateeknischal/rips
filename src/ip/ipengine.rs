extern crate ipnetwork;

use crate::utils::display;
use ipnetwork::{IpNetwork, NetworkSize};

pub fn belongs(parent: &str, child: Vec<&str>) -> bool {
    let parent_ip = parent.parse();
    if parent_ip.is_err() {
        return false;
    };
    let parent_ip: IpNetwork = parent_ip.unwrap();
    let mut ok = true;
    for c in child {
        match c.parse() {
            Ok(p) => {
                ok &= parent_ip.contains(p);
            }
            Err(_e) => {
                eprintln!("The child IP {} is not valid", c);
                ok = false;
            }
        }
    }

    ok
}

pub fn expand_subnet(parent: &str) -> u128 {
    let parent_ip = parent.parse();
    if parent_ip.is_err() {
        eprintln!("Invalid IP or subnet");
        return 0u128;
    }

    let parent_ip: IpNetwork = parent_ip.unwrap();
    let v: Vec<Vec<String>> = parent_ip
        .iter()
        .map(|x| {
            let s = x.to_string();
            vec![s]
        })
        .collect();
    let sz: u128 = match parent_ip.size() {
        NetworkSize::V4(v4) => v4 as u128,
        NetworkSize::V6(v6) => v6,
    };

    display::display(&vec![String::from(format!("Children ({})", sz))], &v);
    sz
}

pub fn find_parent(childen: Vec<String>) -> &str {
    return "";
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
        assert_eq!(16u128, expand_subnet("10.49.129.32/28"));
    }
}
