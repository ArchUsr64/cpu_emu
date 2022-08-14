extern crate ctrlc;
extern crate termsize;

pub struct TerminalSize {
	pub cols: u32,
	pub rows: u32,
}

pub fn get_terminal_size() -> TerminalSize {
	let mut return_struct: TerminalSize = TerminalSize { rows: 0, cols: 0 };
	termsize::get().map(|size| {
		return_struct.rows = size.rows as u32;
		return_struct.cols = size.cols as u32;
	});
	return_struct
}

pub fn reset_terminal() {
	print!("\x1B[2J\x1B[1;1H");
}

pub fn draw_grayscale(val: f32) {
	print_ansi_string(val, " ".to_string());
}

pub fn print_ansi_string(val: f32, text: String) {
	let mut temp_val = val;
	temp_val = 232.0 + temp_val * 23.0;
	print!("\x1b[48;5;{}m{}", temp_val as u16, text);
}

pub fn ctrl_c_init() {
	ctrlc::set_handler(move || {
		println!("Keyboard Interrupt");
		std::process::exit(1);
	})
	.expect("Error setting Ctrl-C handler");
}
