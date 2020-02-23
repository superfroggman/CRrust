use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn main(){
    let args: Vec<_> = env::args().collect();
    let fexten=extensionname(&args[1]);
    println!("{:?}",fexten);
}
fn extensionname(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}