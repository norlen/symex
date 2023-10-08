use std::ffi::CStr;

use llvm_sys::{
    core::{self, LLVMGetConstOpcode, LLVMGetValueKind, LLVMIsConstant},
    prelude::LLVMValueRef,
    LLVMOpcode, LLVMValueKind,
};

use crate::{instruction, Type, Value};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Constant {
    Poison(Poison),
    Undef(Undef),
    TargetNone(TargetNone),
    PointerNull(PointerNull),
    AggregateZero(AggregateZero),
    Integer(Integer),
    Float(Float),
    Array(Array),
    Vector(Vector),
    Structure(Structure),

    Expression(Expression),
}

impl Constant {
    pub(crate) fn new(value_ref: LLVMValueRef) -> Self {
        if unsafe { LLVMIsConstant(value_ref) == 0 } {
            panic!("Value is not a constant");
        }

        let value_kind = unsafe { LLVMGetValueKind(value_ref) };
        match value_kind {
            LLVMValueKind::LLVMPoisonValueKind => Poison::new(value_ref).into(),
            LLVMValueKind::LLVMUndefValueValueKind => Undef::new(value_ref).into(),
            LLVMValueKind::LLVMConstantTokenNoneValueKind => TargetNone::new(value_ref).into(),

            LLVMValueKind::LLVMConstantPointerNullValueKind => PointerNull::new(value_ref).into(),
            LLVMValueKind::LLVMConstantAggregateZeroValueKind => {
                AggregateZero::new(value_ref).into()
            }

            LLVMValueKind::LLVMConstantIntValueKind => Integer::new(value_ref).into(),
            LLVMValueKind::LLVMConstantFPValueKind => Float::new(value_ref).into(),

            LLVMValueKind::LLVMConstantDataArrayValueKind => {
                Constant::Array(Array::new(value_ref).into())
            }
            LLVMValueKind::LLVMConstantArrayValueKind => {
                Constant::Array(Array::new(value_ref).into())
            }

            LLVMValueKind::LLVMConstantDataVectorValueKind => {
                Constant::Vector(Vector::new(value_ref).into())
            }
            LLVMValueKind::LLVMConstantVectorValueKind => {
                Constant::Vector(Vector::new(value_ref).into())
            }

            LLVMValueKind::LLVMConstantStructValueKind => Structure::new(value_ref).into(),

            LLVMValueKind::LLVMConstantExprValueKind => Expression::new(value_ref).into(),

            LLVMValueKind::LLVMArgumentValueKind => todo!(),
            LLVMValueKind::LLVMFunctionValueKind => todo!(),
            LLVMValueKind::LLVMGlobalAliasValueKind => todo!(),
            LLVMValueKind::LLVMGlobalIFuncValueKind => todo!(),
            LLVMValueKind::LLVMGlobalVariableValueKind => todo!(),
            LLVMValueKind::LLVMBlockAddressValueKind => todo!(),
            LLVMValueKind::LLVMMetadataAsValueValueKind => todo!(),
            LLVMValueKind::LLVMInlineAsmValueKind => todo!(),
            LLVMValueKind::LLVMInstructionValueKind => todo!(),
            LLVMValueKind::LLVMConstantTargetNoneValueKind => todo!(),

            _ => panic!("Cannot use value kind {:?} as a constant", value_kind),
        }
    }

