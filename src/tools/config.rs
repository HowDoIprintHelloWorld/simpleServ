use std::collections::HashMap;
use std::{fs, env};


fn getConfigFileContents() -> String {
    let mut configFileContents = String::new();
    let args = env::args().collect::<Vec<String>>();
    let fileLocation = match args.get(0) {
        Some(arg) => {arg.to_string()},
        None => {"".to_string()},
    };
    fs::read("");

    return configFileContents;
}


pub fn getConfigData() -> HashMap<String, String> {
    let mut configData = HashMap::new();
    let configFileContents = getConfigFileContents();
    return configData;
}