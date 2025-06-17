const MAGIC: u32 = 0x100;

pub fn decode_str(q: &[u8], k1: u8, k2: u8) -> String {
    let mut s = String::new();
    let mut i = 0usize;
    let len = q.len();
    while i < len {
        let p = i as u32;
        if i + 1 < len {
            let x = q[i];
            let y = q[i + 1];
            let k = (p * k1 as u32 + k2 as u32) % MAGIC;
            let l = ((p + 1) * k1 as u32 + k2 as u32) % MAGIC;
            s.push((y ^ k as u8) as char);
            s.push((x ^ l as u8) as char);
            i += 2;
        } else {
            let k = (p * k1 as u32 + k2 as u32) % MAGIC;
            s.push((q[i] ^ k as u8) as char);
            i += 1;
        }
    }
    s
}

pub fn encode_str_from_slice(s: &str, k1: u8, k2: u8) -> Vec<u8> {
    encode_str_from_heap(String::from(s), k1, k2)
}

pub fn encode_str_from_heap(s: String, k1: u8, k2: u8) -> Vec<u8> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut q = Vec::with_capacity(len);
    for i in 0..(len / 2) {
        let j1 = 2 * i;
        let j2 = 2 * i + 1;
        let k_j1 = ((j1 as u32 * k1 as u32 + k2 as u32) % MAGIC) as u8;
        let k_j2 = ((j2 as u32 * k1 as u32 + k2 as u32) % MAGIC) as u8;
        q.push(bytes[j2] ^ k_j2);
        q.push(bytes[j1] ^ k_j1);
    }
    if len % 2 == 1 {
        let j = len - 1;
        let k_j = ((j as u32 * k1 as u32 + k2 as u32) % MAGIC) as u8;
        q.push(bytes[j] ^ k_j);
    }
    q
}
