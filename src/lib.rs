use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch="wasm32")] {
        use wasm_bindgen::{JsCast, prelude::*};
        use web_sys::{MessageEvent, WebSocket, Worker};
        use common::constants::{ADDRESS, PROTOCOL};

        mod common;
        mod web;
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
            ($($t:tt)*) => (
                #[allow(unused_unsafe)]
                unsafe{log(&format_args!($($t)*).to_string())}
            )
        }

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace=console)]
            pub fn log(s: &str);
        }


        #[wasm_bindgen(start)]
        pub async fn main() -> Result<(), JsValue> {
            console_error_panic_hook::set_once();
            let ws = WebSocket::new(&format!("{}://{}", PROTOCOL, ADDRESS))?;
            ws.set_binary_type(web_sys::BinaryType::Blob);

            let onmessage_cb = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
                if let Ok(buf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                    let array = js_sys::Uint8Array::new(&buf);
                    array.to_vec();
                    // Create Serde object for receiving RtcOffers

                } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {

                }
            }) as Box<dyn FnMut(web_sys::MessageEvent)>);
            ws.set_onmessage(Some(onmessage_cb.as_ref().unchecked_ref()));
            onmessage_cb.forget();

            let ws_cloned = ws.clone();
            let onopen_cb = Closure::wrap(Box::new(move |e: web_sys::ProgressEvent| {
                console_log!("Connection Successful");
            }) as Box<dyn FnMut(web_sys::ProgressEvent)>);
            ws.set_onopen(Some(onopen_cb.as_ref().unchecked_ref()));
            onopen_cb.forget();

            let onerror_cb = Closure::wrap(Box::new(move |e: web_sys::ErrorEvent| {
                panic!("Failed to connect to Websocket Server: {}", e.message());
            }) as Box<dyn FnMut(web_sys::ErrorEvent)>);
            ws.set_onerror(Some(onerror_cb.as_ref().unchecked_ref()));
            onerror_cb.forget();

            Ok(())
        }

        #[wasm_bindgen]
        pub async fn new_lobby(lobby_name: String) -> Result<(), JsValue> {

            Ok(())
        }

        #[wasm_bindgen]
        pub async fn join_lobby(lobby_id: u16) -> Result<(), JsValue> {

            Ok(())
        }
    }
}