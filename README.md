# Track Topia





What we need is a database. 

database/mod.rs trait
* initialize() -> Result<(), AppError>

* is_initialized() -> bool

    checks for file presence and table presence

* add_tracker(Tracker) -> Result<(), AppError>

    error if tracker already present

* add_tracker_entry(TrackerEntry) -> Result<(), AppError>

    error if tracker not present

* get_tracker_entries([TrackerFilter]) -> Result<[TrackerEntry], AppError>
* get_trackers([TrackerFilter]) -> Result<[TrackerEntry], AppError>

use SqliteDatabase as DatabaseImpl;

database/sqlite.rs implementation



model.rs
* Tracker
* TrackerEntry
* TrackerType = enum Value, Contiguous
* TrackerFilter = enum on name (value), time (operation, value)
