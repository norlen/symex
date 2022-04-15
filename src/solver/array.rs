use boolector::Btor;
use std::rc::Rc;

use crate::BV;

#[derive(Debug, Clone)]
pub struct Array(pub(super) boolector::Array<Rc<Btor>>);

impl Array {
    pub fn read(&self, addr: &BV) -> BV {
        BV(self.0.read(&addr.0))
    }

    pub fn write(&self, addr: &BV, value: &BV) -> Array {
        Array(self.0.write(&addr.0, &value.0))
    }
}
