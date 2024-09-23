use base64::engine::general_purpose;
use base64::Engine as _;
use serde::{Deserialize, Serialize};
use crate::Phitoo;

#[derive(Serialize, Deserialize, Debug)]
struct VmessConfig {
    v: String,
    ps: String,
    add: String,
    port: String,
    id: String,
    aid: String,
    net: String,
    scy: String,
    sni: String,
    #[serde(rename = "type")]
    typ: String,
    host: String,
    path: String,
    tls: String,
}

pub fn parse_vmess(url: String) -> Phitoo {
    let base64_data = &url[8..];

    let decoded_data = general_purpose::STANDARD.decode(base64_data).unwrap();
    let json_data = String::from_utf8(decoded_data).unwrap();

    let config: VmessConfig = serde_json::from_str(&json_data).unwrap();

    let protocol = "vmess".to_string();
    let uuid = config.id;
    let password = uuid.clone();
    let host = config.add;
    let port: u16 = config.port.parse().unwrap();

    Phitoo {
        protocol,
        uuid,
        password,
        host,
        port,
    }
}
