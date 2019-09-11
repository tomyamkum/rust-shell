extern crate dirs;

use std::ffi::CString;
use nix::unistd::*;
use nix::sys::wait::*;
use std::fs;
use std::io::prelude::*;

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
	let mut s = String::new();
	std::io::stdin().read_line(&mut s).ok();
	s.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
	let mut dir = dirs::home_dir().unwrap();
	loop{
		match &dir.to_str() {
			Some(v) => println!("{}", v),
			None => println!("失敗"),
		};
		let command: Vec<String> = read_vec();
		if command.len() == 0 {
			continue;
		}
		match &*command[0] {
			"exit" => {
				println!("プロセスを終了します");
				break;
			},
			"ls" => {
				let mut target: String;
				match &dir.to_str() {
					Some(v) => {
						target = v.to_string();
					},
					None => {
						target = "".to_string();
					},
				};
				let mut files: Vec<String> = Vec::new();
				println!("{:?}",fs::read_dir(&target).unwrap());
				for path in fs::read_dir(&target).unwrap() {
					files.push(path.unwrap().path().display().to_string().replacen(&target, "", 1))
				}

				files.sort();
				for file in files {
					println!("{}", file);
				}
			},
			"cd" => {
				if command.len() == 1 {
					dir = dirs::home_dir().unwrap();
				} else if &*command[1] == ".." {
					dir = dir.parent().unwrap().to_path_buf();
				} else {
					dir.push(&*command[1]);
					let result = fs::File::open(&dir);
					match result {
						Ok(_) => {
							//println!("ディレクトリ変更"),
						},
						Err(_) => dir = dir.parent().unwrap().to_path_buf(),
					};
				}
			},
			"pwd" => {
				println!("{:?}", dir);
			},
			"mkdir" => {
				dir.push(&*command[1]);
				match &dir.to_str() {
					Some(v) => {
						let _ = fs::create_dir(v);
					},
					None => {
						println!("ディレクトリ作成失敗");
					},
				};
				dir = dir.parent().unwrap().to_path_buf();
			},
			"touch" => {
				dir.push(&*command[1]);
				match &dir.to_str() {
					Some(v) => {
						let _ = fs::File::create(v);
					},
					None => {
						println!("ファイル作成失敗");
					},
				};
				dir = dir.parent().unwrap().to_path_buf();
			},
			"cat" => {
				dir.push(&*command[1]);
				match &dir.to_str() {
					Some(v) => {
						let mut file = fs::File::open(v).unwrap();
						let mut contents = String::new();
						file.read_to_string(&mut contents).unwrap();
						println!("{}", contents);
					},
					None => {
						println!("ファイル開くのに失敗");
					},
				}
				dir = dir.parent().unwrap().to_path_buf();
			},
			_ => {
				match fork().expect("プロセス分離に失敗") {
					ForkResult::Parent { child } => {
						let wstatus: Option<WaitPidFlag> = None;
						let _ = waitpid(child, wstatus);
						println!("プロセス{}が終了", child);
					},
					ForkResult::Child => {
						let path = CString::new(&*command[0].to_string()).unwrap();
						let args = if command.len() > 1 {
							CString::new(command[1].to_string()).unwrap()
						} else {
							CString::new("").unwrap()
						};
					
						execv(
							&path,
							&[
								path.clone(),
								args,
							],
						).expect("子プロセス失敗");
					},
				};
			},
		};
		println!();
	}
}
