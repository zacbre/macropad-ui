use crate::error::HidError;

const PACKET_INIT: u8 = 0x69;

#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum PacketHeader {
    ChangeVolume = 0x04,
    GetVolume = 0x05,
    ForceVolume = 0x06,
    Stats = 0x08,
    Unknown = 0xFF,
}

impl PacketHeader {
    pub fn from_u8(byte: u8) -> PacketHeader {
        match byte {
            0x04 => PacketHeader::ChangeVolume,
            0x05 => PacketHeader::GetVolume,
            0x06 => PacketHeader::ForceVolume,
            0x08 => PacketHeader::Stats,
            _ => PacketHeader::Unknown
        }
    }

    pub fn into_u8(&self) -> u8 {
        match self {
            PacketHeader::ChangeVolume => 0x04,
            PacketHeader::GetVolume => 0x05,
            PacketHeader::ForceVolume => 0x06,
            PacketHeader::Stats => 0x08,
            PacketHeader::Unknown => 0xFF,
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    pub header: PacketHeader,
    data: Vec<u8>,
}

impl Packet {
    pub fn new(header: PacketHeader, data: Vec<u8>) -> Self {
        let mut sized_vec = vec![0 as u8; 32];
        let mut i = 0;
        for item in data {
            sized_vec[i] = item;
            i += 1;
        }

        Self {
            header,
            data: sized_vec,
        }
    }

    pub fn from(received: usize, data: &[u8]) -> Result<Self, HidError> {
        let mut vec = data[..received].to_vec();
        println!("Packet: {:?}", vec);
        let packet_header_byte = vec[0];
        vec.remove(0);
        println!("Packet header byte: {:?}", packet_header_byte);

        let packet_header = PacketHeader::from_u8(packet_header_byte);
        if packet_header == PacketHeader::Unknown {
            return Err(HidError::new("Packet header is unknown!".to_string()));
        }

        Ok(Self {
            header: packet_header,
            data: vec,
        })
    }

    pub fn raw(&self) -> Vec<u8> {
        let vec_clone = self.data.clone();
        vec_clone
    }

    pub fn to_packet_bytes(&self) -> Vec<u8> {
        let mut vec_clone = self.data.clone();
        vec_clone.insert(0, PACKET_INIT);
        vec_clone.insert(1, PACKET_INIT);
        vec_clone.insert(2, self.header.into_u8());

        vec_clone
    }
}