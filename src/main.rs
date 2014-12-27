// Copyright (c) 2014 Toby DiPasquale <toby@cbcg.net>
#![feature(phase)]
#[phase(plugin,link)] extern crate log;
extern crate getopts;
extern crate smtpd;

use getopts::{optopt, optflag, getopts, OptGroup, HasArg};
use std::os;
use smtpd::config;
use smtpd::net;

fn main() {
    let args = os::args();
    let program = args[0].clone();
    let opts = &[
        optopt("f", "file", "path to configuration file", "FILE"),
        optflag("h", "help", "print this message"),
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    let config = match config::load_config(matches.opt_str("f")) {
        Ok(config) => { config }
        Err(e) => {
            error!("failed to load configuration from {}: {}", matches.opt_str("f").unwrap(), e);
            return;
        }
    };
    net::main_loop(config);
}

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]\n", program);
    for o in _opts.iter() {
        if o.hasarg == HasArg::Yes {
            println!("   -{},--{} {}\t{}", o.short_name, o.long_name, o.hint, o.desc);
        } else {
            println!("   -{},--{}\t\t{}", o.short_name, o.long_name, o.desc);
        }
    }
    println!("");
}

