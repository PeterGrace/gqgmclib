#[macro_use]
extern crate tracing;
#[macro_use]
extern crate thiserror;

pub mod string_codec;
pub mod gqgmcerror;
pub mod consts;
pub mod int_codec;

use crate::int_codec::IntCodec;
use futures::{SinkExt, TryStreamExt};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::codec::Decoder;
use string_codec::StringCodec;
use crate::consts::{FUNC_GET_CPM, FUNC_GET_VERSION, FUNC_POWERON};
use crate::gqgmcerror::GQGMCError;

pub struct GMC {
    stream: SerialStream,
}

impl GMC {
    pub fn new(port: &str, baud: u32) -> Result<Self, GQGMCError> {
        let stream = match tokio_serial::new(port, baud).open_native_async() {
            Ok(port) => port,
            Err(e) => {
                return Err(GQGMCError::Miscellaneous(e.to_string()));
            }
        };
        Ok(GMC { stream })
    }
    pub async fn get_version(&mut self) -> Result<String, GQGMCError> {
        let mut frame = StringCodec.framed(&mut self.stream);
        frame.send(FUNC_GET_VERSION.to_string()).await.expect("Couldn't write message");
        loop {
            match frame.try_next().await {
                Ok(Some(val)) => return Ok(val),
                Ok(None) => {}
                Err(e) => {
                    error!("{e}");
                    return Err(GQGMCError::Miscellaneous(e.to_string()));
                }
            }
        }
    }
    async fn power_on(&mut self) -> Result<(), GQGMCError>{
        let mut frame = StringCodec.framed(&mut self.stream);
        if let Err(e) = frame.send(FUNC_POWERON.to_string()).await {
            return Err(GQGMCError::Miscellaneous(e.to_string()))
        }
        Ok(())
    }
    pub async fn get_cpm(&mut self) -> Result<u16, GQGMCError> {
        let mut frame = IntCodec.framed(&mut self.stream);
        let resp: u16;
        frame.send(FUNC_GET_CPM.to_string()).await.expect("Couldn't write message");
        loop {
            match frame.try_next().await {
                Ok(Some(val)) => {  resp = val; break;}
                Ok(None) => {}
                Err(e) => {
                    error!("{e}");
                    return Err(GQGMCError::Miscellaneous(e.to_string()));
                }
            }
        };
        if resp == 0 {
            warn!("Device appears to be turned off, sending power-on signal");
            if let Err(e) = self.power_on().await {
                error!("{e}");
            }

        };
        Ok(resp)
    }

}