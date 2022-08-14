mod term;

fn main() {
	let mut term_mode: bool = false;
	let args: Vec<String> = std::env::args().collect();
	if args.len() > 1 {
		term_mode = true;
	}
	if term_mode {
		term::ctrl_c_init();
		let t: term::TerminalSize = term::get_terminal_size();
		println!("Current terminal size: {}x{}", t.rows, t.cols);
		if std::cmp::min(t.rows, t.cols) < 256 {
			println!("Terminal size insufficient, should be greater than 255")
		}
		println!("Row: {}, Col: {}", t.rows, t.cols);
	}
}
