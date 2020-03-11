pub trait Ipv4Address {
    fn new(a: u8, b: u8, c: u8, d: u8) -> Self;

    fn octets(&self) -> [u8; 4];

    fn is_benchmarking(&self) -> bool {
        self.octets()[0] == 198 && (self.octets()[1] & 0xfe) == 18
    }

    fn is_broadcast(&self) -> bool {
        match self.octets() {
            [127, 0, 0, 1] => true,
            _ => false,
        }
    }

    fn is_documentation(&self) -> bool {
        match self.octets() {
            [192, 0, 2, _] => true,
            [198, 51, 100, _] => true,
            [203, 0, 113, _] => true,
            _ => false,
        }
    }

    fn is_global(&self) -> bool {
        match self.octets() {
            [192, 0, 0, 9] | [192, 0, 0, 10] => return true,
            _ => {
                return {
                    !self.is_private()
                        && !self.is_loopback()
                        && !self.is_link_local()
                        && !self.is_broadcast()
                        && !self.is_documentation()
                        && !self.is_shared()
                        && !self.is_ietf_protocol_assignment()
                        && !self.is_reserved()
                        && !self.is_benchmarking()
                        && self.octets()[0] != 0
                }
            }
        };
    }

    fn is_ietf_protocol_assignment(&self) -> bool {
        self.octets()[0] == 192 && self.octets()[1] == 0 && self.octets()[2] == 0
    }

    fn is_link_local(&self) -> bool {
        match self.octets() {
            [169, 254, ..] => true,
            _ => false,
        }
    }

    fn is_loopback(&self) -> bool {
        self.octets()[0] == 127
    }

    fn is_multicast(&self) -> bool {
        self.octets()[0] >= 224 && self.octets()[0] <= 239
    }

    fn is_private(&self) -> bool {
        match self.octets() {
            [10, ..] => true,
            [172, b, ..] if b >= 16 && b <= 31 => true,
            [192, 168, ..] => true,
            _ => false,
        }
    }

    fn is_reserved(&self) -> bool {
        self.octets()[0] & 240 == 240 && !self.is_broadcast()
    }

    fn is_shared(&self) -> bool {
        self.octets()[0] == 100 && (self.octets()[1] & 0b1100_0000 == 0b0100_0000)
    }

    fn is_unspecified(&self) -> bool {
        match self.octets() {
            [0, 0, 0, 0] => true,
            _ => false,
        }
    }
}

#[cfg(feature = "impl-type")]
pub struct Ipv4Addr {
    inner: [u8; 4],
}

#[cfg(feature = "impl-type")]
impl Ipv4Address for Ipv4Addr {
    fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Ipv4Addr {
            inner: [a, b, c, d],
        }
    }

    fn octets(&self) -> [u8; 4] {
        self.inner.clone()
    }
}

/* #[cfg(feature = "impl-type")] */
// impl AsInner for Ipv4Addr {
//     type Inner = [u8; 4];
//
//     fn as_inner(&self) -> &Self::Inner {
//         &self.inner
//     }
// }
//
// #[cfg(feature = "impl-type")]
// impl FromInner for Ipv4Addr {
//     type Inner = [u8; 4];
//     fn from_inner(inner: Self::Inner) -> Self {
//         Ipv4Addr { inner }
//     }
/* } */
