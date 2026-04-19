use std::fs;
use std::str::Lines;
#[derive(Debug)]
pub struct Task {

	pub text: String,
	pub done: bool,
	pub file: String,
	pub line: usize,

}

impl Task {
	fn make_done(self) -> std::io::Result<()> {
		let content = fs::read_to_string(self.file)?;
		let task_text= "- []";

		let changed_text = content
			.lines()
			.map(|line| {
				if line.contains(task_text) {
					line.replace("- []", "- [x]")
				} else {
					line.to_string()
				}
			})
			.collect::<Vec<String>>()
			.join("\n");
		fs::write(content, changed_text)?;

		Ok(())
	}
}