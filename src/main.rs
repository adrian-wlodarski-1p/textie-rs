mod input;
use input::Input;
type Error = Box<dyn std::error::Error>;

// std::num::ParseIntError
fn apply_to_lines(text: &mut str, fun: impl Fn(&mut Vec<&str>, index) -> ()) -> Result<String, std::num::ParseIntError> {
	let mut lines: Vec<&str> = text.split("\n").collect();
	if index.parse()? < lines.len() {
		fun(&mut lines);
	}
	Ok(String::from(lines.join("\n")))
}

fn editor_loop() /*-> Result<(), Box<dyn Error>>*/ {
	let mut input = Input::new();
	let mut text = String::new();
	loop {
		let mut line = String::new();
		input.get_line(&mut line, ": ").unwrap();
		
		let iter = line.split(' ');
		let (com, arg) = iter.next(), iter.next();
		
		match (com, arg) {
			(Some("?clean"), _) => { text = String::new(); },
			(Some("?delete"), index) => {
				apply_to_lines(&mut text, |lns| lns.delete(index), index);
			}
			(Some("?edit"), index) => {
				apply_to_lines(&mut text, |lns| lns[index] = input.edit_line("Edit line: ", lns[index]), index);
			}
			(Some("?replace"), index) => {
				apply_to_lines(&mut text, |lns| lns[index] = input.get_line("Replace with: "), index);
			}
			(Some(x), _) => { text.append(x); }
			(None, _) => ();
		}

		text.push_str(&line);
	}
}

fn main() {
	editor_loop();
}
