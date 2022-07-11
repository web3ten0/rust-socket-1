use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("invalid args");
        std::process::exit(1);
    }
    let protocol: &str = &args[1]; // tcp | udp
    let role: &str = &args[2]; // server | client
    let address: &str = &args[3]; // addr:port
    println!("{} {} {}", protocol, role, address);
    let _ = match protocol {
        "tcp" => match role {
            "server" => tcp_server::serve(address),
            "client" => Ok(()),
            _ => Ok(()),
        },
        "udp" => match role {
            "server" => Ok(()),
            "client" => Ok(()),
            _ => Ok(()),
        },
        _ => Ok(()),
    };
}
