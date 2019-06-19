use super::byte;

struct MaskV4 {
    bytes: [byte::Byte; 4]
}

fn get_bytes_from_mask(short_mask: u32) -> [u8; 4] {
    let mut bytes: [u8; 4] = [255, 255, 255, 255];
    let mut bytes_index = 3;
    for iteration in 0..short_mask {
        bytes[bytes_index] <<= 1;
        if (iteration + 1) % 8 == 0 && iteration > 0 {
            bytes_index -= 1;
        }
    }
    bytes
}

fn get_short_mask_from_hosts(hosts: u32) -> u32 {
    let mut sum: u32 = 0;
    let base: u32 = 2;
    let mut index: u32 = 0;

    while sum < hosts {
        sum += base.pow(index);
        index += 1;
    }

    index
}

impl MaskV4 {
    fn from_available_hosts(hosts: u32) -> MaskV4 {
        let mask: u32 = get_short_mask_from_hosts(hosts);
        let bytes_temp: [u8; 4] = get_bytes_from_mask(mask);
        let mut bytes: [byte::Byte; 4] = [
            byte::Byte::from_usize(255),
            byte::Byte::from_usize(255),
            byte::Byte::from_usize(255),
            byte::Byte::from_usize(255)
        ];
        for index in 0..4 {
            bytes[index] = byte::Byte::from_usize(bytes_temp[index]);
        }
        MaskV4 {
            bytes
        }
    }

    fn from_short_mask(short_mask: &str) -> MaskV4 {
        let bits_to_fill = &short_mask[1..];
        let mut bytes_temp: [u8; 4] = [255; 4];
        match bits_to_fill.parse::<u8>() {
            Ok(bits) => {
                let iterations = 32 - bits;
                bytes_temp = get_bytes_from_mask(iterations as u32);
            },
            Err(e) => println!("Error: {}", e),
        }
        let mut bytes: [byte::Byte; 4] = [
            byte::Byte::from_usize(255),
            byte::Byte::from_usize(255),
            byte::Byte::from_usize(255),
            byte::Byte::from_usize(255)
        ];
        for index in 0..4 {
            bytes[index] = byte::Byte::from_usize(bytes_temp[index]);
        }
        MaskV4 {
            bytes
        }
    }

    fn get_bytes (&self) -> [u8; 4] {
        let mut bytes: [u8; 4] = [255; 4];
        for index in 0..4 {
            bytes[index] = self.bytes[index].get_byte();
        }
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_short_mask() {
        let mask = MaskV4::from_short_mask("/24");
        let bytes = mask.get_bytes();
        assert_eq!(bytes[0], 255);
        assert_eq!(bytes[1], 255);
        assert_eq!(bytes[2], 255);
        assert_eq!(bytes[3], 0);
    }

    #[test]
    fn test_from_available_hosts() {
        let mask = MaskV4::from_available_hosts(1000);
        let bytes = mask.get_bytes();
        assert_eq!(bytes[0], 255);
        assert_eq!(bytes[1], 255);
        assert_eq!(bytes[2], 252);
        assert_eq!(bytes[3], 0);
    }
}
