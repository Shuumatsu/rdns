use crate::proto::{BytePacketBuffer, Result};

#[derive(Clone, Debug)]
pub struct Header {
    // the identifier assigned by the program that generates any kind of query.
    // this identifier is copied the corresponding reply and can be used by the requester to match up replies to outstanding queries.
    pub id: u16, // 16 bits

    // specifies whether this message is a query (0), or a response (1).
    pub response: bool, // 1 bit

    // specifies kind of query in this message.
    pub opcode: u8, // 4 bits

    // specifies that the responding name server is an authority for the domain name in question section.
    // only meaningful in responses.
    pub authoritative_answer: bool, // 1 bit

    // specifies that this message was truncated.
    pub truncated_message: bool, // 1 bit

    // this bit directs the name server to pursue the query recursively.
    // this won't work for an Authoritative Server which will only reply to queries relating to the zones hosted,
    // and as such will send an error response to any queries with the RD flag set.
    // for example, with rd set, `dig @ns1.google.com google.com` is ok
    // but `dig @ns1.google.com yahoo.com` will get an error response
    pub recursion_desired: bool, // 1 bit

    // denotes whether recursive query support is available in the name server. Recursive query support is optional.
    // set or cleared in a response
    pub recursion_available: bool, // 1 bit

    // hardwired to 0. reserved for future use
    pub z: bool,                 // 1 bit
    pub checking_disabled: bool, // 1 bit
    pub authed_data: bool,       // 1 bit

    pub rescode: ResCode, // 4 bits

    pub questions: u16,             // 16 bits
    pub answers: u16,               // 16 bits
    pub authoritative_entries: u16, // 16 bits
    pub resource_entries: u16,      // 16 bits
}

impl Header {
    pub fn new() -> Header {
        Header {
            id: 0,

            recursion_desired: false,
            truncated_message: false,
            authoritative_answer: false,
            opcode: 0,
            response: false,

            rescode: ResCode::NoError,
            checking_disabled: false,
            authed_data: false,
            z: false,
            recursion_available: false,

            questions: 0,
            answers: 0,
            authoritative_entries: 0,
            resource_entries: 0,
        }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {
        self.id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;
        let a = (flags >> 8) as u8;
        let b = (flags & 0xFF) as u8;
        self.recursion_desired = (a & (1 << 0)) > 0;
        self.truncated_message = (a & (1 << 1)) > 0;
        self.authoritative_answer = (a & (1 << 2)) > 0;
        self.opcode = (a >> 3) & 0x0F;
        self.response = (a & (1 << 7)) > 0;

        self.rescode = ResCode::from_num(b & 0x0F);
        self.checking_disabled = (b & (1 << 4)) > 0;
        self.authed_data = (b & (1 << 5)) > 0;
        self.z = (b & (1 << 6)) > 0;
        self.recursion_available = (b & (1 << 7)) > 0;

        self.questions = buffer.read_u16()?;
        self.answers = buffer.read_u16()?;
        self.authoritative_entries = buffer.read_u16()?;
        self.resource_entries = buffer.read_u16()?;

        // Return the constant header size
        Ok(())
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.write_u16(self.id)?;

        buffer.write_u8(
            (self.recursion_desired as u8)
                | ((self.truncated_message as u8) << 1)
                | ((self.authoritative_answer as u8) << 2)
                | (self.opcode << 3)
                | ((self.response as u8) << 7) as u8,
        )?;

        buffer.write_u8(
            (self.rescode as u8)
                | ((self.checking_disabled as u8) << 4)
                | ((self.authed_data as u8) << 5)
                | ((self.z as u8) << 6)
                | ((self.recursion_available as u8) << 7),
        )?;

        buffer.write_u16(self.questions)?;
        buffer.write_u16(self.answers)?;
        buffer.write_u16(self.authoritative_entries)?;
        buffer.write_u16(self.resource_entries)?;

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResCode {
    NoError = 0,
    // the name server was unable to interpret the query.
    FormatError = 1,
    // the name server was unable to process this query due to a problem with the name server.
    ServerFailure = 2,
    // signifies that the domain name referenced in the query does not exist.
    // meaningful only for responses from an authoritative name server.
    NameError = 3,
    // the name server does not support the requested kind of query.
    NotImplemented = 4,
    // the name server refuses to perform the specified operation for policy reasons.
    REFUSED = 5,
}

impl ResCode {
    pub fn from_num(num: u8) -> ResCode {
        match num {
            1 => ResCode::FormatError,
            2 => ResCode::ServerFailure,
            3 => ResCode::NameError,
            4 => ResCode::NotImplemented,
            5 => ResCode::REFUSED,
            0 | _ => ResCode::NoError,
        }
    }
}
