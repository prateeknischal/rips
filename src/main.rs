mod host;
mod ip;
mod utils;

use host::hostip;
use ip::ipengine;

fn main() {
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
}
