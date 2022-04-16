use colored::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file(file_name: &str) -> String {
    let file = File::open(format!("src/static/{}", file_name));
    let file: File = match file {
        Ok(f) => {
            println!(
                "{} {} {}",
                "file".yellow(),
                file_name.green(),
                "was found".yellow()
            );
            f
        }
        Err(_) => create_file(&file_name),
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).ok();
    contents
}

pub fn write_file(file_name: &str, new_line: &str) {
    let file = File::options()
        .append(true)
        .open(format!("src/static/{}", file_name));

    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            create_file(&file_name);
            File::options()
                .append(true)
                .open(format!("src/static/{}", file_name))
                .ok()
                .unwrap()
        }
    };

    file.write_all(format!("{}\n", new_line).as_bytes())
        .expect(format!("{}", "AN error occured when syncing the file".red()).as_str());
    file.sync_all()
        .expect(format!("{}", "AN error occured when syncing the file".red()).as_str());
}

fn create_file(file_name: &str) -> File {
    println!(
        "{} {} {}",
        "file".yellow(),
        file_name.green(),
        "was created".yellow()
    );
    let file = File::create(format!("src/static/{}", file_name)).ok();
    return file.unwrap();
}
