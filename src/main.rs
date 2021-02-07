use std::time::{Duration};
use std::thread::sleep;
use std::process::Command;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};
use std::fs;


fn main(){

    let s = fs::read_to_string("config.yaml").unwrap();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];
    let interval = doc["interval"].as_i64().unwrap();
    let location = doc["directory"].as_str().unwrap();
    let num_of_screenshots = doc["num_of_screenshots"].as_i64().unwrap();

    for x in 0..num_of_screenshots{
        let mut screenshot_command = Command::new("screencapture");
        screenshot_command.arg("-x").arg(location.clone().to_string() + "/screenshot_"+ &*x.clone().to_string() +".png");
        screenshot_command.output().expect("failed to execute process");
        sleep(Duration::new(interval as u64, 0));
    }

}