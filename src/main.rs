use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough Arguments");
        } else if args.len() > 4 {
            return Err("too mant arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments{flag: String::from(""), ipaddr, threads: 4})
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many thready do you want
                \r\n    -h or -help  to show this help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j"){
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IP Address, must be IPv4 or IPv6 ")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Failed to pass thread number"),
                };
                return Ok(Arguments{threads,flag,ipaddr});
            } else {
                return Err("invalid Syntax");
            }
        }
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsring arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let number_threads = arguments.threads;
    let (tx, rx) = channel();
    for i in 0..number_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, number_threads);
        });
    }

}