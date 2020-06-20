mod host;
mod ip;
mod utils;

use host::hostip;
use ip::ipengine;

fn main() {
    hostip::list_interfaces();
    ipengine::expand_subnet("10.49.129.32/30");
}
