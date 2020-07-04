use std::{io, process};

fn get_files() -> io::Result<String> {
    let output = if cfg!(windows) {
        process::Command::new("cmd.exe")
            .args(&["/C", "dir /B"])
            .output()?
    } else {
        process::Command::new("/bin/ls").arg("-A").output()?
    };
    let file_list = String::from_utf8(output.stdout).unwrap();
    Ok(file_list)
}

// fn converter(file_name: String) {}

fn main() {
    let files = get_files().unwrap();
    println!("{:?}", files);
}
