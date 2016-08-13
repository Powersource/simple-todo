use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    match list_content() {
        Ok(()) => println!("Listed contents successfully."),
        Err(e) => println!("Error listing contents: {:?}", e)
    }
}

fn list_content() -> io::Result<()> {
    //read file and print it
    let mut file = try!(File::open("todo.txt"));
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
