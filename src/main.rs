mod input;
use input::{Input, InputError, GetLineErr, EditErr};

pub mod err_mes {
	/* error messages. */
	pub const inv_arg: &str = "Invalid argument.";
	pub const input_err: &str = "Something went wrong during reading the line. Try again.";
	pub const edit_err: &str = "I apologize for the inconvinience, but an editing engine has thrown an error while trying to run \"?edit [line]\". Instead of that, you can do the folowing: use \"?print [line]\", then copy it using Ctrl+C (Cmd+C on MacOS), use \"?replace [line]\", and press Ctrl+V (Cmd+V on MacOS).";
}

fn apply_to_lines(text: &mut str, fun: impl Fn(&mut Vec<&str>) -> Result<(),InputError>, index: &str) {
	let mut lines: Vec<&str> = text.split("\n").collect();
	if let Ok(index) = index.parse::<usize>() && index < lines.len() {
		match fun(&mut lines) {
			Ok(_) => text = lines.join("\n").as_str(),
			Err(GetLineErr(_)) => println!("{}", err_mes::input_err),
			Err(EditErr(_)) => println!("{}", err_mes::edit_err),
		}
	} else { println!("{}", err_mes::inv_arg); }
}

fn editor_loop() /*-> Result<(), Box<dyn Error>>*/ {
	let mut input = Input::new();
	let mut text = String::new();
	loop {
		let line;
		match input.get_line(": ") {
			Ok(ln) => line = ln,
			Err(_) => {
				println!("{}", err_mes::input_err);
				continue;
			},
		}
		
		
		let iter = line.split(' ');
		let (com, arg) = (iter.next(), iter.next());
		
		match (com, arg) {
			(Some("?clean"), _) => { text = String::new(); },
			(Some("?delete"), Some(index)) => {
				apply_to_lines(&mut text, |lns| lns.remove(index), index);
			},
			(Some("?edit"), Some(index)) => {
				apply_to_lines(&mut text, |lns| lns[index] = input.edit_line("Edit line: ", lns[index])?, index);
			},
			(Some("?replace"), Some(index)) => {
				apply_to_lines(&mut text, |lns| lns[index] = input.get_line("Replace with: ")?, index);
			},
			(Some("?delete"|"?edit"|"?replace"), None) => { println!("{}", err_mes::inv_arg); },
			(Some(x), _) => { text.append(x); },
			(None, _) => (),
		}

		text.push_str(&line);
	}
}

fn main() {
	editor_loop();
}
