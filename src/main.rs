use fltk::{app, button::Button, input::Input, frame::Frame, prelude::*, window::Window,enums::Align};
use web3::types::H256;
use tiny_keccak::{Keccak, Hasher};
use hex::decode;

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 600, 300, "Public Key to Address");

    let  input = Input::new(160, 40, 200, 30, "Public Key:");
    let mut output = Frame::new(160, 80, 200, 30, "")
    .with_align(Align::Wrap|Align::Center);

    let mut but = Button::new(160, 120, 80, 40, "Convert");

    wind.end();
    wind.show();

    but.set_callback(move |_| {
        let public_key = input.value();
        if let Ok(address) = public_key_to_address(&public_key) {
            output.set_label(&format!("Address: {:?}", address));
            println!("Address: {:?}", address);
        } else {
            output.set_label("Invalid Public Key");
        }
    });

    app.run().unwrap();
}

fn public_key_to_address(public_key: &str) -> Result<H256, &'static str> {
    // Check if the public key is valid (should be 130 characters for uncompressed key)
    if public_key.len() != 128 {
        return Err("Invalid length");
    }

    let public_key_bytes = decode(public_key).map_err(|_| "Invalid hex")?;

    // Skip the first byte (0x04) which indicates an uncompressed key
    let hash = keccak256(&public_key_bytes[1..]);

    // Take the last 20 bytes
    let address_bytes = &hash[13..];
    Ok(H256::from_slice(address_bytes))
}

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(data);
    hasher.finalize(&mut output);
    output
}
