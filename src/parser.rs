use bytes::{Buf, Bytes};
use thiserror::Error;
use tracing::{debug, info};

use crate::{
    source::Source,
    systeminfo::{Topology, Version},
    Connection,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("String parsing failed")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

pub fn parse_payload(connection: &mut Connection, payload: &mut Bytes) -> Result<(), Error> {
    while payload.has_remaining() {
        let size = payload.get_u16();
        payload.get_u16(); // skip two bytes, unknow function.
        let cmd = payload.split_to(4);
        let data_size = size as usize - 8;
        let mut data = payload.split_to(data_size);
        debug!("Command {:?} Size: {}", cmd, size);

        match &cmd[..] {
            b"_ver" => {
                let version = Version::parse(&mut data);
                info!("Firmware version: {}", version);
                connection.system_info.set_version(version);
            }
            b"_pin" => {
                let product = parse_str(&mut data)?.unwrap();
                info!("Product: {}", product);
                connection.system_info.set_product(&product);
            }
            b"_top" => {
                let topology = Topology::parse(&mut data);
                info!("Topology: {}", topology);
                connection.system_info.set_topology(topology);
            }
            b"InPr" => {
                let source = Source::parse(&mut data)?;
                info!("{}", source);
            }
            _ => {
                debug!(
                    "Unknown command: {} Data: {:02X?} [{}]",
                    String::from_utf8(cmd.to_vec())?,
                    &data[..],
                    data_size
                );
            }
        }
    }

    Ok(())
}

pub fn parse_str(data: &mut Bytes) -> Result<Option<String>, Error> {
    let mut data = data.splitn(2, |b| *b == b'\0');

    if let Some(str) = data.next() {
        Ok(Some(String::from_utf8(str.to_vec())?))
    } else {
        Ok(None)
    }
}
