pub struct Command {
}

pub fn read_vec<T: std::str::FromStr>() -> Vec<T> {
	let mut s = String::new();
	std::io::stdin().read_line(&mut s).ok();
	s.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect()
}

pub fn get_command(_command_s: String) {
}
