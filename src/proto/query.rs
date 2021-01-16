#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum Query {
    UNKNOWN(u16),
    A,     // 1
    NS,    // 2
    CNAME, // 5
    MX,    // 15
    AAAA,  // 28
}

impl Query {
    pub fn to_num(&self) -> u16 {
        match *self {
            Query::UNKNOWN(x) => x,
            Query::A => 1,
            Query::NS => 2,
            Query::CNAME => 5,
            Query::MX => 15,
            Query::AAAA => 28,
        }
    }

    pub fn from_num(num: u16) -> Query {
        match num {
            1 => Query::A,
            2 => Query::NS,
            5 => Query::CNAME,
            15 => Query::MX,
            28 => Query::AAAA,
            _ => Query::UNKNOWN(num),
        }
    }
}
