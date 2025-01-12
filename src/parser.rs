use bytes::Bytes;

pub fn parse_str(data: &mut Bytes) -> Result<Option<String>, std::string::FromUtf8Error> {
    let mut data = data.splitn(2, |b| *b == b'\0');

    if let Some(str) = data.next() {
        Ok(Some(String::from_utf8(str.to_vec())?))
    } else {
        Ok(None)
    }
}
