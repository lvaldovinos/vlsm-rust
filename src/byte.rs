struct DecimalToBinaryConverter {
    byte: u8,
    bits: [bool; 8],
    sum: i32,
    diff: i32
}

fn get_msb (byte: i32) -> u32 {
    let mut index = 0;
    let base: i32 = 2;

    while base.pow(index) <= byte {
        index += 1;
    }
    
    index -= 1;
    index
}

impl DecimalToBinaryConverter {
    fn from_usize(byte: u8) -> DecimalToBinaryConverter {
        DecimalToBinaryConverter {
            byte,
            bits: [false; 8],
            sum: 0,
            diff: 0
        }
    }

    fn get_diff(&self) -> i32 {
        self.diff
    }

    fn get_bits(&self) -> [bool; 8] {
        self.bits
    }

    fn process_bit(&mut self, msb_index: u32) {
        let base: i32 = 2;
        self.bits[msb_index as usize] = true;
        self.sum += base.pow(msb_index);
        self.diff = self.byte as i32 - self.sum;
    }

    fn to_binary(&mut self) {
        if self.byte == 0 {
            return
        }
        let msb_index = get_msb(self.byte as i32);
        self.process_bit(msb_index);

        while self.get_diff() > 0 {
            let msb_index = get_msb(self.get_diff());

            self.process_bit(msb_index);
        }
    }
}

pub struct Byte {
    byte: u8,
    bits: [bool; 8]
}

impl Byte {
    pub fn from_usize(byte: u8) -> Byte {
        let mut converter = DecimalToBinaryConverter::from_usize(byte);
        converter.to_binary();

        Byte {
            byte,
            bits: converter.get_bits()
        }
    }

    pub fn get_bits (&self) -> [bool; 8] {
        self.bits
    }

    pub fn get_byte (&self) -> u8 {
        self.byte
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_usize() {
        let byte = Byte::from_usize(255);
        let bits = byte.get_bits();
        for index in 0..8 {
            assert_eq!(bits[index], true);
        }
        let byte = Byte::from_usize(0);
        let bits = byte.get_bits();
        for index in 0..8 {
            assert_eq!(bits[index], false);
        }
        let byte = Byte::from_usize(150);
        let bits = byte.get_bits();
        for index in 0..8 {
            if index == 1 || index == 2 || index == 4 || index == 7 {
                assert_eq!(bits[index], true);
            } else {
                assert_eq!(bits[index], false);
            }
        }
    }
}
