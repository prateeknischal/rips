# rips - An IP manipulation utility written in Rust

rips is a command line tool that can manipulate IP addresses and subnets and offer other functionalities over it.

## Features to Implement

The below list is kind of a wishlist that I would want from an tool for this purpose.
- [x] Get the host IP interface details
- [ ] Filters for kind of interface
- [ ] Pretty routing tables
- [x] Subnet expander
- [x] Is an IP in a particular subnet
- [x] Parent Subnet for all the listed IPs
- [x] Parent Subnet for all the listed subnets
- [ ] Possibly the smallest IP space(s) for a list of IPs
- [ ] Possibly the smallest IP space(s) for a list of IP subnets

## CLI Usage

```
rips 0.1.0
github @prateeknischal

USAGE:
    rips [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -r, --raw        Print output in raw form
    -V, --version    Prints version information

SUBCOMMANDS:
    belong
    expand    Expand an IP or a subnet
    help      Prints this message or the help of the given subcommand(s)
    net       List all network interfaces
```

## Sample

### Getting all interfaces
```
$ rips net
+-------+-------------------+------------------------------+---------+-------+
| Name  | MAC               | IP                           | Version | Flags |
+-------+-------------------+------------------------------+---------+-------+
| lo0   | 00:00:00:00:00:00 | 127.0.0.1/8                  | v4      | 32841 |
+-------+-------------------+------------------------------+---------+-------+
| lo0   | 00:00:00:00:00:00 | ::1/128                      | v6      | 32841 |
+-------+-------------------+------------------------------+---------+-------+
| lo0   | 00:00:00:00:00:00 | fe80::1/64                   | v6      | 32841 |
+-------+-------------------+------------------------------+---------+-------+
| en0   | ff:ff:ff:ff:ff:ff | fe80::f7:d0fd:2a0c:8c99/64   | v6      | 34915 |
+-------+-------------------+------------------------------+---------+-------+
| en0   | ff:ff:ff:ff:ff:ff | 192.168.0.101/24             | v4      | 34915 |
+-------+-------------------+------------------------------+---------+-------+
| awdl0 | ff:ff:ff:ff:ff:ff | fe80::43c:7cff:fe5c:9e73/64  | v6      | 35139 |
+-------+-------------------+------------------------------+---------+-------+
| utun0 | 00:00:00:00:00:00 | fe80::8217:4150:aeec:28b3/64 | v6      | 32849 |
+-------+-------------------+------------------------------+---------+-------+
| en5   | ff:ff:ff:ff:ff:ff | fe80::aede:48ff:fe00:1122/64 | v6      | 34915 |
+-------+-------------------+------------------------------+---------+-------+
```

### Expand a subnet
```
$ rips expand 192.168.1.4/30
+--------------+
| Children (4) |
+--------------+
| 192.168.1.4  |
+--------------+
| 192.168.1.5  |
+--------------+
| 192.168.1.6  |
+--------------+
| 192.168.1.7  |
+--------------+
```

### Check if the list of childen belong to a parent
```
$ rips belong -i 10.57.162.0/24 10.57.162.142/30 10.57.162.161 10.57.161.91
10.57.162.0/24
10.57.162.142/30
10.57.162.161

$ echo $?
1

$ rips belong 10.57.162.0/24 10.57.162.142/30 10.57.162.161
$ echo $?
0
```
