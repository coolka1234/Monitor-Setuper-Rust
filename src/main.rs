use std::process::{Child, Command, Output, Stdio};

fn main() {

}

fn get_monitor_ids()->String{
    let xrandr_query=Command::new("/usr/bin/xrandr")
        .arg("-q")
        .spawn()
        .expect("Failed to execute xrandr query. Make sure you have an up to date xrandr installation.");
    // let result= match xrandr_query {
    //     Ok(child) => child,
    //     Err(_) => Child() 
    // };
    let placeholde=    String::from("placeholder");
    return placeholde;
}