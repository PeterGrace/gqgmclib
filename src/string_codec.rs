use std::{io, str};
use tokio_util::bytes::{BytesMut, BufMut, Buf};
use tokio_util::codec::{Encoder, Decoder};


pub struct StringCodec;

impl Decoder for StringCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() == 0 {
            return Ok(None)
        }
        debug!("decode: raw bytes: {src:#?}");
        let resp = match str::from_utf8(src.as_ref()) {
            Ok(s) => {
                s.to_string()
            },
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::Other, "Invalid String"));

            }
        };
        src.advance(resp.len());

        Ok(Some(resp))

    }
}

impl Encoder<String> for StringCodec {
    type Error = io::Error;
    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(item.as_bytes());
        Ok(())
    }
}

impl Encoder<&str> for StringCodec {
    type Error = io::Error;
    fn encode(&mut self, item: &str, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(item.as_bytes());
        Ok(())
    }
}
