use std::fs;
use std::io::Write;

fn main() {
    read_file();
}

fn read_file() {
    let file = std::fs::read_to_string(".env").expect("Failed to read .env file");

    write_env_js(&file);
}

fn write_env_js(file: &str) -> std::io::Result<()> {
    let mut output = fs::File::create("env-config.js")?;
    output.write_all("window._env_ = {".as_bytes())?;
    for line in file.lines() {
        let keyval = split_line(line);
        let write_line = format!("{}:\"{}\",",keyval.0,keyval.1);
        output.write_all(write_line.as_bytes())?;
    };
    output.write_all("}".as_bytes())?;
    Ok(())
}

fn split_line(line: &str) -> (&str, &str) {
    let mut split = line.split('=');
    (split.next().unwrap(), split.next().unwrap())
}