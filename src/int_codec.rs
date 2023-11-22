use std::io::Read;
use std::{io, str};
use tokio_util::bytes::{BytesMut, BufMut, Buf};
use tokio_util::codec::{Encoder, Decoder};


pub struct IntCodec;

impl Decoder for IntCodec {
    type Item = u16;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        debug!("decode: raw bytes: {src:#?}");
        if src.len() == 0 {
            return Ok(None)
        }

        let mut buf = [0u8;2];
        if let Err(e) = src.reader().read_exact(&mut buf) {
            warn!("IntCodec::decode error: {e} ");
            return Err(e);
        }
        let val = (buf[0] as u16) << 8 | buf[1] as u16;
        Ok(Some(val))
    }
}

impl Encoder<String> for IntCodec {
    type Error = io::Error;
    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(item.as_bytes());
        Ok(())
    }
}
impl Encoder<&str> for IntCodec {
    type Error = io::Error;
    fn encode(&mut self, item: &str, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(item.as_bytes());
        Ok(())
    }
}
