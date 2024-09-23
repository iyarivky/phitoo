use crate::Phitoo;

pub fn parse_vless(url: String) -> Phitoo {
    let protocol_end = url.find("://").unwrap();
    let protocol = url[..protocol_end].to_string();

    let rest = &url[protocol_end + 3..];
    let uuid_end = rest.find('@').unwrap();
    let uuid = rest[..uuid_end].to_string();

    let password = uuid.clone();

    let after_host = &rest[uuid_end + 1..];
    let host_end = after_host.find(':').unwrap();
    let host = after_host[..host_end].to_string();

    let after_port = &after_host[host_end + 1..];
    let port: u16 = after_port.parse().unwrap();

    Phitoo {
        protocol,
        uuid,
        password,
        host,
        port,
    }
}
