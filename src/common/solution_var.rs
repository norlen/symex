use llvm_ir::TypeRef;

use crate::solver::BV;

#[derive(Clone, Debug)]
pub struct SolutionVariable {
    pub name: String,

    pub value: BV,

    pub ty: Option<TypeRef>,
}
