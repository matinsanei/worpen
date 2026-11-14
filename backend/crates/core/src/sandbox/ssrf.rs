use proto::models::SandboxPolicy;
use std::net::{IpAddr, SocketAddr};

/// Returns true if the IP address belongs to private, loopback, link-local, multicast,
/// CGNAT, or otherwise reserved/non-routable address ranges.
pub fn is_blocked_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            if ipv4.is_loopback()
                || ipv4.is_private()
                || ipv4.is_link_local()
                || ipv4.is_broadcast()
                || ipv4.is_unspecified()
                || ipv4.is_multicast()
            {
                return true;
            }
            let octets = ipv4.octets();
            // Carrier-Grade NAT (RFC 6598: 100.64.0.0/10 -> 100.64.0.0 to 100.127.255.255)
            if octets[0] == 100 && (64..=127).contains(&octets[1]) {
                return true;
            }
            // Reserved / Future use (240.0.0.0/4)
            if octets[0] >= 240 {
                return true;
            }
            false
        }
        IpAddr::V6(ipv6) => {
            if ipv6.is_loopback() || ipv6.is_unspecified() || ipv6.is_multicast() {
                return true;
            }
            // Check IPv4-mapped IPv6 addresses (e.g. ::ffff:127.0.0.1 or ::ffff:10.0.0.1)
            if let Some(ipv4) = ipv6.to_ipv4_mapped() {
                return is_blocked_ip(IpAddr::V4(ipv4));
            }
            let segments = ipv6.segments();
            // Unique Local Address (RFC 4193: fc00::/7 -> 0xfc00..=0xfdff)
            if (segments[0] & 0xfe00) == 0xfc00 {
                return true;
            }
            // Link-Local Unicast (RFC 4291: fe80::/10 -> 0xfe80..=0xfebf)
            if (segments[0] & 0xffc0) == 0xfe80 {
                return true;
            }
            false
        }
    }
}

/// Parse, check allowlist, and resolve a target URL under the given sandbox policy.
/// Returns `(host, port, pinned_ip)`.
pub async fn resolve_and_validate_host(
    url_str: &str,
    policy: &SandboxPolicy,
) -> Result<(String, u16, IpAddr), String> {
    if !policy.allow_net {
        return Err("sandbox: network access denied (allow_net=false)".to_string());
    }

    let parsed = reqwest::Url::parse(url_str)
        .map_err(|e| format!("sandbox: invalid URL '{}': {}", url_str, e))?;

    let host = parsed
        .host_str()
        .ok_or_else(|| format!("sandbox: missing host in URL '{}'", url_str))?
        .to_string();

    let port = parsed.port_or_known_default().unwrap_or(80);

    // 1. Host allowlist check (exact or subdomain suffix)
    if !policy.is_host_allowed(&host) {
        return Err(format!("sandbox: host '{}' denied by allowlist", host));
    }

    // 2. SSRF Private IP check
    if policy.block_private_ips {
        // If host is directly an IP literal
        if let Ok(ip) = host.parse::<IpAddr>() {
            if is_blocked_ip(ip) {
                return Err(format!(
                    "sandbox: SSRF blocked: host '{}' is a private/blocked IP",
                    ip
                ));
            }
            return Ok((host, port, ip));
        }

        // If domain name, resolve DNS asynchronously
        let host_port = format!("{}:{}", host, port);
        let addrs: Vec<SocketAddr> = tokio::net::lookup_host(&host_port)
            .await
            .map_err(|e| format!("sandbox: DNS lookup failed for '{}': {}", host, e))?
            .collect();

        if addrs.is_empty() {
            return Err(format!("sandbox: DNS returned no IP for '{}'", host));
        }

        for addr in &addrs {
            let ip = addr.ip();
            if is_blocked_ip(ip) {
                return Err(format!(
                    "sandbox: SSRF blocked: host '{}' resolves to private/blocked IP '{}'",
                    host, ip
                ));
            }
        }

        return Ok((host, port, addrs[0].ip()));
    }

    // Fallback if private IPs are explicitly allowed
    let host_port = format!("{}:{}", host, port);
    let resolved_ip = match host.parse::<IpAddr>() {
        Ok(ip) => ip,
        Err(_) => {
            let addrs: Vec<SocketAddr> = tokio::net::lookup_host(&host_port)
                .await
                .map_err(|e| format!("sandbox: DNS lookup failed for '{}': {}", host, e))?
                .collect();
            if addrs.is_empty() {
                return Err(format!("sandbox: DNS returned no IP for '{}'", host));
            }
            addrs[0].ip()
        }
    };

    Ok((host, port, resolved_ip))
}
