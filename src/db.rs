use std::env;
use std::fs::create_dir_all;

use rusqlite::{Connection, Error};

pub fn init_connection() -> Connection {
   	let db_folder: String = match env::home_dir() {
	    Some(path) => path.to_string_lossy().into_owned() + "/.local/share/randonlyh/stats",
		None => "./".to_string(),
	};
   
	let db_path = db_folder.clone() + "/apps.sqlite";
	
	match Connection::open(&db_path) {
	    Ok(conn) => conn,
		Err(err) => {
            eprintln!("Staty SQL Command Error: {}", err);
            println!("Staty: Creating DB...");
            create_dir_all(&db_folder).unwrap();
            Connection::open(&db_path).unwrap()
        },
    }
}

pub fn db_setup(connection: &Connection) -> Result<(), Error> {
   	connection.execute("CREATE TABLE IF NOT EXISTS Platforms(
		PlatformID INTEGER PRIMARY KEY,
		Name TEXT);", [])?;
	
	connection.execute("CREATE TABLE IF NOT EXISTS Apps(
		AppID INTEGER PRIMARY KEY,
		Name TEXT,
		PlatformID INTEGER,
		ExternalID INTEGER,
		Developer TEXT,
		Publisher TEXT,
		ReleaseDate INTEGER,
		FOREIGN KEY(PlatformID) REFERENCES Platforms(PlatformID)
		);", [])?;
   
	connection.execute("CREATE TABLE IF NOT EXISTS Users(
	    UserID INTEGER PRIMARY KEY,
		Username TEXT
		);", [])?;
	
	connection.execute("CREATE TABLE IF NOT EXISTS UserData(
		UserID INTEGER,
		AppID INTEGER,
		Playtime INTEGER,
	    LastOpened INTEGER,
		PRIMARY KEY(UserID, AppID),
		FOREIGN KEY(UserID) REFERENCES Users(UserID),
		FOREIGN KEY(AppID) REFERENCES Apps(AppID)
		);", [])?;

	connection.execute("CREATE TABLE IF NOT EXISTS LaunchOptions(
		LaunchID INTEGER,
		AppID INTEGER,
		Name TEXT,
		EnvVars TEXT,
	    InstallPath INTEGER,
		Arguments TEXT,
		DisplayOrder INTEGER,
		PRIMARY KEY(LaunchID, AppID),
		FOREIGN KEY(AppID) REFERENCES Apps(AppID)
		);", [])?;

	connection.execute("CREATE TABLE IF NOT EXISTS PlaySessions(
	    SessionID INTEGER PRIMARY KEY,
		UserID INTEGER,
		AppID INTEGER,
		StartedAt INTEGER,
	    ClosedAt INTEGER,
		Duration INTEGER,
		LaunchID INTEGER,
		FOREIGN KEY(UserID) REFERENCES Users(UserID),
		FOREIGN KEY(AppID) REFERENCES Apps(AppID),
		FOREIGN KEY(LaunchID, AppID) REFERENCES LaunchOptions(LaunchID, AppID)
		);", [])?;
   
	connection.execute("INSERT OR IGNORE INTO Platforms(PlatformID, Name)
	    VALUES (0, 'Unspecified'),
		(1, 'Windows'), (2, 'Linux'), (3, 'MacOS'), (4, 'MS-DOS'),
        (5, 'Steam'), (6, 'Epic Games'), (7, 'GOG'), (8, 'Itch.io'), (9, 'Game Jolt'), (10, 'Zoom Platform'),
        (11, 'Mobile'), (12, 'Android'), (13, 'iOS'),
        (14, 'NES'), (15, 'Famicom'), (16, 'Famicom Disk System'),
        (17, 'SNES'), (18, 'Super Famicom'),
        (19, 'VirtualBoy'),
        (20, 'N64'), (21, 'N64 DD'),
        (22, 'Gamecube'), (23, 'Wii'), (24, 'WiiU'),
        (25, 'Switch'), (26, 'Switch 2'),
        (27, 'Gameboy'), (28, 'Gameboy Colour'), (29, 'Gameboy Advance'),
        (30, 'DS'), (31, 'DSi'),
        (31, '3DS'), (32, 'New 3DS'),
        (33, 'PS1'), (34, 'PS2'), (35, 'PS3'), (36, 'PS4'), (37, 'PS5'),
        (38, 'PSVR'), (39, 'PSVR2'),
        (40, 'PSP'), (41, 'PSVita'),
        (42, 'Xbox'), (43, 'Xbox 360'), (44, 'Xbox One'), (45, 'Xbox Series'),
        (46, 'MasterSystem'), (47, 'MegaDrive'), (48, 'Mega CD'), (49, 'MegaDrive 32X'),
        (50, 'Sega Saturn'),
        (51, 'Dreamcast'),
        (52, 'Game Gear'),
        (53, 'Arcade');", [])?;

	connection.execute("INSERT OR IGNORE INTO Users(Username) VALUES('User')", [])?;
	Ok(())
}