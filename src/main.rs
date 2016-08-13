fn main() {
    println!("Hello, world!");
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
