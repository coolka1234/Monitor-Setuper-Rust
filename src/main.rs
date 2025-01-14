use std::process::{Child, Command, Output, Stdio};

fn main() {
    let entire_xrandr=get_monitor_ids();
    print!("Entire output: {}",entire_xrandr);
    print!("Tag lines: {}", find_tags_of_monitors(entire_xrandr));
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

fn find_tags_of_monitors(input: String)->String{
    let greped_lines = Command::new("/usr/bin/grep")
        .arg("-w")
        .arg(input)
        .spawn()
        .expect("Failed to execute grep query.
        Make sure you have an up to date grep command.");
    let Output{stdout,..}=greped_lines.wait_with_output().unwrap();
    let tag_lines=String::from_utf8_lossy(&stdout);
    return tag_lines.to_string();

}