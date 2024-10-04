use serde_json::{json, Value};
use sha3::{digest::Update, Digest, Sha3_256};

const SALT: &'static str = env!("SALT");

fn hash(string: String) -> String {
    let mut hasher = Sha3_256::new();
    Update::update(&mut hasher, &(string + SALT).into_bytes());
    let result = hasher.finalize();

    let result = &result[..];
    let result = hex::encode(result);
    return result;
}

pub fn get_leader_board(level_id: &str) -> Value {
    // get leaderboard
    let leader_board = match reqwest::blocking::get(&format!(
        "http://danielsson.pythonanywhere.com/get_result/{level_id}"
    )) {
        Ok(resp) => resp.text().unwrap(),
        Err(resp) => panic!("Err: {}", resp),
    };
    let mut leader_board: Value = serde_json::from_str(&leader_board).unwrap_or(json!([]));
    leader_board
}
pub fn log_result(
    id: &str,
    name: &str,
    time: f64,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let string = id.to_owned() + &time.to_string();
    let h = hash(string);
    let brr = reqwest::blocking::get(&format!(
        "http://danielsson.pythonanywhere.com/log_result/{id}/{name}/{time}/{h}"
    ));
    brr
}
