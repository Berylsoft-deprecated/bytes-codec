pub use binrw::binrw as bytes_codec;
use std::io::Cursor;
use binrw::{BinRead, BinWrite, BinReaderExt, BinWriterExt, BinResult};

pub trait BytesCodecExt: BinRead + BinWrite
where
    <Self as BinRead>::Args: Default,
    <Self as BinWrite>::Args: Default,
{
    fn decode<T: AsRef<[u8]>>(bytes: T) -> BinResult<Self> {
        let mut reader = Cursor::new(bytes);
        reader.read_be()
    }

    fn encode(&self) -> BinResult<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut writer = Cursor::new(&mut bytes);
        writer.write_be(&self)?;
        Ok(bytes)
    }
}

impl<T: BinRead + BinWrite> BytesCodecExt for T
where
    <Self as BinRead>::Args: Default,
    <Self as BinWrite>::Args: Default,
{}

#[cfg(test)]
mod test {
    use hex_literal::hex;
    use super::*;

    #[bytes_codec]
    struct Example {
        a: u64,
        b: u32,
        c: u16,
        d: [u8; 4],
    }

    const BYTES: [u8; 18] = hex!("0000 017C A687 618D  2232 75F6  5F49  00 01 02 FF");
    const A: u64 = 1634881462669;
    const B: u32 = 573732342;
    const C: u16 = 24393;
    const D: [u8; 4] = [0, 1, 2, 255];

    #[test]
    fn test_decode() {
        let example = Example::decode(BYTES).unwrap();
        assert_eq!(example.a, A);
        assert_eq!(example.b, B);
        assert_eq!(example.c, C);
        assert_eq!(example.d, D);
    }

    #[test]
    fn test_encode() {
        let example = Example { a: A, b: B, c: C, d: D };
        assert_eq!(example.encode().unwrap(), BYTES);
    }
}
