//! History storage using SQLite

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::path::PathBuf;

/// A single history entry
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub id: i64,
    pub original_prompt: String,
    pub refined_prompt: String,
    pub provider: String,
    pub model: String,
    pub created_at: DateTime<Utc>,
}

/// History database manager
pub struct History {
    conn: Connection,
}

impl History {
    /// Get the database path
    pub fn db_path() -> Result<PathBuf> {
        let dirs = ProjectDirs::from("com", "prep", "prep")
            .context("Could not determine data directory")?;
        let data_dir = dirs.data_dir();
        std::fs::create_dir_all(data_dir)
            .with_context(|| format!("Failed to create data directory: {}", data_dir.display()))?;
        Ok(data_dir.join("history.db"))
    }

    /// Open or create the history database
    pub fn open() -> Result<Self> {
        let path = Self::db_path()?;
        let conn = Connection::open(&path)
            .with_context(|| format!("Failed to open history database: {}", path.display()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                original_prompt TEXT NOT NULL,
                refined_prompt TEXT NOT NULL,
                provider TEXT NOT NULL,
                model TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_created_at ON history(created_at DESC)",
            [],
        )?;

        Ok(Self { conn })
    }

    /// Add a new entry
    pub fn add(
        &self,
        original_prompt: &str,
        refined_prompt: &str,
        provider: &str,
        model: &str,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO history (original_prompt, refined_prompt, provider, model) VALUES (?1, ?2, ?3, ?4)",
            params![original_prompt, refined_prompt, provider, model],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// List recent entries
    pub fn list(&self, limit: usize) -> Result<Vec<HistoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, original_prompt, refined_prompt, provider, model, created_at 
             FROM history 
             ORDER BY created_at DESC 
             LIMIT ?1",
        )?;

        let entries = stmt
            .query_map(params![limit as i64], |row| {
                let created_at_str: String = row.get(5)?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                Ok(HistoryEntry {
                    id: row.get(0)?,
                    original_prompt: row.get(1)?,
                    refined_prompt: row.get(2)?,
                    provider: row.get(3)?,
                    model: row.get(4)?,
                    created_at,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    /// Get a specific entry
    pub fn get(&self, id: i64) -> Result<Option<HistoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, original_prompt, refined_prompt, provider, model, created_at 
             FROM history 
             WHERE id = ?1",
        )?;

        let mut entries = stmt
            .query_map(params![id], |row| {
                let created_at_str: String = row.get(5)?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                Ok(HistoryEntry {
                    id: row.get(0)?,
                    original_prompt: row.get(1)?,
                    refined_prompt: row.get(2)?,
                    provider: row.get(3)?,
                    model: row.get(4)?,
                    created_at,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries.pop())
    }

    /// Search history
    pub fn search(&self, query: &str) -> Result<Vec<HistoryEntry>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, original_prompt, refined_prompt, provider, model, created_at 
             FROM history 
             WHERE original_prompt LIKE ?1 OR refined_prompt LIKE ?1
             ORDER BY created_at DESC 
             LIMIT 50",
        )?;

        let entries = stmt
            .query_map(params![pattern], |row| {
                let created_at_str: String = row.get(5)?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                Ok(HistoryEntry {
                    id: row.get(0)?,
                    original_prompt: row.get(1)?,
                    refined_prompt: row.get(2)?,
                    provider: row.get(3)?,
                    model: row.get(4)?,
                    created_at,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    /// Clear all history
    pub fn clear(&self) -> Result<usize> {
        let count = self.conn.execute("DELETE FROM history", [])?;
        self.conn.execute("VACUUM", [])?;
        Ok(count)
    }

    /// Prune old entries
    pub fn prune(&self, max_entries: usize) -> Result<usize> {
        let count = self.conn.execute(
            "DELETE FROM history WHERE id NOT IN (
                SELECT id FROM history ORDER BY created_at DESC LIMIT ?1
            )",
            params![max_entries as i64],
        )?;
        Ok(count)
    }
}
