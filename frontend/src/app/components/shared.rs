//! Shared constants and helpers for task display.

/// Kanban column definitions: (status_key, display_label).
pub const COLUMNS: &[(&str, &str)] = &[
    ("barely_started", "Barely Started"),
    ("kinda_doing_it", "Kinda Doing It"),
    ("almost_done", "Almost Done"),
    ("done_allegedly", "Done (Allegedly)"),
];

/// Tailwind color class for a priority level (1-5).
pub fn priority_color(priority: f64) -> &'static str {
    match priority as i64 {
        1 => "text-gray-600",
        2 => "text-yellow-600",
        3 => "text-orange-500",
        4 => "text-red-500",
        _ => "text-red-400",
    }
}

/// Index of a status in the COLUMNS array (0 if not found).
pub fn column_index(status: &str) -> usize {
    COLUMNS.iter().position(|(s, _)| *s == status).unwrap_or(0)
}

/// Number of flame icons for a priority level (0 for priority 1).
pub fn priority_flame_count(priority: f64) -> i64 {
    let p = priority as i64;
    if p <= 1 { 0 } else { p.min(5) }
}
