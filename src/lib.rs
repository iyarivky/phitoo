pub struct Phitoo<'a> {
    pub protocol: &'a str,
    pub uuid: &'a str,
    pub host: &'a str,
    pub port: u16,
}

impl<'a> Phitoo<'a> {
    /// Fungsi untuk parsing URL vless secara lebih cepat
    pub fn parse(url: &'a str) -> Self {
        // Mencari batasan untuk protocol, UUID, host, dan port
        let protocol_end = url.find("://").expect("Invalid URL format");
        let protocol = &url[..protocol_end];

        let rest = &url[protocol_end + 3..];
        let uuid_end = rest.find('@').expect("Invalid URL format");
        let uuid = &rest[..uuid_end];

        let after_host = &rest[uuid_end + 1..];
        let host_end = after_host.find(':').expect("Invalid URL format");
        let host = &after_host[..host_end];

        let after_port = &after_host[host_end + 1..];
        let port_end = after_port.find('?').unwrap_or_else(|| after_port.len());
        let port: u16 = after_port[..port_end].parse().expect("Invalid port");

        Phitoo {
            protocol,
            uuid,
            host,
            port,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vless_url() {
        let url = Phitoo::parse("vless://fe841e7b-7631-4a44-b529-6472a1198202@quiz.int.vidio.com:443");

        // Test akses langsung ke field
        assert_eq!(url.protocol, "vless");
        assert_eq!(url.uuid, "fe841e7b-7631-4a44-b529-6472a1198202");
        assert_eq!(url.host, "quiz.int.vidio.com");
        assert_eq!(url.port, 443);
    }
}
