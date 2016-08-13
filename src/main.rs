use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::OpenOptions;

fn main() {
    match list_content() {
        Ok(()) => (),
        Err(e) => 
            match e.kind() {
                ErrorKind::NotFound => println!("todo.txt file not found"),
                _ => println!("Other error with listing")
            }
    }
}

fn list_content() -> io::Result<()> {
    //read file and print it
    let mut file = try!(
        OpenOptions::new()
        .read(true)
        .open("todo.txt"));
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => print!("{}", content),
        Err(e) => println!("Error reading file: {:?}", e)
    }
    Ok(())
}

pub fn hi() -> &'static str {
    return "hi";
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_test() {
		assert_eq!(hi(), "hi");
    }
}
