mod str;
mod varint;

pub use str::encode_str_from_slice;
pub use str::encode_str_from_heap;
pub use str::decode_str;

pub use varint::create_varint;
pub use varint::decode_varint;