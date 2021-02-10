use std::time::{Duration};
use std::thread::sleep;
use std::process::Command;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};
use std::{fs, thread};

fn take_screenshot(name:String){

    let mut screenshot_command = Command::new("screencapture");
    screenshot_command.arg("-x").arg(name);
    screenshot_command.output().expect("failed to execute process");
}

fn main(){

    let s = fs::read_to_string("config.yaml").unwrap();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];
    let interval = doc["interval"].as_i64().unwrap();
    let location = doc["directory"].as_str().unwrap();
    let num_of_screenshots = doc["num_of_screenshots"].as_i64().unwrap();

    for loop_number in 0..num_of_screenshots{
        let name = location.clone().to_string() + "/screenshot_"+ &*loop_number.clone().to_string() +".png";
        thread::spawn(|| {take_screenshot(name);});
        sleep(Duration::new(interval as u64, 0));
    }

}