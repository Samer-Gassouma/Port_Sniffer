use std::env;
use std::net::IpAddr;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::io;
use std::io::Write;

const MAX: u16 = 65535;
struct Arugments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arugments {
    fn new(args: &[String]) -> Result<Arugments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let flag = args[1].clone();
        if flag != "-h" && flag != "-x" && flag != "-j" {
            return Err("invalid flag");
        }
        if flag == "-h" {
            print_help();
            std::process::exit(0);
        }
        if flag == "-x" && args.len() != 3 {
            return Err("not enough arguments");
        }
        if flag == "-x" && args.len() == 3 {
            let ipaddr = match args[2].parse() {
                Ok(s) => s,
                Err(_) => return Err("invalid ip address"),
            };
            return Ok(Arugments { flag, ipaddr, threads: 1000 });
        }

        if flag == "-j" && args.len() != 4 {
            return Err("not enough arguments");
        }

        if flag == "-j" && args.len() == 4 {
            let threads = match args[2].parse() {
                Ok(s) => s,
                Err(_) => return Err("invalid number of threads"),
            };
            let ipaddr = match args[3].parse() {
                Ok(s) => s,
                Err(_) => return Err("invalid ip address"),
            };
            return Ok(Arugments { flag, ipaddr, threads });
        }
        Err("an error occurred")
    }
}

fn print_help() {
    println!("Usage: -j to select how many threads you want to use");
    println!("Usage: -h to show this help message");
    println!("Usage: -x to scan all ports on the ip address");
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        println!("{}" ,port);
        match std::net::TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!("Port {} is open", port);
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if (MAX - port) <= num_threads  {
            break;
        }
        port += num_threads;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arugments::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out {
        println!("Port {} is open", v);
    }
}
