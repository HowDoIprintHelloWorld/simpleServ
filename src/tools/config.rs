use std::collections::HashMap;
use std::{fs, env, process, str, any};
use serde_json;
use serde_json::Value;


fn getConfigFileContents() -> String {
    let args = env::args().collect::<Vec<String>>();
    let fileLocation = match args.get(1) {
        Some(arg) => {arg.to_string()},
        None => {"".to_string()},
    };
    let file = fs::read_to_string(fileLocation);
    let configFileContents: String = match file {
      Ok(fileContent) => {fileContent},
      Err(e) => {panic!("{}", format!("Error reading config file: {:?}", e))}
    };
    return configFileContents;
}

fn configFileToJSON(configFileContents: String) -> HashMap<String, Value> {
    let fileContent: &str = &configFileContents[..];
    let fileContent = match serde_json::from_str::<HashMap<String, Value>>(fileContent) {
        Ok(fileContent) => {fileContent},
        Err(e) => {panic!("Error in config file: {}", e);}
    };
    
    return fileContent;
}


pub fn getConfigData() -> HashMap<String, Value> {
    let configData = configFileToJSON(getConfigFileContents());
    match configData.get("printConfig") {
        Some(state) => {if state == true {println!("{:?}", configData)}},
        None => {}
    }
    return configData;
}