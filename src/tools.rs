use std::{collections::HashMap, net::TcpStream, hash::Hash};
use serde_json::Value;

pub mod config;
pub mod returnWebpage;
pub mod handlePost;
pub mod logins;


pub fn getConfigData() -> HashMap<String, Value> {
    return config::getConfigData();
}

pub fn returnWebpage(requestDetails: HashMap<String, String>, connection: &mut TcpStream) {
    returnWebpage::returnWebpage(requestDetails, connection)
}


pub fn handlePostRequest(requestDetails: HashMap<String, String>, arguments: HashMap<String, String>, configData: &HashMap<String, Value>, connection: &mut TcpStream) {
    handlePost::handleLogins(returnWebpage::getPage(requestDetails.clone()), requestDetails, arguments, configData.clone());
}