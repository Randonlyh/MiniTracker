mod db;
mod sessions;
mod detection;
use db::*;
use sessions::*;
use detection::*;

use std::process::{Command, Stdio};
use std::env;
use std::io;

#[cfg(debug_assertions)]
use std::fs::File;
#[cfg(debug_assertions)]
use std::io::Write;

fn main() -> io::Result<()> {
	let mut process;
	let mut recording_mode = true;

	let connection = init_connection();
	db_setup(&connection).unwrap();

	let mut args = env::args();

	if args.len() < 2 {
		eprintln!("MiniTracker Error: No application supplied!");
		return Ok(());
	}

	// Removes the application from the arguments of the spawned app
	args.next();

	// Any environment variables are passed into the next program
	let mut envs: Vec<(String, String)> = env::vars().collect();

	let mut app_id = 0;
	let mut launch_id = 0;

	let mut current_var = args.next();
	while let Some(ref var) = current_var {
		match var.contains("--trackid=") {
			true => {
				recording_mode = false;

				let parsed = var.trim_start_matches("--trackid=");
				let split: Vec<&str> = parsed.split('.').collect();

				app_id = match split[0].parse::<i64>() {
					Ok(num) => num,
					Err(_) => panic!("MiniTracker Error: Could not parse manual App/Launch IDs! Aborting..."),
				};

				if split.len() > 1 {
					launch_id = match split[1].parse::<i64>() {
						Ok(num) => num,
						Err(_) => panic!("MiniTracker Error: Could not parse manual App/Launch IDs! Aborting..."),
					};
				}

				current_var = args.next();
				continue;
			}
			false => (),
		};

		match var.split_once('=') {
			Some((key, value)) => {
				// Any environment variables passed after this program still go into the next program
				envs.push((key.to_string(), value.to_string()));
			},
			None => {
				break;
			},
		}

		current_var = args.next();
	}

	let current_args: Vec<String> = args.collect();

	let app = current_var.unwrap();

	// Ripping all environment/arguments to a text file in Debug mode
	#[cfg(debug_assertions)]
	print_application_args_to_file(current_args.clone(), envs.clone(), &app);

	process = Command::new(&app)
		.envs(envs.clone().into_iter())
		.args(current_args.clone().into_iter())
		.stdin(Stdio::inherit())
		.stdout(Stdio::inherit())
		.spawn()
		.expect("MiniTracker Error: Application should be a valid process.");

	if recording_mode {
		// Parsing the path of the app
		let mut app_split: Vec<&str> = app.split('/').collect();

		app_id = automatic_app(&mut app_split, &current_args, &envs, &connection);
	} else {
		manual_mode(app_id, launch_id, &connection).unwrap();
	}

	let session_id = create_play_session(&connection, app_id, launch_id);
	let start_time: i64 = connection.query_one("SELECT unixepoch()", [], |row| row.get::<_, i64>(0)).unwrap();

	process.wait().unwrap();

	update_playtime(&connection, app_id, session_id, start_time);

	Ok(())
}

#[cfg(debug_assertions)]
fn print_application_args_to_file(args: Vec<String>, envs: Vec<(String, String)>, name: &String) {
	let log_file: String = match env::home_dir() {
		Some(path) => path.to_string_lossy().into_owned() + "/.local/share/randonlyh/stats/argument-log.txt",
		None => "./".to_string(),
	};

	let mut file = File::create(log_file).expect("File should be created.");

	for i in envs.into_iter() {
		writeln!(&mut file, "{}={} ", i.0, i.1).unwrap();
	}

	writeln!(&mut file, "").unwrap();
	writeln!(&mut file, "{}", name).unwrap();
	writeln!(&mut file, "").unwrap();

	for i in args.into_iter() {
		writeln!(&mut file, "{}", i).unwrap();
	}
}
