use elrond_wasm::{
    api::ManagedTypeApi,
    types::{ManagedBuffer, ManagedVec},
};

use crate::sc_panic_self;
pub trait UtilsU64 {
    fn to_ascii<M: ManagedTypeApi>(&self) -> ManagedBuffer<M>;
    fn to_hex<M: ManagedTypeApi>(&self) -> ManagedBuffer<M>;
}

impl UtilsU64 for u64 {
    fn to_hex<M: ManagedTypeApi>(&self) -> ManagedBuffer<M> {
        let mut reversed_digits = ManagedVec::<M, u8>::new();
        let mut result = self.clone();

        while result > 0 {
            let digit = result % 16;
            result /= 16;

            let digit_char = match digit {
                0 => b'0',
                1 => b'1',
                2 => b'2',
                3 => b'3',
                4 => b'4',
                5 => b'5',
                6 => b'6',
                7 => b'7',
                8 => b'8',
                9 => b'9',
                10 => b'a',
                11 => b'b',
                12 => b'c',
                13 => b'd',
                14 => b'e',
                15 => b'f',
                _ => sc_panic_self!(M, "invalid digit"),
            };

            reversed_digits.push(digit_char);
        }

        if &reversed_digits.len() == &0 {
            return ManagedBuffer::<M>::new_from_bytes(b"00");
        }

        let mut o = ManagedBuffer::<M>::new();

        if &reversed_digits.len() % 2 != 0 {
            o.append_bytes(b"0");
        }

        for digit in reversed_digits.iter().rev() {
            o.append_bytes(&[digit]);
        }

        return o;
    }
    fn to_ascii<M: ManagedTypeApi>(&self) -> ManagedBuffer<M> {
        let mut reversed_digits = ManagedVec::<M, u8>::new();
        let mut result = self.clone();

        while result > 0 {
            let digit = result % 10;
            result /= 10;

            let digit_char = match digit {
                0 => b'0',
                1 => b'1',
                2 => b'2',
                3 => b'3',
                4 => b'4',
                5 => b'5',
                6 => b'6',
                7 => b'7',
                8 => b'8',
                9 => b'9',
                _ => sc_panic_self!(M, "invalid digit"),
            };

            reversed_digits.push(digit_char);
        }

        if &reversed_digits.len() == &0 {
            return ManagedBuffer::<M>::new_from_bytes(b"0");
        }

        let mut o = ManagedBuffer::<M>::new();

        for digit in reversed_digits.iter().rev() {
            o.append_bytes(&[digit]);
        }

        return o;
    }
}
