use binrw::{BinRead, BinWrite, BinReaderExt, BinWriterExt, BinResult, io::Cursor};

pub trait BytesDecodeExt: BinRead
where
    <Self as BinRead>::Args: Default,
{
    fn decode<T: AsRef<[u8]>>(bytes: T) -> BinResult<Self> {
        let mut reader = Cursor::new(bytes);
        reader.read_be()
    }
}

pub trait BytesEncodeExt: BinWrite
where
    <Self as BinWrite>::Args: Default,
{
    fn encode(&self) -> BinResult<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut writer = Cursor::new(&mut bytes);
        writer.write_be(&self)?;
        Ok(bytes)
    }
}

impl<T: BinRead> BytesDecodeExt for T
where
    <Self as BinRead>::Args: Default,
{}

impl<T: BinWrite> BytesEncodeExt for T
where
    <Self as BinWrite>::Args: Default,
{}

#[cfg(test)]
mod test {
    use hex_literal::hex;
    use binrw::{BinRead, BinWrite};
    use super::{BytesDecodeExt, BytesEncodeExt};

    #[derive(BinRead, BinWrite)]
    struct Example {
        a: u64,
        b: u32,
        c: u16,
        d: [u8; 4],
        e: i32,
    }

    const BYTES: [u8; 22] = hex!("0000 017C A687 618D  2232 75F6  5F49  00 01 02 FF  FA45 D360");
    const A: u64 = 1634881462669;
    const B: u32 = 573732342;
    const C: u16 = 24393;
    const D: [u8; 4] = [0, 1, 2, 255];
    const E: i32 = -96087200;

    #[test]
    fn test_decode() {
        let example = Example::decode(BYTES).unwrap();
        assert_eq!(example.a, A);
        assert_eq!(example.b, B);
        assert_eq!(example.c, C);
        assert_eq!(example.d, D);
        assert_eq!(example.e, E);
    }

    #[test]
    fn test_encode() {
        let example = Example { a: A, b: B, c: C, d: D, e: E };
        assert_eq!(example.encode().unwrap(), BYTES);
    }
}
