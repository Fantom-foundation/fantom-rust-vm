
#[get("/health")]
pub fn health() -> &'static str {
    "OK"
}

#[get("/account/<account_hash>")]
pub fn account(account_hash: String) -> String {
  "OK"
}

#[get("/block/<hash>")]
pub fn block(block_hash: String) -> String {
  "OK"
}

#[get("/blockById/<hash>")]
pub fn block_by_id(block_hash: String) -> String {
  "OK"
}

#[get("/accounts")]
pub fn accounts(input: T) -> String {
  "OK"
}

#[post("/call", data = "<input>")]
pub fn call(data: T) -> String {
  "OK"
}

#[post("/tx", data = "<input>")]
pub fn tx(data: T) -> String {
  "OK"
}

#[post("/rawtx", data = "<input>")]
pub fn rawtx(data: T) -> String {
  "OK"
}

#[get("/transactions/<tx_hash>")]
pub fn transactions(tx_hash: String) -> String {
  "OK"
}

#[get("/tx/<tx_hash>")]
pub fn get_tx(tx_hash: String) -> String {
  "OK"
}

#[get("/info")]
pub fn info() -> String {
  "OK"
}

#[get("/html/info")]
pub fn html_info() -> String {
  "OK"
}
