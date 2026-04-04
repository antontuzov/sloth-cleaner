use std::path::Path;
use rusqlite::Connection;
use super::schema;

/// Run database migrations
pub fn run_migrations(db_path: &Path) -> Result<(), String> {
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    // Create tables
    let tables = [
        schema::CREATE_SCANS_TABLE,
        schema::CREATE_CLEANUPS_TABLE,
        schema::CREATE_USER_PREFERENCES_TABLE,
        schema::CREATE_FILE_DECISIONS_TABLE,
        schema::CREATE_SNAPSHOTS_TABLE,
        schema::CREATE_ANALYTICS_TABLE,
    ];
    
    for table_sql in &tables {
        conn.execute(table_sql, [])
            .map_err(|e| format!("Failed to create table: {}", e))?;
    }
    
    // Create indexes
    for index_sql in schema::CREATE_INDEXES {
        conn.execute(index_sql, [])
            .map_err(|e| format!("Failed to create index: {}", e))?;
    }
    
    log::info!("Database migrations completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_migrations() {
        let temp_db = std::env::temp_dir().join("test_sloth.db");
        let result = run_migrations(&temp_db);
        assert!(result.is_ok());
        
        // Cleanup
        std::fs::remove_file(&temp_db).ok();
    }
}
