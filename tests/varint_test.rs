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

fn test_decode_varint0(t: Vec<u8>, r: i32) {
    let res = decode_varint(&t);
    assert!(res.is_ok());
    let res = res.unwrap();
    assert_eq!(res, r);
}

#[test]
fn test_decode_varint() {
    assert!(decode_varint(&vec![0, 0, 0, 0, 0, 0]).is_err());
    test_decode_varint0(vec![0], 0);
    test_decode_varint0(vec![1], 1);
    test_decode_varint0(vec![2], 2);
    test_decode_varint0(vec![127], 127);
    test_decode_varint0(vec![128, 1], 128);
    test_decode_varint0(vec![255, 1], 255);
    test_decode_varint0(vec![221, 199, 1], 25565);
    test_decode_varint0(vec![255, 255, 127], 2097151);
    test_decode_varint0(vec![255, 255, 255, 255, 7], 2147483647);
    test_decode_varint0(vec![255, 255, 255, 255, 15], -1);
    test_decode_varint0(vec![128, 128, 128, 128, 8], -2147483648);
}
