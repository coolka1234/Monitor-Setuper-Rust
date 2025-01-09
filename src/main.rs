use std::process::{Child, Command, Output, Stdio};

fn main() {

}

fn get_monitor_ids(){
    let xrandr_query=Command::new("/usr/bin/xrandr")
        .arg("-q")
        .spawn();
    let result= match xrandr_query {
        Ok(Child) => Child,
        Err(_) => " " 
    };
}