use std::process::Command;

use chrono::Local;

pub struct DatabaseBackup;

impl DatabaseBackup {
    pub fn backup_insert(self: Self, database_name: &str, backup_path: &str) {
        let output = Command::new("pg_dump")
            .arg(database_name)
            .arg(format!(
                "--file={0}_{1}.sql",
                backup_path,
                Local::now().format("%Y%m%d_%H%M%S")
            ))
            .arg("--data-only")
            .arg("--encoding=UTF-8")
            .arg("--format=plain")
            .arg("--column-inserts")
            .arg("--no-sync")
            .arg("--exclude-table=_sqlx_migrations")
            .arg("--exclude-table=listen_flow")
            .arg("--exclude-table=security_temp")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Output: {}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {}", stderr);
        }
    }

    pub fn backup_copy(self: Self, database_name: &str, backup_path: &str) {
        let output = Command::new("pg_dump")
            .arg(database_name)
            .arg(format!(
                "--file={0}_{1}.sql",
                backup_path,
                Local::now().format("%Y%m%d_%H%M%S")
            ))
            .arg("--data-only")
            .arg("--encoding=UTF-8")
            .arg("--format=plain")
            .arg("--no-sync")
            .arg("--exclude-table=_sqlx_migrations")
            .arg("--exclude-table=listen_flow")
            .arg("--exclude-table=security_temp")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Output: {}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {}", stderr);
        }
    }
}
