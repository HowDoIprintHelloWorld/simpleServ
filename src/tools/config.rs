use std::collections::HashMap;
use std::{fs, env, process, str};
use serde_json;


fn getConfigFileContents() -> String {
    let mut configFileContents = String::new();
    let args = env::args().collect::<Vec<String>>();
    let fileLocation = match args.get(1) {
        Some(arg) => {arg.to_string()},
        None => {"".to_string()},
    };
    println!("{}", fileLocation);
    let file = fs::read_to_string(fileLocation);
    let fileContentUtf8 = match file {
      Ok(fileContent) => {fileContent},
      Err(e) => {panic!("{}", format!("Error reading config file: {:?}", e))}
    };
    let fileContent = serde_json::from_str(&fileContentUtf8[..]);
    println!("Content {:?}", fileContent);
    return configFileContents;
}


pub fn getConfigData() -> HashMap<String, String> {
    let mut configData = HashMap::new();
    let configFileContents = getConfigFileContents();
    return configData;
}