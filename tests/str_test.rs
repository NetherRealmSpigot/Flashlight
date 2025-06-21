use flashlight::{decode_str, encode_str_from_heap, encode_str_from_slice};

#[test]
fn test_encode() {
    let r1 = encode_str_from_slice("Hello world!", &0x11, &0x17);
    let a1: Vec<u8> = vec![77, 95, 38, 85, 76, 52, 225, 10, 220, 237, 243, 165];
    assert_eq!(r1, a1);

    let r2 = encode_str_from_heap(&String::from("Hello world!"), &0x10, &0x16);
    let a2: Vec<u8> = vec![67, 94, 42, 90, 70, 57, 233, 1, 202, 228, 231, 210];
    assert_eq!(r2, a2);

    let r3 = encode_str_from_slice("Hello world", &0x11, &0x17);
    let a3: Vec<u8> = vec![77, 95, 38, 85, 76, 52, 225, 10, 220, 237, 165];
    assert_eq!(r3, a3);

    let r4 = encode_str_from_heap(&String::from("Hello world"), &0x10, &0x16);
    let a4: Vec<u8> = vec![67, 94, 42, 90, 70, 57, 233, 1, 202, 228, 210];
    assert_eq!(r4, a4);
}

#[test]
fn test_decode() {
    let t1: Vec<u8> = vec![77, 95, 38, 85, 76, 52, 225, 10, 220, 237, 243, 165];
    let r1 = decode_str(&t1, &0x11, &0x17);
    assert_eq!("Hello world!", r1);

    let t2: Vec<u8> = vec![67, 94, 42, 90, 70, 57, 233, 1, 202, 228, 231, 210];
    let r2 = decode_str(&t2, &0x10, &0x16);
    assert_eq!("Hello world!", r2);

    let t3: Vec<u8> = vec![77, 95, 38, 85, 76, 52, 225, 10, 220, 237, 165];
    let r3 = decode_str(&t3, &0x11, &0x17);
    assert_eq!("Hello world", r3);

    let t4: Vec<u8> = vec![67, 94, 42, 90, 70, 57, 233, 1, 202, 228, 210];
    let r4 = decode_str(&t4, &0x10, &0x16);
    assert_eq!("Hello world", r4);
}