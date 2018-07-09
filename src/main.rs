// imports {{{
// macros
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate duct;
#[macro_use]
extern crate lazy_static;
// regular
extern crate chrono;
extern crate clap;
extern crate maxminddb;
extern crate regex;
// usage
use chrono::prelude::*;
use clap::{App, Arg, SubCommand};
use duct::cmd;
use maxminddb::geoip2;
use regex::Regex;
use std::env;
use std::net::IpAddr;
use std::str::FromStr;
type MQ = (i32, String, String);
// use prettytable::row::Row;
// use prettytable::cell::Cell;
// }}}
// vec of strings macro {{{
// macro_rules! vec_of_strings {
//     ($($x:expr),*) => (vec![$($x.to_string()),*]);
// }
// }}}
// regex {{{
lazy_static! {
    pub static ref REG: Regex = Regex::new(r#"(\d{1,}) (\w{1,}).*<(.*\.\w{1,})"#).unwrap();
}
// }}}
fn main() {
    // app {{{
    let mut mailq = Vec::new();
    let matches = App::new("MyApp")
        .version("1.0")
        .author("ice")
        .about("psp cli")
        .arg(Arg::with_name("input").help("function to exec").index(1))
        .arg(
            Arg::with_name("ip")
                .short("i")
                .long("ip")
                .value_name("email")
                .takes_value(true)
                .help("get ip"),
        )
        .subcommand(
            SubCommand::with_name("a5")
                .about("amout05")
                .arg(Arg::with_name("mqid").help("check mail info by mailq id").index(1)),
        )
        .subcommand(
            SubCommand::with_name("a7")
                .about("amout07")
                .arg(Arg::with_name("mqid").help("check mail info by mailq id").index(1)),
        )
        .get_matches();
    // }}}
    // subcommands {{{
    if let Some(matches) = matches.subcommand_matches("a5") {
        // a5 ran
        println!("amout5 subcommand");
        ssh(
            &mut mailq,
            dotenv!("U3").to_string(),
            dotenv!("P2").to_string(),
            r"216.230.254.45".to_string(),
        );
        // emails args supplied
        // let mut full = &mailq;
        // println!("full: {:?}", full);
        if let Some(i) = matches.value_of("mqid") {
            println!("subcommand args: {}", &i);
            getip(
                &mailq,
                i.to_string(),
                dotenv!("U3").to_string(),
                dotenv!("P2").to_string(),
                r"216.230.254.45".to_string(),
            );

        }
    }
    if let Some(matches) = matches.subcommand_matches("a7") {
        // a7 ran
        println!("amout7 subcommand");
        // emails args supplied
        // let mut full = &mailq;
        // println!("full: {:?}", full);
        ssh(
            &mut mailq,
            dotenv!("U3").to_string(),
            dotenv!("P2").to_string(),
            r"216.230.254.47".to_string(),
        );
        if let Some(i) = matches.value_of("mqid") {
            getip(
                &mailq,
                i.to_string(),
                dotenv!("U3").to_string(),
                dotenv!("P2").to_string(),
                r"216.230.254.47".to_string(),
            );
            println!("subcommand args: {}", i);
        }
    }
    // }}}
    // match {{{
    if let Some(o) = matches.value_of("input") {
        println!("INPUT: {}", o);
        match o {
            // "ao5" => ssh(
            //     dotenv!("U3").to_string(),
            //     dotenv!("P2").to_string(),
            //     r"216.230.254.45".to_string(),
            // ),
            // "ao6" => ssh(
            //     dotenv!("U3").to_string(),
            //     dotenv!("P2").to_string(),
            //     r"216.230.254.46".to_string(),
            // ),
            // "ao7" => ssh(
            //     dotenv!("U3").to_string(),
            //     dotenv!("P2").to_string(),
            //     r"216.230.254.47".to_string(),
            // ),
            // "ao8" => ssh(
            //     dotenv!("U3").to_string(),
            //     dotenv!("P2").to_string(),
            //     r"216.230.254.48".to_string(),
            // ),
            "ip" => ip(),
            "b" => build(),
            _ => println!("0"),
        }
    }
    // }}}
}
// functions
// build {{{
fn build() {
    let utc: DateTime<Utc> = Utc::now();
    println!("{}", utc.to_string());
    let p = env::current_dir().unwrap();
    println!("The current directory is {}", p.display());
    let cmd = format!("cd {}/m/psp;cargo build", dotenv!("HOME"));
    cmd!("sh", "-c", cmd).run().unwrap();
}
// }}}
// ssh {{{
fn ssh(mailq: &mut Vec<(MQ)>, user: String, pass: String, ip: String) {
    let command = format!("spawn ssh {}@{};expect \"password\";send \"{}\n\";expect \"root\";send \"mailq | grep \\$(date | awk \\'{{print \\$2}}\\') | sort -k 6 -r | sort -k 7 -r | uniq -c -f6 | sort -k 1 | awk \\'{{print \\$1,\\$2,\\$8}}\\'\\n\";expect \"root\";send \"exit\\n\";expect eof;exit", user, ip, pass);
    // println!("{}", command);
    let args = &["-c", command.as_str()];
    let stdout = cmd("expect", args).read().unwrap();
    // println!("{}", stdout);
    // println!("{:?}", stdout);
    for cap in REG.captures_iter(&stdout) {
        let count = cap[1].parse::<i32>().unwrap();
        let id = cap[2].to_string();
        let email = cap[3].to_string();
        mailq.push((count, id, email));
    }
    let sorted = mailq;
    // sort by count
    sorted.sort_by_key(|x| x.0);
    // print each element
    sorted.iter().for_each(|x| println!("{} {} {}", x.0.to_string(), x.1, x.2))
}
// }}}
// getip {{{
fn getip(mailq: &Vec<(MQ)>, input: String, user: String, pass: String, ip: String) {
    let command = format!("spawn ssh {}@{};expect \"password\";send \"{}\n\";expect \"root\";send \"grep -oP \\'\\\\\\[\\\\K(\\[^\\]\\]+)|by \\\\K(\\[^\\.\\]+)|From: \\\\K(\\[^\\\\n\\]+)|Subject: \\\\K(\\.{{0,40}})\\' /var/spool/mqueue/qf{} | tail -4\\n\";expect \"root\";send \"exit\\n\";expect eof;exit", user, ip, pass, input);
    println!("{}", mailq[0].1);
    let args = &["-c", command.as_str()];
    let stdout = cmd("expect", args).read().unwrap();
    println!("{}", stdout);
}
// }}}
// ip {{{
fn ip() {
    let reader = maxminddb::Reader::open(
        "/home/fish/Downloads/GeoLite2-Country_20180605/GeoLite2-Country.mmdb",
    ).unwrap();
    let ip: IpAddr = FromStr::from_str("27.83.236.133").unwrap();
    let country: geoip2::Country = reader.lookup(ip).unwrap();
    print!("{:?}\n", country.country.unwrap().iso_code.unwrap());
}
// }}}
// notes: {{{
// awk
// https://stackoverflow.com/questions/1915636/is-there-a-way-to-uniq-by-column
// https://unix.stackexchange.com/questions/204747/get-or-filter-duplicated-lines-by-column
// https://stackoverflow.com/questions/24014194/how-to-grep-the-last-occurrence-of-the-line-pattern
// https://unix.stackexchange.com/questions/59893/grep-lines-starting-with-1-in-ubuntu

// sort by time then name
// mailq | grep $(date | awk '{print $2}') | sort -k 6 -r | sort -k 7 -r
//
// get last unique mailq id:
// mailq | grep $(date | awk '{print $2}') | awk '!seen[$7]++'
// FINAL VERSION:
// mailq | grep $(date | awk '{print $2}') | sort -k 6 -r | sort -k 7 -r | uniq -c -f6 | sort -k 1 | awk '{print $1,$2,$8}'

// grep for ip
// grep -Po '^H\?\?Received.*' /var/spool/mqueue/qfw5JFKFOG026685 | tail -1
// FINAL GET IP:
//echo -n 'yo' && grep -Po 'Received.*' /var/spool/mqueue/qfw5Q1bTLI044854 | tail -1 | grep -Po '\[(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\]'
// grep -Po 'Received.*' /var/spool/mqueue/qfw5QAqD38270223 | tail -1 | grep -Po '\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}'
// }}}
