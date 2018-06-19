#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate duct;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate clap;
extern crate maxminddb;
extern crate regex;

use chrono::prelude::*;
use clap::{App, Arg};
use duct::cmd;
use maxminddb::geoip2;
use regex::Regex;
use std::env;
use std::net::IpAddr;
use std::str::FromStr;

// macro_rules! vec_of_strings {
//     ($($x:expr),*) => (vec![$($x.to_string()),*]);
// }
lazy_static! {
    pub static ref REG: Regex = Regex::new(r#" (\d{1,}) .*(<.*>)"#).unwrap();
    // pub static ref REIP: Regex = Regex::new(r#"(\b\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}\b)\]\)\r\n\tby (ltsub01.alpha-mail.net|ltsub02.alpha-mail.net|amsub01.alpha-mail.net|amsub02.alpha-mail.net)"#).unwrap();
}

fn main() {
    let matches = App::new("MyApp")
        .version("1.0")
        .author("ice")
        .about("psp cli")
        .arg(Arg::with_name("input").help("function to exec").index(1))
        .get_matches();

    if let Some(o) = matches.value_of("input") {
        println!("INPUT: {}", o);
        match o {
            "ao5" => ssh(
                dotenv!("U3").to_string(),
                dotenv!("P2").to_string(),
                r"216.230.254.45".to_string(),
            ),
            "ao6" => ssh(
                dotenv!("U3").to_string(),
                dotenv!("P2").to_string(),
                r"216.230.254.46".to_string(),
            ),
            "ao7" => ssh(
                dotenv!("U3").to_string(),
                dotenv!("P2").to_string(),
                r"216.230.254.47".to_string(),
            ),
            "ao8" => ssh(
                dotenv!("U3").to_string(),
                dotenv!("P2").to_string(),
                r"216.230.254.48".to_string(),
            ),
            "ip" => ip(),
            "b" => build(),
            _ => println!("0"),
        }
    }
}

fn build() {
    let utc: DateTime<Utc> = Utc::now();
    println!("{}", utc.to_string());
    let p = env::current_dir().unwrap();
    println!("The current directory is {}", p.display());
    let cmd = format!("cd {}/m/psp;cargo build", dotenv!("HOME"));
    cmd!("sh", "-c", cmd).run().unwrap();
}

fn ssh(user: String, pass: String, ip: String) {
    let mut mailq = Vec::new();
    let command = format!("spawn ssh {}@{};expect \"password\";send \"{}\n\";expect \"root\";send \"mailq | grep \\$(date | awk \\'{{print \\$2}}\\') | awk \\'{{print \\$7}}\\' | sort | uniq -c | sort -n\\n\";expect \"root\";send \"exit\\n\";expect eof;exit", user, ip, pass);
    // println!("{}", command);
    let args = &["-c", command.as_str()];
    let stdout = cmd("expect", args).read().unwrap();
    // println!("{}", stdout);
    // println!("{:?}", stdout);
    for cap in REG.captures_iter(&stdout) {
        let msg = cap[2].to_string().replace("<", "").replace(">", "");
        let email = cap[1].to_string();
        mailq.push((email, msg));
    }
    println!("{:?}", mailq);
}

fn ip() {
    let reader = maxminddb::Reader::open(
        "/home/fish/Downloads/GeoLite2-Country_20180605/GeoLite2-Country.mmdb",
    ).unwrap();
    let ip: IpAddr = FromStr::from_str("27.83.236.133").unwrap();
    let country: geoip2::Country = reader.lookup(ip).unwrap();
    print!("{:?}\n", country.country.unwrap().iso_code.unwrap());
}


// sort by time
// mailq | grep $(date | awk '{print $2}') | sort -k 6
// mailq | grep $(date | awk '{print $2}') | sort -k 6 | sort -k 7
//awk -F, 'p!=$13 && p0 {print p0} {p=$13; p0=$0} END{print p0}' file

// get unique mailq id:
// mailq | grep $(date | awk '{print $2}') | awk '!seen[$7]++'

// example output:
// w5JEIDrg022952-    3369 Tue Jun 19 23:18 MAILER-DAEMON
// w5JEOG5n023310-      59 Tue Jun 19 23:24 <saito@escalader.co.jp>
// w5JE5O8L022147-      46 Tue Jun 19 23:05 <ishida@ntecweb.co.jp>
// w5JDj3Au021117       55 Tue Jun 19 22:45 <ohta-m@cortina.co.jp>
// w5JDvtFG021606-      49 Tue Jun 19 22:57 <takeyasu@suikan.co.jp>
// w5JDfjXF020986       41 Tue Jun 19 22:41 <takahiro.tagawa@mitsushima.co.jp>
// w5JDvfSP021570-      48 Tue Jun 19 22:57 <hirokazu.nishimura@mitsushima.co.jp>
// w5JE2hSo022014-      57 Tue Jun 19 23:02 <tomohisa.takase@mitsushima.co.jp>
// w5J8OJGM411748-    4118 Tue Jun 19 17:24 <nichizo@nichizo.co.jp>
// w5J7MCX9383350      802 Tue Jun 19 16:22 <info-satokon@sato-konpo.co.jp>
