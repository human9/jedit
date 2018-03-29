extern crate ncurses;
use ncurses::*;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub struct Buffer {
	pub lines: Vec<String>, // each string in the vec is a line in the buffer
	pub pos: (usize, usize), // cursor position in the buffer
	pub scroll: usize, // top line we are scrolled to
}

impl Buffer {

	pub fn new(file: File) -> Self {
		let reader = BufReader::new(&file);
		let mut lines = Vec::new();
		for line in reader.lines() {
			lines.push(line.unwrap());
		}
		lines.push(String::new());
		Buffer {
			lines,
			pos: (0,0),
			scroll: 0,
		}
	}

	pub fn to_string(&self) -> String {
		let mut s = String::new();
		for (i, line) in self.lines.iter().enumerate() {
            s.push_str(line);
            if i != self.lines.len()-1 {
                s.push_str("\n");
            }
        }
        s
	}

	/// Write the contents of the buffer to the given file
	pub fn write_to_file(&self, file: &mut File) -> Result<(), Box<Error>> {
        
        file.write(self.to_string().as_bytes())?;
		Ok(())
	}

	pub fn take_input(&mut self, ch: i32) -> Result<(), ()> {
		match ch {
			27 /* ESC */ => {
				return Err(()); // calling function should handle this
			},
			10 /* ENTER */ => {
                let new: String = self.lines.get(self.pos.1).unwrap()[self.pos.0..].to_string();
                self.lines.get_mut(self.pos.1).unwrap().truncate(self.pos.0);
				self.pos.1 += 1;
				self.lines.insert(self.pos.1, new); // push a new line
				self.pos.0 = 0;
			},
			127 | KEY_BACKSPACE | KEY_DC | KEY_DL => /* how does backspace */{
				if self.pos.0 > 0  { 
					// there are characters to delete, so delete them, easy
					self.lines.get_mut(self.pos.1).unwrap().remove(self.pos.0-1); 
                    self.pos.0 -= 1;
				}
				else if self.pos.1 > 0 {
					// no characters to delete, but lines to alter
					if self.lines.get(self.pos.1).unwrap().len() > 0 {
						// there are no characters left to delete, but we aren't on the first line
						// so we have to move this line to the end of the last
	                    let mv: String = self.lines.get(self.pos.1).unwrap().to_string();
	                    self.lines.remove(self.pos.1);
	                    self.pos.1 -= 1;
	                    self.pos.0 = self.lines.get(self.pos.1).unwrap().len();
	                    self.lines.get_mut(self.pos.1).unwrap().push_str(&mv);
					}
					else {
						// just delete this line, and move to the end of the one above
						self.lines.remove(self.pos.1);
						self.pos.1 -= 1;
						self.pos.0 = self.lines.get(self.pos.1).unwrap().len();
					}	
				}			
			},
			KEY_UP => {
				if self.pos.1 > 0 {
					self.pos.1 -= 1;
					let line_len = self.lines.get(self.pos.1).unwrap().len();
					if line_len < self.pos.0 {
						self.pos.0 = line_len; 
					}
				}
			},
			KEY_DOWN => {
				if self.pos.1 < self.lines.len() - 1 {
					self.pos.1 += 1;
					let line_len = self.lines.get(self.pos.1).unwrap().len();
					if line_len < self.pos.0 {
						self.pos.0 = line_len; 
					}
				}

			},
			KEY_LEFT => {
				if self.pos.0 > 0 {
					self.pos.0 -= 1;
				}
				else if self.pos.1 > 0 {
					self.pos.1 -= 1;
					let line_len = self.lines.get(self.pos.1).unwrap().len();
					self.pos.0 = line_len;
				}
			},
			KEY_RIGHT => {
				if self.pos.0 < self.lines.get(self.pos.1).unwrap().len() {
					self.pos.0 += 1;
				}
				else if self.pos.1 < self.lines.len() - 1 {
					self.pos.1 += 1;
					self.pos.0 = 0;
				}
			},
			32 ... 126 => {
				self.lines.get_mut(self.pos.1).unwrap().insert(self.pos.0, ch as u8 as char); // push character to the last line
				self.pos.0 += 1;
			},
			_ => (), //don't know
		}
		Ok(())

	}
}
