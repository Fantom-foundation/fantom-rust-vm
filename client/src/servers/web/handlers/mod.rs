#[get("/health")]
pub fn health() -> &'static str {
    "OK"
}

#[get("/account/<account_hash>")]
pub fn account(account_hash: String) -> String {
    account_hash
}

#[get("/block/<hash>")]
pub fn block(hash: String) -> String {
    hash
}

#[get("/blockById/<hash>")]
pub fn block_by_id(hash: String) -> String {
    hash
}

#[get("/accounts")]
pub fn accounts() -> String {
    "OK".to_string()
}

// #[post("/call", format = "application/json", data = "<input>")]
// pub fn call(input: T) -> String {
//   "OK".to_string()
// }

// #[post("/tx", data = "<input>")]
// pub fn tx(input: T) -> String {
//   "OK".to_string()
// }

// #[post("/rawtx", data = "<input>")]
// pub fn rawtx(input: T) -> String {
//   "OK".to_string()
// }

#[get("/transactions/<tx_hash>")]
pub fn transactions(tx_hash: String) -> String {
    tx_hash
}

#[get("/tx/<tx_hash>")]
pub fn get_tx(tx_hash: String) -> String {
    tx_hash
}

#[get("/info")]
pub fn info() -> String {
    "OK".to_string()
}

#[get("/html/info")]
pub fn html_info() -> String {
    "OK".to_string()
}
