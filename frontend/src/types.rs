//! Re-exports generated types and adds frontend-specific helpers.

pub use crate::generated::{
    AppSettings, Categories, Comments, Media, Rel, Tasks, TasksSubtasks, Users,
};

/// Auth token extracted from the request cookie, provided via Leptos context
/// so server functions can access it during SSR.
#[derive(Debug, Clone, Default)]
pub struct AuthToken(pub Option<String>);

// -- Rel helpers -----------------------------------------------------

/// Get a string field from a populated `Rel<T>` via an accessor function.
pub fn rel_field<T>(rel: &Option<Rel<T>>, f: impl FnOnce(&T) -> String) -> Option<String> {
    rel.as_ref().and_then(|r| r.as_doc()).map(f)
}

/// Get an optional string field from a populated `Rel<T>`.
pub fn rel_field_opt<T>(
    rel: &Option<Rel<T>>,
    f: impl FnOnce(&T) -> Option<String>,
) -> Option<String> {
    rel.as_ref().and_then(|r| r.as_doc()).and_then(f)
}

/// Get the raw ID from a `Rel<T>` (unpopulated form).
pub fn rel_id<T>(rel: &Option<Rel<T>>) -> Option<String> {
    rel.as_ref().and_then(|r| match r {
        Rel::Id(id) => Some(id.clone()),
        Rel::Doc(_) => None,
    })
}

/// Get the name from a populated user relation.
pub fn rel_user_name(rel: &Option<Rel<Users>>) -> Option<String> {
    rel_field(rel, |u| u.name.clone())
}

/// Get the name from a populated category relation.
pub fn rel_category_name(rel: &Option<Rel<Categories>>) -> Option<String> {
    rel_field(rel, |c| c.name.clone())
}

/// Get the color hex from a populated category relation.
pub fn rel_category_color(rel: &Option<Rel<Categories>>) -> Option<String> {
    rel_field_opt(rel, |c| c.color.clone())
}

// -- Date formatting -------------------------------------------------

/// Format a date string into a locale-aware short format (e.g. "Apr 17").
pub fn format_date(raw: &str) -> String {
    #[cfg(feature = "hydrate")]
    return format_date_wasm(raw);

    #[cfg(not(feature = "hydrate"))]
    return format_date_ssr(raw);
}

#[cfg(feature = "hydrate")]
fn format_date_wasm(raw: &str) -> String {
    use wasm_bindgen::JsValue;
    use web_sys::js_sys;

    let date_str = if raw.len() == 10 {
        format!("{raw}T00:00:00")
    } else {
        raw.to_string()
    };

    let date = js_sys::Date::new(&JsValue::from_str(&date_str));

    if date.get_time().is_nan() {
        return raw.to_string();
    }

    let options = js_sys::Object::new();
    js_sys::Reflect::set(&options, &"month".into(), &"short".into()).ok();
    js_sys::Reflect::set(&options, &"day".into(), &"numeric".into()).ok();

    let formatter = js_sys::Intl::DateTimeFormat::new(&js_sys::Array::new(), &options);

    formatter
        .format()
        .call1(&JsValue::UNDEFINED, &date)
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| raw.to_string())
}

#[cfg(not(feature = "hydrate"))]
fn format_date_ssr(raw: &str) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let date_part = &raw[..raw.len().min(10)];
    let parts: Vec<&str> = date_part.split('-').collect();

    if parts.len() != 3 {
        return raw.to_string();
    }

    let month_idx: usize = parts[1].parse().unwrap_or(1);
    let day: u32 = parts[2].parse().unwrap_or(1);
    let month = MONTHS.get(month_idx.wrapping_sub(1)).unwrap_or(&"???");

    format!("{month} {day}")
}
