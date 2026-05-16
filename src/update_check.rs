// One-shot update check.
//
// `check_for_update()` runs on a background thread (joined inside an
// async wrapper, mirroring how the rest of `crate::io` wraps blocking
// work for iced's `Task::perform`). It hits the GitHub releases API and
// returns `Some(latest_version)` when a newer release is available, or
// `None` when up to date / on network failure / on parse error.

const RELEASES_API: &str =
    "https://api.github.com/repos/HakanSeven12/H7CAD/releases/latest";
pub const RELEASES_PAGE: &str =
    "https://github.com/HakanSeven12/H7CAD/releases/latest";

pub async fn check_for_update() -> Option<String> {
    std::thread::spawn(fetch_latest_if_outdated)
        .join()
        .ok()
        .flatten()
}

fn fetch_latest_if_outdated() -> Option<String> {
    let body = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .get(RELEASES_API)
        .set("User-Agent", concat!("h7cad/", env!("CARGO_PKG_VERSION")))
        .set("Accept", "application/vnd.github+json")
        .call()
        .ok()?
        .into_string()
        .ok()?;
    let latest = extract_tag(&body)?
        .trim_start_matches('v')
        .to_string();
    if latest != env!("CARGO_PKG_VERSION") {
        Some(latest)
    } else {
        None
    }
}

fn extract_tag(body: &str) -> Option<String> {
    // Minimal substring extraction so we don't pull serde_json just for
    // one field. The GitHub releases endpoint always returns
    // `"tag_name":"v…"` near the top of the JSON object.
    const KEY: &str = "\"tag_name\":\"";
    let start = body.find(KEY)? + KEY.len();
    let end = body[start..].find('"')? + start;
    Some(body[start..end].to_string())
}
