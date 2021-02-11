use std::time::{Duration};
use std::thread::sleep;
use std::process::Command;
use std::{fs, thread, env};

fn take_screenshot(name:String){

    let mut screenshot_command = Command::new("screencapture");
    screenshot_command.arg("-x").arg(name);
    screenshot_command.output().expect("failed to execute process");
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let folder_name = args[1].clone().to_string();
    let number_of_screenshots: i64 = args[2].clone().parse().unwrap();
    let interval: i64 = args[3].clone().parse().unwrap();

    match fs::create_dir_all(folder_name.clone()){
        Ok(_) => {}
        Err(_) => {panic!("An Error occurred")}
    }

    for loop_number in 0..number_of_screenshots {
        let name = folder_name.clone().to_string() + "/screenshot_" + &*loop_number.clone().to_string() + ".png";
        thread::spawn(|| { take_screenshot(name); });
        sleep(Duration::new(interval as u64, 0));
    }
}