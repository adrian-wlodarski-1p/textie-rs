use std::io::{self, Write};
use std::fmt;

// credit to https://stackoverflow.com/questions/73837411/rust-create-editable-cli-input
use rustyline::{DefaultEditor, error::ReadlineError};

pub struct Input {
	stdin: io::Stdin,
	editor: Option<DefaultEditor>,
}

impl Input {
	pub fn new() -> Input {
		Input {
			stdin: io::stdin(),
			editor: None,
		}
	}
	
	pub fn get_line(&mut self, prompt: &str) -> Result<String, InputError> {
		print!("{}", prompt);
		io::stdout().flush()?; // send text
		
		let mut line = String::new();
		self.stdin.read_line(&mut line)?;
		Ok(line)
	}
	
	pub fn edit_line(&self, prompt: &str, text: &str) -> rustyline::Result<()> {
		let editor;
		match self.editor {
			None => { editor = self.editor = Some(DefaultEditor::new()?); },
			Some(ed) => { editor = ed; },
		};
		editor.readline_with_initial(prompt, (text, ""))?
	}
}

pub enum InputError {
	GetLineErr(io::Error),
	EditErr(ReadlineError),
}
pub use InputError::*;

impl fmt::Display for InputError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self { GetLineErr(e) => e.fmt(f),
			         EditErr(e) => e.fmt(f), }
	}
}
impl From<io::Error> for InputError {
	fn from(e: io::Error) -> Self { GetLineErr(e) }
}
impl From<ReadlineError> for InputError {
	fn from(e: ReadlineError) -> Self { EditErr(e) }
}

#[cfg(test)]
mod tests {
	#[test]
	pub fn test_get_line() {
		assert!(matches!(Input::new().edit_line("Test input: "), Ok(_)));
	}
	
	#[test]
	pub fn test_edit_line() {
		assert!(matches!(Input::new().edit_line("Test edit: ", "default"), Ok(_)));
	}
}
