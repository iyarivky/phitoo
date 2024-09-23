use crate::Phitoo;

pub fn parse_vless(url: String) -> Phitoo {
    let protocol_end = url.find("://").unwrap_or(0);
    let protocol = if protocol_end > 0 {
        url[..protocol_end].to_string()
    } else {
        "null".to_string()
    };

    let rest = &url[protocol_end + 3..];

    let uuid_end = rest.find('@').unwrap_or(0);
    let uuid = if uuid_end > 0 {
        rest[..uuid_end].to_string()
    } else {
        "null".to_string()
    };

    let password = uuid.clone();

    let after_host = &rest[uuid_end + 1..];
    let host_end = after_host.find(':').unwrap_or(0);
    let host = if host_end > 0 {
        after_host[..host_end].to_string()
    } else {
        "null".to_string()
    };

    let port: u16 = after_host[host_end + 1..]
        .parse()
        .unwrap_or_else(|_| 0);

    Phitoo {
        protocol,
        uuid,
        password,
        host,
        port,
    }
}
