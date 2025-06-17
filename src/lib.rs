mod str;
mod varint;

pub use str::encode_from_slice;
pub use str::encode_from_heap;
pub use str::decode;

pub use varint::create_varint;
pub use varint::decode_varint;