    pub fn ty(&self) -> Type {
        match self {
            Constant::Poison(c) => Type::new(unsafe { core::LLVMTypeOf(c.0) }),
            Constant::Undef(c) => Type::new(unsafe { core::LLVMTypeOf(c.0) }),
            Constant::TargetNone(c) => Type::new(unsafe { core::LLVMTypeOf(c.0) }),
            Constant::PointerNull(c) => Type::new(unsafe { core::LLVMTypeOf(c.0) }),
            Constant::AggregateZero(c) => Type::new(unsafe { core::LLVMTypeOf(c.0) }),
            Constant::Integer(c) => Type::new(unsafe { core::LLVMTypeOf(c.0) }),
            Constant::Float(c) => Type::new(unsafe { core::LLVMTypeOf(c.0) }),
            Constant::Expression(e) => e.result_type(),
            Constant::Array(c) => c.ty(),
            Constant::Vector(c) => c.ty(),
            Constant::Structure(c) => c.ty(),
        }
    }
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Poison(c) => write!(f, "{c}"),
            Constant::Undef(c) => write!(f, "{c}"),
            Constant::TargetNone(c) => write!(f, "{c}"),
            Constant::PointerNull(c) => write!(f, "{c}"),
            Constant::AggregateZero(c) => write!(f, "{c}"),
            Constant::Integer(c) => write!(f, "{c}"),
            Constant::Float(c) => write!(f, "{c}"),
            Constant::Array(c) => write!(f, "{c}"),
            Constant::Vector(c) => write!(f, "{c}"),
            Constant::Expression(c) => write!(f, "{c}"),
            Constant::Structure(c) => write!(f, "{c}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression {
    Trunc(instruction::Trunc),
    ZExt(instruction::ZExt),
    SExt(instruction::SExt),
    FPTrunc(instruction::FPTrunc),
    FPExt(instruction::FPExt),
    FPToUI(instruction::FPToUI),
    FPToSI(instruction::FPToSI),
    UIToFP(instruction::UIToFP),
    SIToFP(instruction::SIToFP),
    PtrToInt(instruction::PtrToInt),
    IntToPtr(instruction::IntToPtr),
    BitCast(instruction::BitCast),
    AddrSpaceCast(instruction::AddrSpaceCast),
    GetElementPtr(instruction::GetElementPtr),
    ICmp(instruction::ICmp),
    FCmp(instruction::FCmp),
    ExtractElement(instruction::ExtractElement),
    InsertElement(instruction::InsertElement),
    ShuffleVector(instruction::ShuffleVector),
    Add(instruction::Add),
    FAdd(instruction::FAdd),
    Sub(instruction::Sub),
    FSub(instruction::FSub),
    Mul(instruction::Mul),
    FMul(instruction::FMul),
    Shl(instruction::Shl),
    LShr(instruction::LShr),
    AShr(instruction::AShr),
    Xor(instruction::Xor),
}

impl Expression {
    pub(crate) fn new(value_ref: LLVMValueRef) -> Self {
        let op_code = unsafe { LLVMGetConstOpcode(value_ref) };
        match op_code {
            LLVMOpcode::LLVMTrunc => Expression::Trunc(instruction::Trunc::new(value_ref)),
            LLVMOpcode::LLVMZExt => Expression::ZExt(instruction::ZExt::new(value_ref)),
            LLVMOpcode::LLVMSExt => Expression::SExt(instruction::SExt::new(value_ref)),
            LLVMOpcode::LLVMFPTrunc => Expression::FPTrunc(instruction::FPTrunc::new(value_ref)),
            LLVMOpcode::LLVMFPExt => Expression::FPExt(instruction::FPExt::new(value_ref)),
            LLVMOpcode::LLVMFPToUI => Expression::FPToUI(instruction::FPToUI::new(value_ref)),
            LLVMOpcode::LLVMFPToSI => Expression::FPToSI(instruction::FPToSI::new(value_ref)),
            LLVMOpcode::LLVMUIToFP => Expression::UIToFP(instruction::UIToFP::new(value_ref)),
            LLVMOpcode::LLVMSIToFP => Expression::SIToFP(instruction::SIToFP::new(value_ref)),
            LLVMOpcode::LLVMPtrToInt => Expression::PtrToInt(instruction::PtrToInt::new(value_ref)),
            LLVMOpcode::LLVMIntToPtr => Expression::IntToPtr(instruction::IntToPtr::new(value_ref)),
            LLVMOpcode::LLVMBitCast => Expression::BitCast(instruction::BitCast::new(value_ref)),
            LLVMOpcode::LLVMAddrSpaceCast => {
                Expression::AddrSpaceCast(instruction::AddrSpaceCast::new(value_ref))
            }
            LLVMOpcode::LLVMGetElementPtr => {
                Expression::GetElementPtr(instruction::GetElementPtr::new(value_ref))
            }
            LLVMOpcode::LLVMICmp => Expression::ICmp(instruction::ICmp::new(value_ref)),
            LLVMOpcode::LLVMFCmp => Expression::FCmp(instruction::FCmp::new(value_ref)),
            LLVMOpcode::LLVMExtractElement => {
                Expression::ExtractElement(instruction::ExtractElement::new(value_ref))
            }
            LLVMOpcode::LLVMInsertElement => {
                Expression::InsertElement(instruction::InsertElement::new(value_ref))
            }
            LLVMOpcode::LLVMShuffleVector => {
                Expression::ShuffleVector(instruction::ShuffleVector::new(value_ref))
            }
            LLVMOpcode::LLVMAdd => Expression::Add(instruction::Add::new(value_ref)),
            LLVMOpcode::LLVMFAdd => Expression::FAdd(instruction::FAdd::new(value_ref)),
            LLVMOpcode::LLVMSub => Expression::Sub(instruction::Sub::new(value_ref)),
            LLVMOpcode::LLVMFSub => Expression::FSub(instruction::FSub::new(value_ref)),
            LLVMOpcode::LLVMMul => Expression::Mul(instruction::Mul::new(value_ref)),
            LLVMOpcode::LLVMFMul => Expression::FMul(instruction::FMul::new(value_ref)),
            LLVMOpcode::LLVMShl => Expression::Shl(instruction::Shl::new(value_ref)),
            LLVMOpcode::LLVMLShr => Expression::LShr(instruction::LShr::new(value_ref)),
            LLVMOpcode::LLVMAShr => Expression::AShr(instruction::AShr::new(value_ref)),
            LLVMOpcode::LLVMXor => Expression::Xor(instruction::Xor::new(value_ref)),

            _ => panic!("Cannot use opcode {:?} as a constant expression", op_code),
        }
    }

