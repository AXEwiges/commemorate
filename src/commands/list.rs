use chrono::{DateTime, Utc};
use prettytable::{Cell, Row, Table};

use crate::error::CommemorateResult;
use crate::utils::list_memoria_files;

pub fn list_events() -> CommemorateResult<()> {
    let memoria_files = list_memoria_files()?;

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Event Name"),
        Cell::new("Last Modified"),
    ]));

    for path in memoria_files {
        let file_name = path.file_stem().unwrap().to_string_lossy();
        let last_modified = path
            .metadata()
            .and_then(|m| m.modified())
            .map(|time| {
                let datetime: DateTime<Utc> = time.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            })
            .unwrap_or_else(|_| "Unknown".to_string());

        table.add_row(Row::new(vec![
            Cell::new(&file_name),
            Cell::new(&last_modified),
        ]));
    }

    table.printstd();
    Ok(())
}
