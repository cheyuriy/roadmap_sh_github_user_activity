use reqwest::blocking::Client;
use crate::models::Event;

pub fn get_events(username: &str) -> Result<Vec<Event>, reqwest::Error> {
    let url = format!("https://api.github.com/users/{}/events", username);
    let client = Client::new();
    let req = client
        .get(&url)
        .header("User-Agent", "reqwest")
        .header("Accept", "application/vnd.github.v3+json")
        .send()?;

    if req.status().is_success() {
        let events: Vec<Event> = req.json()?;
        return Ok(events);
    } else {
        return Err(req.error_for_status().unwrap_err());
    }
}