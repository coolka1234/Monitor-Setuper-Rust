use std::process::{Child, Command, Output, Stdio};

fn main() {
    print!("{}",get_monitor_ids())
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
    let Output{stdout,..}=xrandr_query.wait_with_output().unwrap();
    let xrandr_output=String::from_utf8_lossy(&stdout);
    return xrandr_output.to_string();
}