pub fn create_varint(num: i32) -> Vec<u8> {
    let mut v = num;
    let mut buf : Vec<u8> = Vec::with_capacity(5usize);
    for _ in 0..5 {
        if v & (!0x7Fi32) == 0 {
            buf.push(v as u8);
            break
        }
        buf.push(((v & 0x7Fi32) | 0x80i32) as u8);
        v = ((v as u32) >> 7) as i32;
    }
    buf
}

pub fn decode_varint(q: &[u8]) -> i32 {
    let len = q.len();
    let mut res = 0i32;
    for i in 0..len {
        res = res | (((q[i] as i32) & 0x7Fi32) << (7 * i));
        if ((q[i] as i32) & 0x80i32) == 0 {
            break
        }
    }
    res
}
