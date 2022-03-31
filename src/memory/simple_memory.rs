use anyhow::Result;
use boolector::Btor;
use log::trace;
use std::rc::Rc;

use super::BITS_IN_BYTE;
use crate::{Solver, BV};

type Array = boolector::Array<Rc<Btor>>;

#[derive(Debug, Clone)]
pub struct Memory {
    name: String,

    solver: Solver,

    mem: Array,

    ptr_size: u32,
    //null_detection: bool,
}

impl Memory {
    pub fn new_uninitialized(solver: Solver, ptr_size: u32) -> Self {
        const NAME: &str = "simple_memory";

        let mem = solver.array(ptr_size, BITS_IN_BYTE, Some(NAME));
        Self {
            name: NAME.to_owned(),
            solver,
            mem,
            ptr_size,
        }
    }

    pub fn read_u8(&self, addr: &BV) -> BV {
        self.mem.read(addr).into()
    }

    pub fn write_u8(&mut self, addr: &BV, val: &BV) {
        self.mem = self.mem.write(addr, val);
    }

    pub fn read(&self, addr: &BV, bits: u32) -> Result<BV> {
        trace!("{} read addr={:?}, bits={}", self.name, addr, bits);
        assert_eq!(
            addr.get_width(),
            self.ptr_size,
            "passed wrong sized address"
        );

        // if self.null_detection {
        //     panic!("");
        // }

        let value = if bits < BITS_IN_BYTE {
            self.read_u8(addr).slice(bits - 1, 0).into()
        } else {
            // Ensure we only read full bytes now.
            assert_eq!(bits % BITS_IN_BYTE, 0, "Must read bytes, if bits >= 8");

            let num_bytes = bits / BITS_IN_BYTE;

            // ..
            (0..num_bytes)
                .map(|byte| {
                    let offset = self.solver.bv_from_u64(byte as u64, self.ptr_size);
                    self.read_u8(&addr.add(&offset))
                })
                .reduce(|acc, byte| byte.concat(&acc).into())
                .unwrap()
        };
        trace!("Read value: {:?} at address: {:?}", value, addr);

        Ok(value)
    }

    pub fn write(&mut self, addr: &BV, value: BV) -> Result<()> {
        trace!("{} write addr={:?}, value={:?}", self.name, addr, value);
        assert_eq!(
            addr.get_width(),
            self.ptr_size,
            "passed wrong sized address"
        );

        // if self.null_detection {
        //     panic!("");
        // }

        // Check if we should zero extend the value (if it less than 8-bits).
        let value = if value.get_width() < BITS_IN_BYTE {
            value.uext(BITS_IN_BYTE - value.get_width()).into()
        } else {
            value
        };

        // Ensure the value we write is a multiple of `BITS_IN_BYTE`.
        assert_eq!(value.get_width() & BITS_IN_BYTE, 0);

        let num_bytes = value.get_width() / BITS_IN_BYTE;
        for n in 0..num_bytes {
            let low_bit = n * BITS_IN_BYTE;
            let high_bit = (n + 1) * BITS_IN_BYTE - 1;
            let byte = value.slice(high_bit, low_bit);

            let offset = self.solver.bv_from_u64(n as u64, self.ptr_size);
            let addr = addr.add(&offset);
            self.write_u8(&addr, &byte.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test() {
    //     todo!()
    // }
}
