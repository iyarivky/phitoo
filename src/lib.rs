mod protocol;

use protocol::{vmess, vless};

pub struct Phitoo<'a> {
    pub protocol: &'a str,
    pub uuid: &'a str,
    pub host: &'a str,
    pub port: u16,
}

impl<'a> Phitoo<'a> {
    pub fn parse(url: &'a str) -> Self {
        let protocol_end = url.find("://").expect("Format URL tidak valid");
        let protocol = &url[..protocol_end];

        match protocol {
            "vless" => vless::parse_vless(url),
            "vmess" => vmess::parse_vmess(url),
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
        let url = Phitoo::parse("vmess://some-vmess-uuid@somehost.com:1234");

        assert_eq!(url.protocol, "vmess");
        assert_eq!(url.uuid, "some-vmess-uuid");
        assert_eq!(url.host, "somehost.com");
        assert_eq!(url.port, 1234);
    }
}
