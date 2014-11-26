#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate postgres;

use postgres::{Connection, SslMode};
use regex::{Regex};
use std::io::{BufferedReader, File, IoResult, TypeFile};
use std::io::fs::{walk_dir, stat};
use std::result::Result;

fn put_line(conn: &Connection, server: String, datetime: String, loglevel: String, message: String) -> Result<(), String> {

    conn.execute("INSERT INTO LOGLINE1 (server, datetime, loglevel, message) VALUES ($1, $2, $3, $4)",
                    &[&server, &datetime, &loglevel, &message]);
    Ok(())
}

fn is_file(path: &Path) -> IoResult<bool> {
    stat(path).map(|fs| fs.kind == TypeFile)
}

struct Line {
    server:     String,
    datetime:   String,
    loglevel:   String,
    message:    String,
}

fn start_archive_task(rx: Receiver<Line>) {
    spawn(proc() {
        let conn = Connection::connect("postgres://slurp:slurp@localhost/slurp", &SslMode::None).unwrap();

        loop {
            let l = rx.recv();
            put_line(&conn, l.server, l.datetime, l.loglevel, l.message);
        }
    });
}

fn main() {
    let path = Path::new(".");
    let (tx, rx): (Sender<Line>, Receiver<Line>) = channel();

    start_archive_task(rx);

    for e in walk_dir(&path).unwrap() {
        if is_file(&e).unwrap() && e.filename_str().unwrap_or("").starts_with("servicemix") {
            let tx1 = tx.clone();
            spawn(proc() {
                let re = regex!(r"(\d{4}-\d{2}-\d{2} \d{1,2}:\d{1,2}:\d{2},\d{3}) \| (\w+)\s*\| (.*)");
                let server = e.dirname_str().unwrap_or("unknown");
                let mut last_line : Option<String> = None;
                let mut f = BufferedReader::new(File::open(&e));

                for l in f.lines() {
                    let l1 = l.unwrap_or(String::from_str(""));
                    if l1.starts_with("2014") {
                        if last_line.is_some() {
                            match re.captures(l1.as_slice()) {
                                Some(c) =>
                                    tx1.send(Line {server:      String::from_str(server), 
                                                   datetime:    String::from_str(c.at(1)),
                                                   loglevel:    String::from_str(c.at(2)),
                                                   message:     String::from_str(c.at(3)),
                                                  }),
                                None => println!("Line {} doesn't match", last_line),
                            }
                        }
                        last_line = Some(l1);
                    } else {
                        last_line = last_line.map(|ll| ll + l1);
                    }
                }
            });
        }
    }
}
