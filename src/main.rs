extern crate dirs;
extern crate colored;

use std::ffi::CString;
use nix::unistd::*;
use nix::sys::wait::*;
use std::fs;
use std::io::prelude::*;
use shell::parser::parser;
use colored::*;

fn main() {
	println!("使えるコマンドは以下");
	println!("exit: プロセス終了");
	println!("ls: ディレクトリ内ファイル一覧表示");
	println!("cd: ディレクトリ移動");
	println!("pwd: 現在ディレクトリ一表示");
	println!("mkdir: ディレクトリ作成");
	println!("touch: ファイル作成");
	println!("cat: ファイル内容表示");
	println!("rm: ファイル削除");
	println!("text hoge: hogeファイルを編集");
	println!("dentaku: dentakuインタラクティブシェルを開く");
	println!("help: コマンド一覧");
	println!("実行ファイルの実行も可能");
	let mut dir = dirs::home_dir().unwrap();
	loop{
		match &dir.to_str() {
			Some(v) => println!("{}", v.blue()),
			None => println!("{}", "失敗".red()),
		};
		let command: Vec<String> = parser::read_vec();
		if command.len() == 0 {
			continue;
		}
		match &*command[0] {
			"exit" => {
				println!("{}", "プロセスを終了します".red());
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
				for path in fs::read_dir(&target).unwrap() {
					files.push(path.unwrap().path().display().to_string().replacen(&target, "", 1))
				}

				files.sort();
				for file in files {
					println!("{}", file.replacen("/", "", 1).blue());
				}
			},
			"cd" => {
				if command.len() == 1 {
					dir = dirs::home_dir().unwrap();
				} else if &*command[1] == ".." {
					dir = dir.parent().unwrap().to_path_buf();
				} else if &*command[1] == "~" {
					dir = dirs::home_dir().unwrap();
				} else {
					dir.push(&*command[1]);
					let result = fs::File::open(&dir);
					match result {
						Ok(_) => {
							//println!("ディレクトリ変更"),
						},
						Err(_) => {
							println!("{}", "指定したディレクトリは存在しません".red());
							dir = dir.parent().unwrap().to_path_buf();
						}
					};
				}
			},
			"pwd" => {
				println!("{:?}", dir);
			},
			"mkdir" => {
				if command.len() <= 1 {
					continue;
				}
				dir.push(&*command[1]);
				match &dir.to_str() {
					Some(v) => {
						let _ = fs::create_dir(v);
					},
					None => {
						println!("{}", "ディレクトリ作成失敗".red());
					},
				};
				dir = dir.parent().unwrap().to_path_buf();
			},
			"touch" => {
				if command.len() <= 1 {
					continue;
				}
				dir.push(&*command[1]);
				match &dir.to_str() {
					Some(v) => {
						let _ = fs::File::create(v);
						println!("ファイル作成完了");
					},
					None => {
						println!("{}", "ファイル作成失敗".red());
					},
				};
				dir = dir.parent().unwrap().to_path_buf();
			},
			"cat" => {
				if command.len() <= 1 {
					continue;
				}
				dir.push(&*command[1]);
				match &dir.to_str() {
					Some(v) => {
						let mut file = fs::File::open(v).unwrap();
						let mut contents = String::new();
						file.read_to_string(&mut contents).unwrap();
						println!("{}", contents);
					},
					None => {
						println!("{}", "ファイル開くのに失敗".red());
					},
				}
				dir = dir.parent().unwrap().to_path_buf();
			},
			"rm" => {
				if command.len() <= 1 {
					continue;
				}
				dir.push(&*command[1]);
				match &dir.to_str() {
					Some(v) => {
						let _ = fs::remove_file(v);
						println!("ファイル削除完了");
					},
					None => {
						println!("{}", "ファイル削除失敗".red());
					},
				};
				dir = dir.parent().unwrap().to_path_buf();
			},
			"help" => {
				println!("使えるコマンドは以下");
				println!("exit: プロセス終了");
				println!("ls: ディレクトリ内ファイル一覧表示");
				println!("cd: ディレクトリ移動");
				println!("pwd: 現在ディレクトリ一表示");
				println!("mkdir: ディレクトリ作成");
				println!("touch: ファイル作成");
				println!("cat: ファイル内容表示");
				println!("rm: ファイル削除");
				println!("text hoge: hogeファイルを編集");
				println!("dentaku: dentakuインタラクティブシェルを開く");
				println!("help: コマンド一覧");
				println!("実行ファイルの実行も可能");
			},
			"dentaku" => {
				match fork().expect("プロセス分離に失敗") {
					ForkResult::Parent { child } => {
						let wstatus: Option<WaitPidFlag> = None;
						let _ = waitpid(child, wstatus);
					},
					ForkResult::Child => {
						dir.push(&*command[0]);
						let path = CString::new("/Users/tomoya/rustworks/dentaku/target/debug/./dentaku").unwrap();
						//let path = CString::new("./text_editer").unwrap();
						let args = if command.len() > 1 {
							dir.set_file_name(&command[1]);
							CString::new(dir.to_str().unwrap()).unwrap()
							//CString::new(dir + command[1].to_string()).unwrap()
						} else {
							CString::new("").unwrap()
						};
						execv(
							&path,
							&[
								path.clone(),
								args,
							],
						).expect("textプログラム失敗");
					},
				};
			},
			"text" => {
				match fork().expect("プロセス分離に失敗") {
					ForkResult::Parent { child } => {
						let wstatus: Option<WaitPidFlag> = None;
						let _ = waitpid(child, wstatus);
					},
					ForkResult::Child => {
						dir.push(&*command[0]);
						let path = CString::new("/Users/tomoya/rustworks/text_editer/target/debug/./text_editer").unwrap();
						//let path = CString::new("./text_editer").unwrap();
						let args = if command.len() > 1 {
							dir.set_file_name(&command[1]);
							CString::new(dir.to_str().unwrap()).unwrap()
							//CString::new(dir + command[1].to_string()).unwrap()
						} else {
							CString::new("").unwrap()
						};
						execv(
							&path,
							&[
								path.clone(),
								args,
							],
						).expect("textプログラム失敗");
					},
				};
			},
			_ => {
				match fork().expect("プロセス分離に失敗") {
					ForkResult::Parent { child } => {
						let wstatus: Option<WaitPidFlag> = None;
						let _ = waitpid(child, wstatus);
					},
					ForkResult::Child => {
						dir.push(&*command[0]);
						match &dir.to_str() {
							Some(v) => {
								let path = CString::new(v.to_string()).unwrap();
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
							None => {
								println!("{}", "子プロセス失敗".red());
							},
						};
					},
				};
			},
		};
	}
}
