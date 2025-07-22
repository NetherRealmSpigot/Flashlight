use crate::{create_varint, decode_varint};

#[repr(C)]
pub struct FlashlightVarint {
    pub val: *mut u8,
    pub len: usize
}

#[repr(C)]
pub struct FlashlightVarintDecodeResult {
    pub val: i32,
    pub res: bool
}

#[no_mangle]
pub extern "C" fn flashlight_create_varint(num: i32) -> FlashlightVarint {
    let mut res = create_varint(num);
    let val = res.as_mut_ptr();
    let len = res.len();
    std::mem::forget(res);
    FlashlightVarint { val, len }
}

#[no_mangle]
unsafe extern "C" fn flashlight_free_varint_object(var: FlashlightVarint) {
    let _ = Vec::from_raw_parts(var.val, var.len, var.len);
}

#[no_mangle]
unsafe extern "C" fn flashlight_decode_varint(var: FlashlightVarint) -> FlashlightVarintDecodeResult {
    let v = Vec::from_raw_parts(var.val, var.len, var.len);
    let res = decode_varint(&v);
    match res {
        Ok(v) => FlashlightVarintDecodeResult { val: v, res: true },
        _ => FlashlightVarintDecodeResult { val: 0, res: false }
    }
}
