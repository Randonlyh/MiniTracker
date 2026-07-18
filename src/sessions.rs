use rusqlite::Connection;

pub fn create_play_session(connection: &Connection, app_id: i64, launch_id: i64) -> i64 {
	// Add a new play session
	if let Err(e) = connection.execute("INSERT INTO PlaySessions(UserID, AppID, StartedAt, LaunchID)
	VALUES(1, ?1, datetime('now', 'localtime'), ?2)", [app_id, launch_id]) {
		eprintln!("Staty SQL Error: {}", e);
	}

	// Grab play session ID for later
	connection.query_row("SELECT SessionID
	FROM PlaySessions WHERE AppID == ?1 AND LaunchID == ?2 ORDER BY StartedAt DESC",
	[app_id, launch_id], |row| row.get::<_, i64>(0)).unwrap()
}

pub fn update_playtime(connection: &Connection, app_id: i64, session_id: i64, start_time: i64) {
	// Update play session with end time and duration
	connection.execute("UPDATE PlaySessions
	SET ClosedAt = datetime('now', 'localtime'),
	Duration = strftime('%s', 'now') - ?1
	WHERE SessionID = ?2", [start_time, session_id]).unwrap();

	// Update total playtime
	connection.execute("UPDATE UserData
	SET Playtime = Playtime + (SELECT Duration FROM PlaySessions WHERE SessionID == ?1),
	LastOpened = (SELECT ClosedAt FROM PlaySessions WHERE SessionID == ?1)
	WHERE AppID == ?2 AND UserID == 1", [session_id, app_id]).unwrap();
}
