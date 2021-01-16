use crate::proto::query::Query;
use crate::proto::{BytePacketBuffer, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    // A domain name represented as a sequence of labels, where each label consists of a length octet followed by that number of octets.
    pub name: String,
    pub qtype: Query,
}

impl Question {
    pub fn new(name: String, qtype: Query) -> Question {
        Question { name, qtype }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.read_qname(&mut self.name)?;
        self.qtype = Query::from_num(buffer.read_u16()?); // qtype
        let _ = buffer.read_u16()?; // class

        Ok(())
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.write_qname(&self.name)?;

        let typenum = self.qtype.to_num();
        buffer.write_u16(typenum)?;
        buffer.write_u16(1)?;

        Ok(())
    }
}