    pub fn result_type(&self) -> Type {
        match self {
            Expression::Trunc(i) => i.result_type(),
            Expression::ZExt(i) => i.result_type(),
            Expression::SExt(i) => i.result_type(),
            Expression::FPTrunc(i) => i.result_type(),
            Expression::FPExt(i) => i.result_type(),
            Expression::FPToUI(i) => i.result_type(),
            Expression::FPToSI(i) => i.result_type(),
            Expression::UIToFP(i) => i.result_type(),
            Expression::SIToFP(i) => i.result_type(),
            Expression::PtrToInt(i) => i.result_type(),
            Expression::IntToPtr(i) => i.result_type(),
            Expression::BitCast(i) => i.result_type(),
            Expression::AddrSpaceCast(i) => i.result_type(),
            Expression::GetElementPtr(i) => i.result_type(),
            Expression::ICmp(i) => i.result_type(),
            Expression::FCmp(i) => i.result_type(),
            Expression::ExtractElement(i) => i.result_type(),
            Expression::InsertElement(i) => i.result_type(),
            Expression::ShuffleVector(i) => i.result_type(),
            Expression::Add(i) => i.result_type(),
            Expression::FAdd(i) => i.result_type(),
            Expression::Sub(i) => i.result_type(),
            Expression::FSub(i) => i.result_type(),
            Expression::Mul(i) => i.result_type(),
            Expression::FMul(i) => i.result_type(),
            Expression::Shl(i) => i.result_type(),
            Expression::LShr(i) => i.result_type(),
            Expression::AShr(i) => i.result_type(),
            Expression::Xor(i) => i.result_type(),
        }
    }
}

impl From<Expression> for Constant {
    fn from(value: Expression) -> Self {
        Self::Expression(value)
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Trunc(i) => write!(f, "{i}"),
            Expression::ZExt(i) => write!(f, "{i}"),
            Expression::SExt(i) => write!(f, "{i}"),
            Expression::FPTrunc(i) => write!(f, "{i}"),
            Expression::FPExt(i) => write!(f, "{i}"),
            Expression::FPToUI(i) => write!(f, "{i}"),
            Expression::FPToSI(i) => write!(f, "{i}"),
            Expression::UIToFP(i) => write!(f, "{i}"),
            Expression::SIToFP(i) => write!(f, "{i}"),
            Expression::PtrToInt(i) => write!(f, "{i}"),
            Expression::IntToPtr(i) => write!(f, "{i}"),
            Expression::BitCast(i) => write!(f, "{i}"),
            Expression::AddrSpaceCast(i) => write!(f, "{i}"),
            Expression::GetElementPtr(i) => write!(f, "{i}"),
            Expression::ICmp(i) => write!(f, "{i}"),
            Expression::FCmp(i) => write!(f, "{i}"),
            Expression::ExtractElement(i) => write!(f, "{i}"),
            Expression::InsertElement(i) => write!(f, "{i}"),
            Expression::ShuffleVector(i) => write!(f, "{i}"),
            Expression::Add(i) => write!(f, "{i}"),
            Expression::FAdd(i) => write!(f, "{i}"),
            Expression::Sub(i) => write!(f, "{i}"),
            Expression::FSub(i) => write!(f, "{i}"),
            Expression::Mul(i) => write!(f, "{i}"),
            Expression::FMul(i) => write!(f, "{i}"),
            Expression::Shl(i) => write!(f, "{i}"),
            Expression::LShr(i) => write!(f, "{i}"),
            Expression::AShr(i) => write!(f, "{i}"),
            Expression::Xor(i) => write!(f, "{i}"),
        }
    }
}

