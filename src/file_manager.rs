use colored::*;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file(file_name: &str) -> String {
    let file = File::open(format!("src/static/{}", file_name));
    let file: File = match file {
        Ok(f) => f,
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
        .expect(format!("{}", "An error occured when syncing the file".red()).as_str());
    file.sync_all()
        .expect(format!("{}", "An error occured when syncing the file".red()).as_str());
}

fn create_file(file_name: &str) -> File {
    println!(
        "{} {} {}",
        "file".yellow(),
        file_name.green(),
        "was created".yellow()
    );
    let file = File::create(format!("src/static/{}", file_name)).ok();
    return file.expect("Failed to create file");
}

#[allow(dead_code)]
pub fn remove_file(file_name: &str) -> std::io::Result<()> {
    fs::remove_file(format!("src/static/{}", file_name))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::file_manager;

    #[test]
    fn reading_non_existing_file_makes_new_empty_file() {
        let file_name = "test_file1.txt";
        let content = file_manager::read_file(file_name);
        assert_eq!(content.is_empty(), true);
        let remove_result = file_manager::remove_file(file_name);
        assert_eq!(remove_result.is_ok(), true);
    }
    #[test]
    fn writing_to_file_stores_content() {
        let file_name = "test_file2.txt";
        let content = "This test works";
        file_manager::write_file(file_name, content);
        let expected_content = file_manager::read_file(file_name);
        assert_eq!(format!("{}\n", content), expected_content);
        let remove_result = file_manager::remove_file(file_name);
        assert_eq!(remove_result.is_ok(), true);
    }
    #[test]
    #[should_panic]
    fn writing_to_file_with_incorrect_name_panics() {
        file_manager::write_file("/", "this should not work");
    }
    #[test]
    fn reading_to_file_with_incorrect_name_should_return_empty_content() {
        let contents = file_manager::read_file("/");
        assert_eq!(contents.is_empty(), true)
    }
}