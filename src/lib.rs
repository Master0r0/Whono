use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{MessageEvent, WebSocket, Worker};
use common::constants::{ADDRESS, PROTOCOL};

mod common;
mod game;

/*
* Possibly use a webworker for managing the entire game, as a host with webrtc and websocket connection
* the host client connects to the webworker via webrtc like any other client but no need to
* communicate their signal over websocket
* All other clients connect to the webworker via webrtc via signals over websocket
* Should allow for host client to just act exclusively as a client
*/

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (unsafe{log(&format_args!($($t)*).to_string())})
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: &str);
}


#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub async fn new_lobby() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub async fn join_lobby(lobby_id: u16) -> Result<(), JsValue> {

    Ok(())
}