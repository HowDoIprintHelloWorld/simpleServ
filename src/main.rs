use std::{net::{TcpListener, TcpStream, SocketAddr}, io::{Read, Write}, ops::Index, hash::Hash};
use std::{str, env, fs, os};
use std::collections::HashMap;

mod tools;


fn startListener(configData: HashMap<String, String>) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    while true {
        match listener.accept() {
            Ok((connection, addr)) => handleClient(connection, addr, &configData),
            Err(e) => println!("Couldn't get client: {e:?}"),
        }
    }
}


fn handleClient(mut connection: TcpStream, addr: SocketAddr, configData: &HashMap<String, String>) {
    // println!("New client connected: {addr:?}"); 
    let mut buf = [0; 1024];
    connection.read(&mut buf);
    let (requestDetails, parameters) = getRequestDetails(&buf);
    if requestDetails.contains_key("GET") {
        returnWebpage(requestDetails, &mut connection, addr);
    } else if requestDetails.contains_key("POST") {
        println!("{:?}", parameters);
    }
}


fn getContent(path: &String) -> Vec<u8> {
    let possibleContents = getPossibleContents();
    println!("{:?}", possibleContents);
    let path = if path == "/" {"index.html".to_string()} else {path.clone().to_string()[1..].to_string()};
    if !possibleContents.contains(&path) {
        return "<h1>404 - Page not found<h1>".as_bytes().to_vec();
    }
    let content = fs::read("content/".to_string()+(&path)).expect(&format!("Could not read file for path {path}")[..]);
    return content
}


fn getPossibleContents() -> Vec<String> {
    let mut possibleContents = fs::read_dir("content/").unwrap().filter_map(|x| x.ok()).map(|e| e.path().to_string_lossy().into_owned()).collect::<Vec<String>>();
    return possibleContents.iter().map(|x| x.replace("content/","")).collect()
}


fn getPage(requestDetails: HashMap<String, String>) -> String{
    let mut page = String::new();
    match requestDetails.get("GET") {
        Some(pageSome) => {page = pageSome.clone();},
        None => return String::new(),
    }
    page.split(" ").collect::<Vec<&str>>()[0].to_string()
}


fn returnWebpage(requestDetails: HashMap<String, String>, connection: &mut TcpStream, addr: SocketAddr) {
    let page = getPage(requestDetails);
    let statusLine = "HTTP/1.1 200 OK";
    let mut contents = getContent(&page);
    let contentsLength = contents.len();
    println!("Client {addr} requested: {page}");
    let msg = format!("{statusLine}\r\nContent-Length: {contentsLength}\r\n\r\n");
    let mut msg = msg.as_bytes().to_vec();
    msg.append(&mut contents);
    connection.write_all(&msg[..]).unwrap();
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