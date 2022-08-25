use serde_json::Value;

enum Endianness {
    Little,
    Big,
    Unknow,
}
#[derive(Default)]
struct Header {
    tag: String,
    version: u32, // Must be one, allows to detect endianness
}

impl Header {
    fn new(tag: String, version: u32) -> Self {
        Header { tag, version }
    }

    fn from_data(data: &[u8]) -> Self {
        if data.len() != 8 as usize {
            return Header::default();
        }

        let tag = data[0..4].iter().map(|d| *d as char).collect::<String>();
        let version = data[4..8]
            .iter()
            .enumerate()
            .map(|(i, d)| (*d as u32) << (i * 8))
            .sum::<u32>();

        let mut header = Header::new(tag, version);

        match header.endianess() {
            Endianness::Big => {
                let mut reversed_data = data.to_vec();
                reversed_data.reverse();
                header = Header::from_data(&reversed_data[0..8])
            }
            Endianness::Unknow => {
                header = Header::default();
            }
            Endianness::Little => {}
        }

        header
    }

    fn endianess(&self) -> Endianness {
        match self.version {
            1 => Endianness::Little,
            16777216 => Endianness::Big,
            _ => Endianness::Unknow,
        }
    }
}

pub fn decode(qbjs: &Vec<u8>) -> Value {
    if qbjs.len() > 8 {
        let header = Header::from_data(&qbjs[0..8]);
        println!("qbjs tag: {}", header.tag);
        println!("qbjs version: {:#b}", header.version);
    }

    serde_json::Value::Null
}
