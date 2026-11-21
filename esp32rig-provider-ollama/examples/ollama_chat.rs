use esp32rig_provider_ollama::transport::{
    HttpTransport,
    LinuxHttpTransport,
};

const REQUEST: &[u8] = br#"{
    "model": "qwen3:4b",
    "messages": [
        {
            "role": "user",
            "content": "Reply with exactly: ESP32Rig connected"
        }
    ],
    "stream": false
}"#;

fn main() {
    let mut transport = LinuxHttpTransport::new();

    let mut response_buffer = [0u8; 16384];

    println!("Connecting to Ollama...");

    let response = transport
        .post(
            "http://localhost:11434/api/chat",
            REQUEST,
            &mut response_buffer,
        )
        .expect("HTTP request failed");

    println!("HTTP status: {}", response.status);

    let response_body = std::str::from_utf8(
        &response_buffer[..response.body_len],
    )
        .expect("Invalid UTF-8 response");

    println!("Ollama response:");
    println!("{}", response_body);
}