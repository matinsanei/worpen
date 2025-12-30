use std::fmt::Write;

/// Base64 encode a string
pub fn base64_encode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut result = String::new();
    
    // Simple base64 encoding
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3f) as usize;
        
        write!(&mut result, "{}", CHARSET[b1] as char).unwrap();
        write!(&mut result, "{}", CHARSET[b2] as char).unwrap();
        
        if chunk.len() > 1 {
            write!(&mut result, "{}", CHARSET[b3] as char).unwrap();
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            write!(&mut result, "{}", CHARSET[b4] as char).unwrap();
        } else {
            result.push('=');
        }
    }
    
    result
}

/// Base64 decode a string
pub fn base64_decode(input: &str) -> Result<String, String> {
    // Simple base64 decoding (for demo purposes)
    // In production, use base64 crate
    let input = input.trim_end_matches('=');
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut bytes = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    
    for chunk in chars.chunks(4) {
        let mut values = Vec::new();
        for &ch in chunk {
            let val = charset.find(ch)
                .ok_or_else(|| format!("Invalid base64 character: {}", ch))?;
            values.push(val as u8);
        }
        
        while values.len() < 4 {
            values.push(0);
        }
        
        bytes.push((values[0] << 2) | (values[1] >> 4));
        if chunk.len() > 2 {
            bytes.push((values[1] << 4) | (values[2] >> 2));
        }
        if chunk.len() > 3 {
            bytes.push((values[2] << 6) | values[3]);
        }
    }
    
    String::from_utf8(bytes).map_err(|_| "Invalid UTF-8".to_string())
}

/// URL encode a string
pub fn url_encode(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}

/// URL decode a string
pub fn url_decode(input: &str) -> Result<String, String> {
    let bytes = input.as_bytes();
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                result.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i+1..i+3])
                    .map_err(|_| "Invalid UTF-8")?;
                let byte = u8::from_str_radix(hex, 16)
                    .map_err(|_| "Invalid hex")?;
                result.push(byte);
                i += 3;
            }
            _ => {
                result.push(bytes[i]);
                i += 1;
            }
        }
    }
    
    String::from_utf8(result).map_err(|_| "Invalid UTF-8".to_string())
}

/// Slugify a string (convert to URL-friendly slug)
pub fn slugify(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            ' ' | '_' | '-' => '-',
            _ => '-',
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Escape HTML entities
pub fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Unescape HTML entities
pub fn html_unescape(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
}
