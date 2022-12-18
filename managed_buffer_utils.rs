use core::cmp::Ordering;

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{ManagedBuffer, ManagedVec},
};

use crate::sc_panic_self;

pub trait ManagedBufferUtils<M: ManagedTypeApi> {
    fn load_512_bytes(&self) -> [u8; 512];

    fn split(&self, char: u8) -> ManagedVec<M, ManagedBuffer<M>>;
    fn split_last_occurence(&self, char: u8) -> (ManagedBuffer<M>, ManagedBuffer<M>);

    fn remove_first_char(&self) -> ManagedBuffer<M>;
    fn remove_first_and_last_char(&self) -> ManagedBuffer<M>;

    fn hex_to_u64(&self) -> Option<u64>;
    fn ascii_to_u64(&self) -> Option<u64>;
    /// Set the first character to uppercase
    fn capitalize(&self) -> ManagedBuffer<M>;
    fn equals_ignore_case(&self, other: &ManagedBuffer<M>) -> bool;
    fn append_trailing_character_if_missing(&self, character: u8) -> ManagedBuffer<M>;
    /// The replace method use new_buffer as ManagedBuffer because is it the easier way to implement
    fn replace(&self, old_buffer: &[u8], new_buffer: &ManagedBuffer<M>) -> ManagedBuffer<M>;
    fn contains(&self, to_find: &[u8]) -> bool;
    fn to_lowercase(&self) -> ManagedBuffer<M>;

    /// Returns 0 if equals. Return 1 if self is after other in the alphabetically order. Returns 0 if self is before other in the alphabetically order.
    fn compare(&self, other: &Self) -> Ordering;
}

impl<M: ManagedTypeApi> ManagedBufferUtils<M> for ManagedBuffer<M> {
    fn load_512_bytes(&self) -> [u8; 512] {
        if (self.len() as usize) > 512 {
            sc_panic_self!(M, "ManagedBuffer is too big");
        }

        let mut bytes: [u8; 512] = [0; 512];

        self.load_to_byte_array(&mut bytes);

        return bytes;
    }

    fn split(&self, char: u8) -> ManagedVec<M, ManagedBuffer<M>> {
        if self.len() == 0 {
            return ManagedVec::new();
        }

        let bytes = self.load_512_bytes();

        let mut output = ManagedVec::<M, ManagedBuffer<M>>::new();

        let mut start_index = 0;

        for (i, &byte) in bytes.iter().enumerate() {
            if byte == char || i >= self.len() {
                let slice = ManagedBuffer::new_from_bytes(&bytes[start_index..i]);
                output.push(slice);
                start_index = i + 1;

                if i >= self.len() {
                    break;
                }
            }
        }

        return output;
    }

    fn split_last_occurence(&self, char: u8) -> (ManagedBuffer<M>, ManagedBuffer<M>) {
        let bytes = self.load_512_bytes();

        for i in (0..self.len() - 1).rev() {
            if bytes[i] == char {
                let first = ManagedBuffer::<M>::new_from_bytes(&bytes[..i]);
                let second = ManagedBuffer::<M>::new_from_bytes(&bytes[i + 1..self.len()]);

                return (first, second);
            }
        }

        sc_panic_self!(M, "no occurence of char {:x} in bytes {:x}", char, self);
    }

    fn remove_first_char(&self) -> ManagedBuffer<M> {
        let bytes = self.load_512_bytes();

        ManagedBuffer::new_from_bytes(&bytes[1..self.len()])
    }

    fn remove_first_and_last_char(&self) -> ManagedBuffer<M> {
        let bytes = self.load_512_bytes();

        return ManagedBuffer::new_from_bytes(&bytes[1..self.len() - 1]);
    }

    fn hex_to_u64(&self) -> Option<u64> {
        let bytes = self.load_512_bytes();

        let mut result: u64 = 0;

        for i in bytes {
            if i == 0 {
                break;
            }

            result *= 16;
            result += (i as char).to_digit(16)? as u64;
        }

        Some(result)
    }

    fn ascii_to_u64(&self) -> Option<u64> {
        let bytes = self.load_512_bytes();

        let mut result: u64 = 0;

        for i in bytes {
            if i == 0 {
                break;
            }

            result *= 10;
            result += (i as char).to_digit(10)? as u64;
        }

        Some(result)
    }

    fn capitalize(&self) -> ManagedBuffer<M> {
        let bytes = self.load_512_bytes();

        let mut o = ManagedBuffer::<M>::new();

        // uppercase first letter
        o.append_bytes(&[bytes[0].to_ascii_uppercase()]);
        o.append_bytes(&bytes[1..self.len()]);

        return o;
    }

    fn equals_ignore_case(self: &ManagedBuffer<M>, other: &ManagedBuffer<M>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let self_bytes = self.load_512_bytes();

        let mut other_bytes: [u8; 512] = [0; 512];
        other.load_to_byte_array(&mut other_bytes);

        for i in 0..self.len() {
            if self_bytes[i].to_ascii_lowercase() != other_bytes[i].to_ascii_lowercase() {
                return false;
            }
        }

        return true;
    }

    fn append_trailing_character_if_missing(&self, character: u8) -> ManagedBuffer<M> {
        let bytes = self.load_512_bytes();

        let mut o = ManagedBuffer::<M>::new();

        o.append_bytes(&bytes[0..self.len()]);

        if bytes[self.len() - 1] != character {
            o.append_bytes(&[character]);
        }

        return o;
    }

    fn replace(&self, old_buffer: &[u8], new_buffer: &ManagedBuffer<M>) -> ManagedBuffer<M> {
        let bytes = self.load_512_bytes();

        let mut o = ManagedBuffer::<M>::new();

        let mut elements_to_skip = 0;

        for i in 0..self.len() {
            if elements_to_skip > 0 {
                elements_to_skip -= 1;
                continue;
            }

            if bytes[i] == old_buffer[0] {
                for j in 0..old_buffer.len() {
                    // is not a match, let's continue to the next character
                    if bytes[i + j] != old_buffer[j] {
                        o.append_bytes(&[bytes[i]]);
                        break;
                    }

                    // is it a match
                    if j == old_buffer.len() - 1 {
                        o.append(new_buffer);
                        elements_to_skip = j; // skip the old buffer
                        break;
                    }
                }
            } else {
                o.append_bytes(&[bytes[i]]);
            }
        }

        return o;
    }

    fn contains(&self, to_find: &[u8]) -> bool {
        let bytes = self.load_512_bytes();

        // naive implementation of includes() algorithm
        // An upgrade could be the KMP algorithm
        for i in 0..self.len() {
            if bytes[i] == to_find[0] {
                for j in 0..to_find.len() {
                    if bytes[i + j] != to_find[j] {
                        break;
                    }

                    if j == to_find.len() - 1 {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    fn to_lowercase(&self) -> ManagedBuffer<M> {
        let bytes = self.load_512_bytes();

        let mut o = ManagedBuffer::<M>::new();

        for i in 0..self.len() {
            o.append_bytes(&[bytes[i].to_ascii_lowercase()]);
        }

        return o;
    }

    fn compare(&self, other: &Self) -> Ordering {
        let a_bytes = self.load_512_bytes();
        let b_bytes = other.load_512_bytes();

        return a_bytes.cmp(&b_bytes);
    }
}
