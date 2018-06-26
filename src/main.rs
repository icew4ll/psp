// imports {{{
// macros
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate duct;
#[macro_use]
extern crate lazy_static;
// regular
#[macro_use]
extern crate prettytable;
extern crate chrono;
extern crate clap;
extern crate maxminddb;
extern crate regex;
// usage
use chrono::prelude::*;
use clap::{App, Arg, SubCommand};
use duct::cmd;
use maxminddb::geoip2;
use prettytable::Table;
use regex::Regex;
use std::env;
use std::net::IpAddr;
use std::str::FromStr;
type MQ = (String, String, String);
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
                .arg(Arg::with_name("emails").help("check email ips").index(1)),
        )
        .subcommand(
            SubCommand::with_name("a6")
                .about("amout06")
                .arg(Arg::with_name("emails").help("check email ips").index(1)),
        )
        .subcommand(
            SubCommand::with_name("a7")
                .about("amout07")
                .arg(Arg::with_name("emails").help("check email ips").index(1)),
        )
        .subcommand(
            SubCommand::with_name("a8")
                .about("amout08")
                .arg(Arg::with_name("emails").help("check email ips").index(1)),
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
        if let Some(i) = matches.value_of("emails") {
            println!("subcommand args: {}", i);
            getip(
                &mailq,
                dotenv!("U3").to_string(),
                dotenv!("P2").to_string(),
                r"216.230.254.45".to_string(),
            );

        }
    }
    // if let Some(matches) = matches.subcommand_matches("a6") {
    //     println!("amout6 subcommand");
    //     ssh(
    //         dotenv!("U3").to_string(),
    //         dotenv!("P2").to_string(),
    //         r"216.230.254.46".to_string(),
    //     );
    //     if let Some(i) = matches.value_of("emails") {
    //         println!("Email args: {}", i);
    //     }
    // }
    // if let Some(matches) = matches.subcommand_matches("a7") {
    //     println!("amout7 subcommand");
    //     ssh(
    //         dotenv!("U3").to_string(),
    //         dotenv!("P2").to_string(),
    //         r"216.230.254.47".to_string(),
    //     );
    //     if let Some(i) = matches.value_of("emails") {
    //         println!("Email args: {}", i);
    //     }
    // }
    // if let Some(matches) = matches.subcommand_matches("a8") {
    //     println!("amout8 subcommand");
    //     ssh(
    //         dotenv!("U3").to_string(),
    //         dotenv!("P2").to_string(),
    //         r"216.230.254.4".to_string(),
    //     );
    //     if let Some(i) = matches.value_of("emails") {
    //         println!("Email args: {}", i);
    //     }
    // }
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
        let count = cap[1].to_string();
        let id = cap[2].to_string();
        let email = cap[3].to_string();
        mailq.push((count, id, email));
    }
    // println!("{:?}", mailq);
    // println!("{:?}", mailq[0].1);
    // iterate vec, return vec
    // let q = mailq.iter().map(|x| &x.1).collect::<Vec<_>>();
    // println!("{:?}", q);
    // print elements of vec with iter
    // mailq.iter().map(|x| &x.1).for_each(|x| println!("{:?}", x));

    // sort mailq on count
    let t2 = mailq;
    t2.sort_by(|a, b| a.cmp(b));
    // t2.sort_by_key(|x| &x.0);

    // table output
    let mut table = Table::new();
    // header
    table.add_row(row!["COUNT", "ID", "EMAILS"]);
    // iterate mailq create new row
    for x in t2 {
        table.add_row(row![x.0, x.1, x.2]);
    }
    // table.add_row(row![mailq[0].0, mailq[0].1, mailq[0].2]);
    // Print the table to stdout
    table.printstd();
}
// }}}
// getip {{{
fn getip(mailq: &Vec<(MQ)>, user: String, pass: String, ip: String) {
    // println!("{:?}{}{}{}", mailq, user, pass, ip);
    // let command = format!("spawn ssh {}@{};expect \"password\";send \"{}\n\";expect \"root\";send \"echo -n \\'{}\\' && grep -Po \\'Received.*\\' /var/spool/mqueue/qf{} | tail -1 | grep -Po \\'\\\\\\[\\\\d{{1,3}}\\\\.\\\\d{{1,3}}\\\\.\\\\d{{1,3}}\\\\.\\\\d{{1,3}}\\\\\\]\\'\n\";expect \"root\";send \"exit\\n\";expect eof;exit", user, ip, pass, mailq[0].2, mailq[0].1);
    // tac /var/spool/mqueue/qfw5QAqD38270223 | sed -n '/From:/{p;n;p;n;p;n;p;n}'
    // t2="$(tac /var/spool/mqueue/qfw5QAqD38270223 | sed -n '/From:/{p;n;p;n;p;n;p;n})"
    // t2="$(cat /var/spool/mqueue/qfw5QAqD38270223)" && echo $t2
    let command = format!("spawn ssh {}@{};expect \"password\";send \"{}\n\";expect \"root\";send \"echo -n \\'{}\\' && grep -Po \\'Received.*\\' /var/spool/mqueue/qf{} | tail -1 | grep -Po \\'\\\\\\[\\\\d{{1,3}}\\\\.\\\\d{{1,3}}\\\\.\\\\d{{1,3}}\\\\.\\\\d{{1,3}}\\\\\\]\\'\n\";expect \"root\";send \"exit\\n\";expect eof;exit", user, ip, pass, mailq[0].2, mailq[0].1);
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

// notes:
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
// H??Received: from miycaaykdt (unknown [177.66.59.207])
// get line matching brochure and print next 2 lines
// sed -n '/Received:/{p;n;p;n;p;n;p;n}' /var/spool/mqueue/qfw5QAqD38270223
// get line matching from and previous 3 lines
// tac /var/spool/mqueue/qfw5QAqD38270223 | sed -n '/From:/{p;n;p;n;p;n;p;n}'

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
// example stdout
// [("1", "w5Q1bTLI044854", "hira90-24@ando-che.co.jp"), ("1", "w5Q1O0uW038769", "tokiya@sato-zen.co.jp"), ("1", "w5Q2f3mn074021", "o-i@funabori.co.jp"), ("1", "w5Q5gc5Y147098", "t.nawa@beams-dc.co.jp"), ("1", "w5Q6WE9o172191", "libro@jei.or.jp"), ("1", "w5Q6xtgH183057", "s-hayatsu@adrs-s.co.jp"), ("1", "w5Q6xuHF183075", "hazama-r@njr.jyutaku.co.jp"), ("1", "w5Q71g8k183924", "n.ishige@shantery.co.jp"), ("1", "w5Q77DNc186402", "master002@cds-ito.co.jp"), ("1", "w5Q7DVmV188873", "katsurada@keioizumi.co.jp"), ("1", "w5Q7EDUq189104", "fcshienbu@reins.co.jp"), ("1", "w5Q7EOBc189155", "m-furuse@kawamoto-ind.co.jp"), ("1", "w5Q7EQhj189180", "user@b0nds.jp"), ("1", "w5Q7ERVX189187", "teramoto@okouchi.co.jp"), ("2", "w5Q7EMla189140", "utm.m@nifty.com")]
// example out
// H??Received: from ampri02.alpha-mail.net (unknown [216.230.254.13])
//         by amis06.alpha-mail.net (Postfix) with ESMTP;
//         Tue, 26 Jun 2018 19:52:13 +0900 (JST)
// H??Received: from oens05 (165-100-168-182.tokyo.otk.vectant.ne.jp [165.100.168.182])
//         by ampri02.alpha-mail.net (Alpha-mail) with ESMTP id D4D5780040;
//         Tue, 26 Jun 2018 19:52:12 +0900 (JST)
// H??From: =?iso-2022-jp?B?GyRCPi5FRCEhNTFDSxsoQg==?= <oda@oens.co.jp>
// another
// t2="$(cat /var/spool/mqueue/qfw5QAqD38270223)" && echo $t2
// V8 T1530010333 K1530027782 N11 P1082216 I253/2/524822 B7BIT MDeferred: Connection reset by mailgw.jec.ac.jp. Fbs $_webarc02 [216.230.254.82] $rESMTP $swebarc02.alpha-mail.jp ${daemon_flags} ${if_addr}216.230.254.45 S<oda@oens.co.jp> MDeferred: Connection reset by mailgw.jec.ac.jp. Qrfc822;17jn0315@jec.ac.jp rRFC822; 17jn0315@jec.ac.jp RPFD:<17jn0315@jec.ac.jp> H?P?Return-Path:V8 T1530010333 K1530027782 N11 P1082216 I253/2/524822 B7BIT MDeferred: Connection reset by mailgw.jec.ac.jp. Fbs $_webarc02 [216.230.254.82] $rESMTP $swebarc02.alpha-mail.jp ${daemon_flags} ${if_addr}216.230.254.45 S<oda@oens.co.jp> MDeferred: Connection reset by mailgw.jec.ac.jp. Qrfc822;17jn0315@jec.ac.jp rRFC822; 17jn0315@jec.ac.jp RPFD:<17jn0315@jec.ac.jp> H?P?Return-Path: <g> H??Received: from webarc02.alpha-mail.jp (webarc02 [216.230.254.82]) by amout05.alpha-mail.net with ESMTP id w5QAqD38270223; Tue, 26 Jun 2018 19:52:13 +0900 H??X-Virus-Scanned: amavisd-new at Alpha-Mail Out H??Received: from amis06.alpha-mail.net (amis06 [216.230.254.36]) by webarc02.alpha-mail.jp (Postfix) with ESMTP id F1F8A131004E; Tue, 26 Jun 2018 19:52:03 +0900 (JST) H??Reg> H??Received: from webarc02.alpha-mail.jp (webarc02 [216.230.254.82]) by amout05.alpha-mail.net with ESMTP id w5QAqD38270223; Tue, 26 Jun 2018 19:52:13 +0900 H??X-Virus-Scanned: amavisd-new at Alpha-Mail Out H??Received: from amis06.alpha-mail.net (amis06 [216.230.254.36]) by webarc02.alpha-mail.jp (Postfix) with ESMTP id F1F8A131004E; Tue, 26 Jun 2018 19:52:03 +0900 (JST) H??Received: from amis06.alpha-mail.net (unknown [127.0.0.1]) by IMSVA (Postfix) with ESMTP id 5F90F110042; Tue, 26 Jun 2018 19:52:13 +0900 (JST) H??Received: from amis06.alpha-mail.net (unknown [127.0.0.1]) by IMSVA (Postfix) with ESMTP id 53FA811003F; Tue, 26 Jun 2018 19:52:13 +0900 (JST) H??Received: from ampri02.alpha-mail.net (unknown [216.230.254.13]) by amis06.alpha-mail.net (Postfix) with ESMTP; Tue, 26 Jun 2018 19:52:13 +0900 (JST) H??Received: from oens05 (165-100-168-182.tokyo.otk.vectant.ne.jp [165.100.168.182]) by ampri02.alpha-mail.net (Alpha-mail) with ESMTP id D4D5780040; Tue, 26 Jun 2018 19:52:12 +0900 (JST) H??From: =?iso-2022-jp?B?GyRCPi5FRCEhNTFDSxsoQg==?= <oda@oens.co.jp> H??To: <17jn0315@jec.ac.jp> H??Cc: =?iso-2022-jp?B?GyRCPi5FRBsoQiAbJEJBbxsoQg==?= <oda_s@oens.co.jp>, <hagiwara_i@oens.co.jp> H??Subject: =?iso-2022-jp?B?GyRCJSohPCUoJXMlOUBiTEAycSRLJCoxWyQ3JCQkPyRAJC0bKEI=?= =?iso-2022-jp?B?GyRCJCIkaiQsJEgkJiQ0JDYkJCReJDckPxsoQg==?= H??Date: Tue, 26 Jun 2018 19:52:13 +0900 H??Message-ID: <005101d40d3b$bdb1e190$3915a4b0$@oens.co.jp> H??MIME-Version: 1.0 H??Content-Type: text/plain; charset="iso-2022-jp" H??Content-Transfer-Encoding: 7bit H??X-Mailer: Microsoft Outlook 16.0 H??Thread-Index: AdQNKG7wtZF4UKM5TIG7QuPftFZyzg== H??Content-Language: ja H??X-TM-AS-GCONF: 00 .
