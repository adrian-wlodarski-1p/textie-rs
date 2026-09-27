use std::io::{self, Write, BufRead};
// credit to https://stackoverflow.com/questions/73837411/rust-create-editable-cli-input

use rustyline::{DefaultEditor};

pub struct Input {
	stdin: io::Stdin,
	editor: Option<DefaultEditor>
}

impl Input {
	pub fn new() -> Input {
		Input {
			stdin: io::stdin(),
			editor: None,
		}
	}
	
	pub fn get_line(&mut self, prompt: &str) -> Result<String, io::Error> {
		print!("{}", prompt);
		io::stdout().flush()?; // send text
		
		let mut line = String::new();
		self.stdin.read_line(&mut line)?;
		Ok(line)
	}
	
	pub fn edit_line(&self, prompt: &str, text: &str) -> rustyline::Result<()> {
		if self.editor.is_none() { self.editor = Some(DefaultEditor::new()?); }
		self.editor.readline_with_initial(prompt, (text, ""))?
	}
}

fn main() {
	let mut input = Input::new();
	assert!(matches!(input.get_line("Test: "), Ok(_)));
	assert!(matches!(input.edit_line("Test2: ", "default"), Ok(_)));
	println!("Input.rs loaded successfully.");
}