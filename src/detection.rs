use rusqlite::{Connection, Error, params};

#[allow(unused)]
#[derive(Copy, Clone)]
enum PlatformIDs {
	Unspecified = 0,
	Windows = 1,
	Linux = 2,
	MacOS = 3,
	MSDOS = 4,
	Steam = 5,
	EpicGames = 6,
	GOG = 7,
	ItchIo = 8,
	GameJolt = 9,
	ZoomPlatform = 10,
	Mobile = 11,
	Android = 12,
	IOS = 13,
	NES = 14,
	Famicom = 15,
	FamicomDiskSystem = 16,
	SNES = 17,
	SuperFamicom = 18,
	VirtualBoy = 19,
	N64 = 20,
	N64DiskDrive = 21,
	Gamecube = 22,
	Wii = 23,
	WiiU = 24,
	Switch = 25,
	Switch2 = 26,
	Gameboy = 27,
	GameboyColour = 28,
	GameboyAdvance = 29,
	DS = 30,
	DSi = 31,
	Old3DS = 32,
	New3DS = 33,
	PS1 = 34,
	PS2 = 35,
	PS3 = 36,
	PS4 = 37,
	PS5 = 38,
	PSVR = 39,
	PSVR2 = 40,
	PSP = 41,
	PSVita = 42,
	Xbox = 43,
	Xbox360 = 44,
	XboxOne = 45,
	XboxSeries = 46,
	MasterSystem = 47,
	MegaDrive = 48,
	MegaCD = 49,
	MegaDrive32X = 50,
	SegaSaturn = 51,
	Dreamcast = 52,
	GameGear = 53,
	Arcade = 54
}

pub fn manual_mode(app_id: i64, launch_id: i64, connection: &Connection) -> Result<(), Error> {
	// Check if IDs exists and create them if not
	connection.execute("INSERT OR IGNORE INTO Apps(AppID) VALUES(?1)", [app_id])?;
	connection.execute("INSERT OR IGNORE INTO UserData(UserID, AppID, Playtime) VALUES(1, ?1, 0)", [app_id])?;
	connection.execute("INSERT OR IGNORE INTO LaunchOptions(LaunchID, AppID) VALUES(?1, ?2)", [launch_id, app_id])?;
	Ok(())
}

pub fn automatic_app(app_split: &Vec<&str>, connection: &Connection) -> i64 {
	let mut app_name = *app_split.last().unwrap();
	trim_app_name(&mut app_name);

	let platform_id: i64 = find_platform();
	let app_id: i64 = match connection.query_row("SELECT AppID FROM Apps WHERE Name == ?1 AND PlatformID == ?2",
	params!(app_name, platform_id), |row| row.get::<_, i64>(0)) {
		Ok(id) => id,
		Err(_) => {
			match connection.query_row("INSERT INTO Apps(Name, PlatformID) VALUES(?1, ?2) RETURNING AppID",
			params!(app_name, platform_id), |row| row.get::<_, i64>(0)) {
				Ok(id) => id,
				Err(err) => panic!("MiniTracker SQL Command Error: {}", err),
			}
		},
	};

	// Create UserData entry for app if it does not exist
	match connection.query_row("SELECT * FROM UserData WHERE AppID == ?1 AND UserID == 1",
	[app_id], |row| row.get::<_, i64>(0)) {
		Ok(_) => (),
		Err(_) => {
			connection.execute("INSERT INTO UserData(UserID, AppID, Playtime) VALUES(1, ?1, 0)", [app_id]).unwrap();
		},
	};

	connection.execute("INSERT OR IGNORE INTO LaunchOptions(LaunchID, AppID, Name)
		VALUES(0, ?1, ?2)", params![app_id, app_name]).unwrap();

	app_id
}

#[allow(unused_assignments)]
fn trim_app_name(name: &mut &str) {
	*name = name.trim_end_matches(".sh");
	*name = name.trim_end_matches(".exe");
	*name = name.trim_end_matches(".elf");
	*name = name.trim_end_matches(".appimage");
}

fn find_platform() -> i64 {
	PlatformIDs::Linux as i64
}
