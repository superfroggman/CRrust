use std::process::Command;

fn main() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo hello Windows!"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello not windows!")
                .output()
                .expect("failed to execute process")
    };
    
    let hello = output.stdout;

    println!("{:?}", hello);
}