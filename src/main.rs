use std::{net::{TcpListener, TcpStream, SocketAddr}, io::{Read, Write}, ops::Index, hash::Hash};
use std::{str, env, fs, os};
use std::collections::HashMap;
use serde_json::Value;

mod tools;


fn startListener(configData: HashMap<String, Value>) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    while true {
        match listener.accept() {
            Ok((connection, addr)) => handleClient(connection, addr, &configData),
            Err(e) => println!("Couldn't get client: {e:?}"),
        }
    }
}


fn handleClient(mut connection: TcpStream, addr: SocketAddr, configData: &HashMap<String, Value>) {
    // println!("New client connected: {addr:?}"); 
    let mut buf = [0; 1024];
    connection.read(&mut buf);
    let (requestDetails, parameters) = getRequestDetails(&buf);
    if requestDetails.contains_key("GET") {
        tools::returnWebpage(requestDetails, &mut connection);
    } else if requestDetails.contains_key("POST") {
        tools::handlePostRequest(requestDetails, parameters, configData, &mut connection)
    
    }
}





fn splitUpRequestDetail(detail: String, separator: &str) -> (String, String) {
    let firstSpace = detail.find(separator).expect(&format!("Invalid request sent by browser {}", detail)[..]);
    let mut firstHalf = detail[..firstSpace].replace(":", "");
    let secondHalf = detail[firstSpace..].trim().replace("\r", "");
    return (firstHalf, secondHalf);
}


fn getRequestDetails(buf: &[u8]) -> (HashMap<String, String>, HashMap<String, String>) {
    let mut foundOther = false;
    for item in buf {
        if item != &0u8 {
            foundOther = true;
        }
    }
    let mut requestDetails = HashMap::new();
    let mut parameters = HashMap::new();
    let mut stringRequest = str::from_utf8(&buf).unwrap();
    if stringRequest.trim().replace("\r", "").is_empty() || !foundOther{
        return (requestDetails, parameters);
    }
    for line in stringRequest.split("\n") {
        if line.contains(" ") {
            let (firstHalf, secondHalf) = splitUpRequestDetail(line.to_string(), " ");
            requestDetails.insert(firstHalf.clone(), secondHalf);
        } else if line.contains("=") {
            for line in line.split("&") {
                let (firstHalf, secondHalf) = splitUpRequestDetail(line.to_string(), "=");
                parameters.insert(firstHalf.clone(), secondHalf.replace("\0", "")[1..].to_string());    
            }
        }
    }
    return (requestDetails, parameters);
}

fn main() {
    println!("Starting listener...");
    let configData = tools::getConfigData();
    startListener(configData);
}