macro_rules! impl_constant {
    ($name:ident) => {
        impl $name {
            pub(crate) fn new(value_ref: llvm_sys::prelude::LLVMValueRef) -> Self {
                Self(value_ref)
            }

            pub fn ty(&self) -> crate::Type {
                crate::Type::new(unsafe { llvm_sys::core::LLVMTypeOf(self.0) })
            }
        }

        impl From<$name> for Constant {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = {
                    let ptr = unsafe { llvm_sys::core::LLVMPrintValueToString(self.0) };
                    unsafe { std::ffi::CStr::from_ptr(ptr) }
                };
                write!(f, "{}", s.to_string_lossy())
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Poison(LLVMValueRef);
impl_constant!(Poison);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Undef(LLVMValueRef);
impl_constant!(Undef);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TargetNone(LLVMValueRef);
impl_constant!(TargetNone);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointerNull(LLVMValueRef);
impl_constant!(PointerNull);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AggregateZero(LLVMValueRef);
impl_constant!(AggregateZero);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Integer(LLVMValueRef);
impl_constant!(Integer);

impl Integer {
    pub fn value(&self) -> u64 {
        unsafe { core::LLVMConstIntGetZExtValue(self.0) }
    }

    pub fn is_zero(&self) -> bool {
        unsafe { core::LLVMConstIntGetZExtValue(self.0) == 0 }
    }

    pub fn is_one(&self) -> bool {
        unsafe { core::LLVMConstIntGetZExtValue(self.0) == 1 }
    }

    pub fn is_all_ones(&self) -> bool {
        unsafe { core::LLVMConstIntGetSExtValue(self.0) == -1 }
    }

    pub fn is_max_signed_value(&self) -> bool {
        unsafe { core::LLVMConstIntGetSExtValue(self.0) == i64::MAX }
    }

    pub fn is_min_signed_value(&self) -> bool {
        unsafe { core::LLVMConstIntGetSExtValue(self.0) == i64::MIN }
    }

    pub fn is_max_unsigned_value(&self) -> bool {
        unsafe { core::LLVMConstIntGetZExtValue(self.0) == u64::MAX }
    }

    pub fn is_min_unsigned_value(&self) -> bool {
        unsafe { core::LLVMConstIntGetZExtValue(self.0) == u64::MIN }
    }

    pub fn is_power_of_two(&self) -> bool {
        unsafe { core::LLVMConstIntGetZExtValue(self.0).is_power_of_two() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Float(LLVMValueRef);
impl_constant!(Float);

impl Float {
    pub fn value(&self) -> f64 {
        let mut loses_info = 0;
        unsafe { core::LLVMConstRealGetDouble(self.0, &mut loses_info) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Array(LLVMValueRef);
impl_constant!(Array);

impl Array {
    pub fn num_elements(&self) -> u64 {
        let type_ref = unsafe { core::LLVMTypeOf(self.0) };
        unsafe { core::LLVMGetArrayLength2(type_ref) }
    }

    pub fn get(&self, index: usize) -> Option<Value> {
        if index >= self.num_elements() as usize {
            return None;
        }

        let value = unsafe { core::LLVMGetAggregateElement(self.0, index as u32) };
        match value.is_null() {
            true => None,
            false => Some(Value::new(value)),
        }
    }

    pub fn elements(&self) -> ArrayIter {
        ArrayIter::new(self.0, self.num_elements() as u32)
    }
}

pub struct ArrayIter {
    array: LLVMValueRef,
    index: u32,
    len: u32,
}

impl ArrayIter {
    pub(crate) fn new(array: LLVMValueRef, len: u32) -> Self {
        Self {
            array,
            index: 0,
            len,
        }
    }
}

impl Iterator for ArrayIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        let value = unsafe { core::LLVMGetAggregateElement(self.array, self.index) };
        self.index += 1;
        match value.is_null() {
            true => None,
            false => Some(Value::new(value)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector(LLVMValueRef);
impl_constant!(Vector);

impl Vector {
    pub fn num_elements(&self) -> u32 {
        let type_ref = unsafe { core::LLVMTypeOf(self.0) };
        unsafe { core::LLVMGetVectorSize(type_ref) }
    }

    pub fn get(&self, index: usize) -> Option<Value> {
        if index >= self.num_elements() as usize {
            return None;
        }

        let value = unsafe { core::LLVMGetAggregateElement(self.0, index as u32) };
        match value.is_null() {
            true => None,
            false => Some(Value::new(value)),
        }
    }

    pub fn elements(&self) -> VectorIter {
        VectorIter::new(self.0, self.num_elements())
    }
}

pub struct VectorIter {
    array: LLVMValueRef,
    index: u32,
    len: u32,
}

impl VectorIter {
    pub(crate) fn new(array: LLVMValueRef, len: u32) -> Self {
        Self {
            array,
            index: 0,
            len,
        }
    }
}

impl Iterator for VectorIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        let value = unsafe { core::LLVMGetAggregateElement(self.array, self.index) };
        self.index += 1;
        match value.is_null() {
            true => None,
            false => Some(Value::new(value)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Structure(LLVMValueRef);
impl_constant!(Structure);

impl Structure {
    pub fn name(&self) -> Option<&CStr> {
        let name = unsafe {
            let mut len = 0;
            let ptr = core::LLVMGetValueName2(self.0, &mut len);
            std::ffi::CStr::from_ptr(ptr)
        };
        match name.is_empty() {
            true => None,
            false => Some(name),
        }
    }

    pub fn num_fields(&self) -> u32 {
        let type_ref = unsafe { core::LLVMTypeOf(self.0) };
        unsafe { core::LLVMCountStructElementTypes(type_ref) }
    }

    pub fn fields(&self) -> StructureIter {
        StructureIter::new(self.0, self.num_fields())
    }
}

pub struct StructureIter {
    array: LLVMValueRef,
    index: u32,
    len: u32,
}

impl StructureIter {
    pub(crate) fn new(array: LLVMValueRef, len: u32) -> Self {
        Self {
            array,
            index: 0,
            len,
        }
    }
}

impl Iterator for StructureIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            None
        } else {
            let value = unsafe { core::LLVMGetOperand(self.array, self.index) };
            self.index += 1;
            match value.is_null() {
                true => None,
                false => Some(Value::new(value)),
            }
        }
    }
}
