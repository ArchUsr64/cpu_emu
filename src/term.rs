extern crate ctrlc;

#[macro_export]
macro_rules! log {
	($y:expr, $($x:expr), * ) => {
		if $y {
			let mut time = std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)
				.unwrap()
				.as_secs();
			let time_zone_offset = (5 * 3600) + (30 * 60);
			time += time_zone_offset;
			let hours = (time / 3600) % 24;
			let minutes = (time / 60) % 60;
			let seconds = time % 60;
			print!("[{:02}:{:02}:{:02}] ", hours, minutes, seconds);
			println!("[DEBUG] {}" ,format!($($x,)*));
		};
	};
}

pub fn ctrl_c_init() {
	ctrlc::set_handler(move || {
		println!("Keyboard Interrupt");
		std::process::exit(1);
	})
	.expect("Error setting Ctrl-C handler");
}
