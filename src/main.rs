// disable dead code warnings
#![allow(dead_code)]
use server::Server;
use std::env;
use website_handler::WebsiteHandler;


mod http;
mod server;
mod website_handler;

fn main() {
    // TEST SETUP DEBUG CODE
    let ip_with_port_string = String::from("127.0.0.1:8080".to_string());
    let ip_address_string: String = get_ip_address_string(&ip_with_port_string);
    let port_string: String = get_port_string(&ip_with_port_string);
    dbg!(&ip_with_port_string);
    dbg!("IP: ",&ip_address_string);
    dbg!("PORT: ",&port_string);
    println!("IP: {}", &ip_address_string);
    println!("PORT: {}", &port_string);
    // END TEST SETUP DEBUG CODE

    let default_path = format!("{}{}public",  env!("CARGO_MANIFEST_DIR"), std::path::MAIN_SEPARATOR);
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new(ip_with_port_string);
    println!("HTTP SERVER APPLICATION!");
    println!("public path: {}", public_path);
    server.run(WebsiteHandler::new(public_path));
}

fn find_in_string(string: &String, del: char) -> usize {
    
    
    for i in string.chars().enumerate(){
        if i.1 == del {
            println!("{} found at index : {}", del, i.0);
            return i.0;
        }
        else {
            continue;
        }
    }
    println!("could not find {} in string. returning 0", del);
    return 0;
}

fn get_ip_address_string(string: &String) -> String {
    let index = find_in_string(&string, ':');
    let ip_address = string[0..index].to_string();
    return ip_address;
}

fn get_port_string (string: &String)-> String {
    let index = find_in_string(&string, ':') + 1;
    let port = string[index..].to_string();
    return port;
}
