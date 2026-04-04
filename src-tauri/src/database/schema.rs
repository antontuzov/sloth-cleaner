/// Database schema definitions
pub const CREATE_SCANS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS scans (
        id TEXT PRIMARY KEY,
        timestamp DATETIME NOT NULL,
        total_size INTEGER NOT NULL,
        file_count INTEGER NOT NULL,
        duration_ms INTEGER NOT NULL,
        categories_json TEXT NOT NULL
    )
";

pub const CREATE_CLEANUPS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS cleanups (
        id TEXT PRIMARY KEY,
        scan_id TEXT REFERENCES scans(id),
        timestamp DATETIME NOT NULL,
        freed_space INTEGER NOT NULL,
        files_deleted INTEGER NOT NULL,
        snapshot_id TEXT,
        status TEXT NOT NULL
    )
";

pub const CREATE_USER_PREFERENCES_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS user_preferences (
        category TEXT PRIMARY KEY,
        keep_count INTEGER DEFAULT 0,
        delete_count INTEGER DEFAULT 0,
        last_interaction DATETIME,
        confidence_score REAL DEFAULT 0.5
    )
";

pub const CREATE_FILE_DECISIONS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS file_decisions (
        id TEXT PRIMARY KEY,
        file_path TEXT NOT NULL,
        file_size INTEGER NOT NULL,
        category TEXT NOT NULL,
        file_age_days INTEGER NOT NULL,
        decision TEXT NOT NULL,
        timestamp DATETIME NOT NULL,
        ai_score REAL
    )
";

pub const CREATE_SNAPSHOTS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS snapshots (
        id TEXT PRIMARY KEY,
        timestamp DATETIME NOT NULL,
        files_count INTEGER NOT NULL,
        total_size INTEGER NOT NULL,
        restore_count INTEGER DEFAULT 0
    )
";

pub const CREATE_ANALYTICS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS analytics (
        date DATE PRIMARY KEY,
        space_freed INTEGER DEFAULT 0,
        scans_count INTEGER DEFAULT 0,
        cleanups_count INTEGER DEFAULT 0
    )
";

// Indexes for performance
pub const CREATE_INDEXES: &[&str] = &[
    "CREATE INDEX IF NOT EXISTS idx_scans_timestamp ON scans(timestamp)",
    "CREATE INDEX IF NOT EXISTS idx_cleanups_timestamp ON cleanups(timestamp)",
    "CREATE INDEX IF NOT EXISTS idx_file_decisions_category ON file_decisions(category)",
];
