use crate::Phitoo;

pub fn parse_vless(url: &str) -> Phitoo {
    let protocol_end = url.find("://").unwrap();
    let protocol = &url[..protocol_end];

    let rest = &url[protocol_end + 3..];
    let uuid_end = rest.find('@').unwrap();
    let uuid = &rest[..uuid_end];

    let after_host = &rest[uuid_end + 1..];
    let host_end = after_host.find(':').unwrap();
    let host = &after_host[..host_end];

    let after_port = &after_host[host_end + 1..];
    let port_end = after_port.find('?').unwrap_or_else(|| after_port.len());
    let port: u16 = after_port[..port_end].parse().unwrap();

    Phitoo {
        protocol,
        uuid,
        host,
        port,
    }
}
