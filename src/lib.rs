pub use str::decode_str;
pub use str::encode_str_from_heap;
pub use str::encode_str_from_slice;
pub use varint::create_varint;
pub use varint::decode_varint;

mod str;
mod varint;
