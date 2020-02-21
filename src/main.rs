use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::thread;
use std::sync::mpsc::{Sender, channel};
use std::io::{self, Write};

const MAX: u16 = 65535;
#[derive(Debug)]
struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("缺少参数！");
        } else if args.len() > 4 {
            return Err("参数过多！");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr: ipaddr,
                threads: 20,
            });
        } else {
            if args.len() == 2 && f.contains("-h") || f.contains("-help") {
                println!("this is help info...");
            } else if f.contains("-h") || f.contains("-help") {
                println!("参数过多...");
            } else if f.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => {
                        return Err("params error[010]");
                    },
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => {
                        return Err("parse params threads error");
                    },
                };
                return Ok(Arguments{
                    flag: String::from(""),
                    ipaddr: ipaddr,
                    threads: threads,
                });
            }
            return Err("something is error[011]");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arguments: Arguments = Arguments::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(0);
    });
    let thread_num = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..thread_num {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, thread_num);
        });
    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    for v in out {
        println!("{} is open", v);
    }
}

/// 扫描端口
fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, thread_num: u16) {
    let mut port: u16 = start_port + 1;
    println!("current port is: {}", port);
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            },
            Err(_) => {
                println!("this port {} may not open", port);
            },
        }
        if MAX - port <= thread_num {
            break;
        }
        port += thread_num;
    }
}
