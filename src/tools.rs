use std::collections::HashMap;

pub mod config;


pub fn getConfigData() -> HashMap<String, String> {
    return config::getConfigData();
}