use flashlight::{create_varint, decode_varint};

#[test]
fn test_create_varint() {
    assert_eq!(create_varint(0), vec![0]);
    assert_eq!(create_varint(1), vec![1]);
    assert_eq!(create_varint(2), vec![2]);
    assert_eq!(create_varint(127), vec![127]);
    assert_eq!(create_varint(128), vec![128, 1]);
    assert_eq!(create_varint(255), vec![255, 1]);
    assert_eq!(create_varint(25565), vec![221, 199, 1]);
    assert_eq!(create_varint(2097151), vec![255, 255, 127]);
    assert_eq!(create_varint(2147483647), vec![255, 255, 255, 255, 7]);
    assert_eq!(create_varint(-1), vec![255, 255, 255, 255, 15]);
    assert_eq!(create_varint(-2147483648), vec![128, 128, 128, 128, 8]);
}

#[test]
fn test_decode_varint() {
    let t : Vec<u8> = vec![0];
    assert_eq!(decode_varint(&t), 0);
    let t : Vec<u8> = vec![1];
    assert_eq!(decode_varint(&t), 1);
    let t : Vec<u8> = vec![2];
    assert_eq!(decode_varint(&t), 2);
    let t : Vec<u8> = vec![127];
    assert_eq!(decode_varint(&t), 127);
    let t : Vec<u8> = vec![128, 1];
    assert_eq!(decode_varint(&t), 128);
    let t : Vec<u8> = vec![255, 1];
    assert_eq!(decode_varint(&t), 255);
    let t : Vec<u8> = vec![221, 199, 1];
    assert_eq!(decode_varint(&t), 25565);
    let t : Vec<u8> = vec![255, 255, 127];
    assert_eq!(decode_varint(&t), 2097151);
    let t : Vec<u8> = vec![255, 255, 255, 255, 7];
    assert_eq!(decode_varint(&t), 2147483647);
    let t : Vec<u8> = vec![255, 255, 255, 255, 15];
    assert_eq!(decode_varint(&t), -1);
    let t : Vec<u8> = vec![128, 128, 128, 128, 8];
    assert_eq!(decode_varint(&t), -2147483648);
}
