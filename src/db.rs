use std::env;
use std::fs::create_dir_all;

use rusqlite::{Connection, Error};

pub fn init_connection() -> Connection {
    let db_folder: String = match env::home_dir() {
        Some(path) => {
            path.to_string_lossy().into_owned() + "/.local/share/randonlyh/launcher/stats"
        }
        None => "./".to_string(),
    };

    let db_path = db_folder.clone() + "/db.sqlite";

    match Connection::open(&db_path) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("MiniTracker SQL Command Error: {}", err);
            println!("MiniTracker: Creating DB...");
            create_dir_all(&db_folder).unwrap();
            Connection::open(&db_path).unwrap()
        }
    }
}

pub fn db_setup(connection: &Connection) -> Result<(), Error> {
    connection.execute(
    	"CREATE TABLE IF NOT EXISTS Platforms(
         PlatformID INTEGER PRIMARY KEY,
         Name       TEXT NOT NULL UNIQUE
         );",
        [],
    )?;

    connection.execute(
    	"CREATE TABLE IF NOT EXISTS Apps(
         AppID       INTEGER PRIMARY KEY,
         Name        TEXT,
         Description TEXT,
         Developer   TEXT,
         Publisher   TEXT,
         ReleaseDate TEXT,
         PlatformID  INTEGER,
         ExternalID  TEXT,
         FOREIGN KEY (
             PlatformID
         )
         REFERENCES Platforms (PlatformID)
         );",
        [],
    )?;

    connection.execute(
    	"CREATE TABLE IF NOT EXISTS Users(
         UserID   INTEGER PRIMARY KEY,
         Username TEXT NOT NULL UNIQUE,
         Colour   INTEGER
         );",
        [],
    )?;

    connection.execute(
    	"CREATE TABLE IF NOT EXISTS UserData(
         UserID     INTEGER,
         AppID      INTEGER,
         Playtime   INTEGER NOT NULL DEFAULT 0,
         LastOpened TEXT,
         PRIMARY KEY (
             UserID,
             AppID
         ),
         FOREIGN KEY (
             UserID
         )
         REFERENCES Users (UserID),
         FOREIGN KEY (
             AppID
         )
         REFERENCES Apps (AppID)
         );",
        [],
    )?;

    connection.execute(
       	"CREATE TABLE IF NOT EXISTS LaunchOptions(
        AppID        INTEGER,
        LaunchID     INTEGER,
        Name         TEXT,
        Executable   TEXT,
        Installed    INTEGER,
        DisplayOrder INTEGER,
        PRIMARY KEY (
            LaunchID,
            AppID
        ),
        FOREIGN KEY (
            AppID
        )
        REFERENCES Apps (AppID)
        );",
        [],
    )?;

    connection.execute(
       	"CREATE TABLE IF NOT EXISTS PlaySessions(
        SessionID INTEGER PRIMARY KEY,
        UserID    INTEGER,
        AppID     INTEGER,
        LaunchID  INTEGER,
        StartedAt TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
        ClosedAt  TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
        Duration  INTEGER GENERATED ALWAYS AS (unixepoch(ClosedAt) - unixepoch(StartedAt)) STORED,
        FOREIGN KEY (
            UserID
        )
        REFERENCES Users (UserID),
        FOREIGN KEY (
            LaunchID,
            AppID
        )
        REFERENCES LaunchOptions (LaunchID,
        AppID)
        );",
        [],
    )?;

    connection.execute("INSERT OR IGNORE INTO Platforms(PlatformID, Name) VALUES (0, 'Unspecified'), (1, 'Windows'), (2, 'Linux'), 
         (3, 'Mac OS'), (4, 'MS-DOS'), (5, 'Steam'), (6, 'Epic Games'), (7, 'GOG'), (8, 'Itch.io'), (9, 'Game Jolt'), 
         (10, 'Zoom Platform'), (11, 'Mobile'), (12, 'Android'), (13, 'iOS'), (14, 'NES'), (15, 'Famicom'), (16, 
         'Famicom Disk System'), (17, 'SNES'), (18, 'Super Famicom'), (19, 'Virtual Boy'), (20, 'N64'), (21, 'N64 
         DD'), (22, 'GameCube'), (23, 'Wii'), (24, 'Wii U'), (25, 'Nintendo Switch'), (26, 'Nintendo Switch 2'), (27, 
         'Gameboy'), (28, 'Gameboy Colour'), (29, 'Gameboy Advance'), (30, 'Nintendo DS'), (31, 'Nintendo DSi'), (32, 
         'Nintendo 3DS'), (33, 'New Nintendo 3DS'), (34, 'PS1'), (35, 'PS2'), (36, 'PS3'), (37, 'PS4'), (38, 'PS5'), 
         (39, 'PSVR'), (40, 'PSVR2'), (41, 'PSP'), (42, 'PSVita'), (43, 'Xbox'), (44, 'Xbox 360'), (45, 'Xbox One'), 
         (46, 'Xbox Series'), (47, 'Master System'), (48, 'Mega Drive'), (49, 'Mega CD'), (50, 'Mega Drive 32X'), 
         (51, 'Sega Saturn'), (52, 'Dreamcast'), (53, 'Game Gear'), (54, 'Arcade');", [])?;

    connection.execute(
        "INSERT OR IGNORE INTO Users(UserID, Username) VALUES(1, 'User')",
        [],
    )?;
    Ok(())
}
