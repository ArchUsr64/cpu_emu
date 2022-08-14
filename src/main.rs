extern crate ctrlc;
extern crate termsize;

struct TerminalSize {
	cols: u32,
	rows: u32,
}

fn main() {
	let mut term_mode: bool = false;
	let args: Vec<String> = std::env::args().collect();
	if args.len() > 1 {
		term_mode = true;
	}
	if term_mode {
		ctrl_c_init();
		let t: TerminalSize = get_terminal_size();
		println!("Current terminal size: {}x{}", t.rows, t.cols);
		if std::cmp::min(t.rows, t.cols) < 256 {
			println!("Terminal size insufficient, should be greater than 255")
		}
		println!("Row: {}, Col: {}", t.rows, t.cols);
	}
}

fn get_terminal_size() -> TerminalSize {
	let mut return_struct: TerminalSize = TerminalSize { rows: 0, cols: 0 };
	termsize::get().map(|size| {
		return_struct.rows = size.rows as u32;
		return_struct.cols = size.cols as u32;
	});
	return_struct
}

fn reset_terminal() {
	print!("\x1B[2J\x1B[1;1H");
}

fn draw_grayscale(val: f32) {
	print_ansi_string(val, " ".to_string());
}

fn print_ansi_string(val: f32, text: String) {
	let mut temp_val = val;
	temp_val = 232.0 + temp_val * 23.0;
	print!("\x1b[48;5;{}m{}", temp_val as u16, text);
}

fn ctrl_c_init() {
	ctrlc::set_handler(move || {
		println!("Keyboard Interrupt");
		std::process::exit(1);
	})
	.expect("Error setting Ctrl-C handler");
}
