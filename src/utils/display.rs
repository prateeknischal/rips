extern crate prettytable;

use prettytable::{Attr, Cell, Row, Table};

pub fn display(headers: &Vec<String>, rows: &Vec<Vec<String>>, raw: bool) {
    if raw {
        for row in rows {
            for cell in row {
                print!("{}\t", cell);
            }
            println!("");
        }
        return;
    }
    let mut table = Table::new();

    let mut th = Vec::new();
    for header in headers {
        th.push(Cell::new(header.as_str()).with_style(Attr::Bold));
    }

    table.add_row(Row::new(th));

    for row in rows {
        let mut tr = Vec::new();
        for cell in row {
            tr.push(Cell::new(cell.as_str()));
        }

        table.add_row(Row::new(tr));
    }
    table.printstd();
}
