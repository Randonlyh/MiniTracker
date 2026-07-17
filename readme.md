# MiniTracker

## The Problem

There is no good way to universally track playtime for your apps and games on your PC. Sure game launchers like Steam can do it, but Steam for example only tracks Steam games (Non-Steam games are always 0) and it will only track your playtime when online. Launchers like Steam are also quite heavy, and you should not need a massive RAM and launch time penalty when you just want to see how long you used something for!

## The Solution

MiniTracker solves this by being a small lightweight wrapper for Linux. Upon launching an app/game with MiniTracker prepended to it like `mtracker path/to/your/app`, your start time of the session of the process is recorded. Once the application is closed, the end time, duration of the session and total playtime (Every session added up) is also calculated. Everything is stored in a small SQLite database in `$HOME/.local/share/randonlyh/stats/apps.sqlite`, so you can easily look through your data for everything. (A dedicated app for visualising all your stats should come at a later date!)
The app is tiny, depending on only the Rust Standard Library, and while running it uses ~4Mb of RAM and has no noticeable impact on app launch/closing speeds.

### Advanced Usage

MiniTracker has 2 modes. By defualt it uses Automatic detection, which uses the environment variables, application, and arguments to try to figure out the name and other details of the app/game you're using. This allows it to update the database under the correct name or create a new entry. That being said it should be more than good enough for telling apps apart from each other. Currently MiniTracker detects every app as a Linux application, but support for the following platforms are planned:
* Steam
* Dolphin (Gamecube/Wii)
* Cemu (Wii U)
* Heroic
  * GOG
  * Epic Games
* Waydroid

With more to come as I need them or per request.

Do note that detection can't figure out too much information. There is only so much you can surmise from these values, and trying to check a database of some kind would bloat the app. To solve this there is a secondary mode:

#### Manual Mode

Usage:
```bash
mtracker --trackid=54.1 path/to/your/game
mtracker --trackid={AppID}.{LaunchID} path/to/your/game
```

You can manually assign an AppID and LaunchID before your application and MiniTracker will use your values, ignoring everything else about the app. This is useful in 2 main ways:
* You want to manually fill out the details of Apps in the database and still allow MiniTracker to track under the correct ID's
* An app such as a game launcher fills out your App table, and it can map each game to IDs for you to track correctly

> [!WARNING]
> You can very easily overwrite an existing entry using this system which will skew your data. Be careful!

> [!NOTE]
> Automatic detection will always set your LaunchID to 0. If you have an app that you launch in different ways (e.g. a Modded vs Vanilla game), but you would still like them to be tracked underneath the same overall App, that's where LaunchID comes in.

## Build Instructions

Simply run:
```bash
cargo build --release
```

in the same folder you copied the repository to (Assuming you have `rust` installed). Compilation may take a bit longer than expected as it only uses 1 codegen-unit to maximise compiler optimisations (You can disable this if wanted by editing the `Cargo.toml` file).

## AI Disclaimer

AI was used for a tiny amount of debugging and cleanup. All logic was handwritten.
