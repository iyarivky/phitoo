mod protocol;

use protocol::{vmess, vless};

pub struct Phitoo {
    pub protocol: String,
    pub uuid: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

impl Phitoo {
    pub fn parse(url: &str) -> Self {
        let protocol_end = url.find("://").expect("Format URL tidak valid");
        let protocol = url[..protocol_end].to_string();  // Convert to String

        match protocol.as_str() {
            "vless" => vless::parse_vless(url.to_string()),
            "vmess" => vmess::parse_vmess(url.to_string()),
            _ => panic!("Protokol tidak didukung: {}", protocol),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vless_url() {
        let url = Phitoo::parse("vless://fe841e7b-7631-4a44-b529-6472a1198202@quiz.int.vidio.com:443");

        assert_eq!(url.protocol, "vless");
        assert_eq!(url.uuid, "fe841e7b-7631-4a44-b529-6472a1198202");
        assert_eq!(url.host, "quiz.int.vidio.com");
        assert_eq!(url.port, 443);
    }

    #[test]
    fn test_parse_vmess_url() {
        let url = Phitoo::parse("vmess://eyJ2IjoiMiIsInBzIjoiYWt1bnNzaC1idW1pIiwiYWRkIjoic2cxLXZtZXNzLmRuc3gubXkuaWQiLCJwb3J0IjoiNDQzIiwiaWQiOiIxYjMxY2IyZS03OTA0LTExZWYtOTUzOC00ZjQ4YzRhZDE5MzciLCJhaWQiOiIwIiwibmV0Ijoid3MiLCJzY3kiOiJhdXRvIiwic25pIjoiIiwidHlwZSI6IiIsImhvc3QiOiJzZzEtdm1lc3MuZG5zeC5teS5pZCIsInBhdGgiOiIvdm1lc3MiLCJ0bHMiOiJ0bHMifQ==");

        assert_eq!(url.protocol, "vmess");
        assert_eq!(url.uuid, "1b31cb2e-7904-11ef-9538-4f48c4ad1937");
        assert_eq!(url.host, "sg1-vmess.dnsx.my.id");
        assert_eq!(url.port, 443);
    }
}
