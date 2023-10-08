use llvm_sys::{
    core::{
        LLVMCountIncoming, LLVMGetAlignment, LLVMGetAllocatedType, LLVMGetAtomicRMWBinOp,
        LLVMGetCalledFunctionType, LLVMGetCalledValue, LLVMGetCmpXchgFailureOrdering,
        LLVMGetCmpXchgSuccessOrdering, LLVMGetFCmpPredicate, LLVMGetFirstInstruction,
        LLVMGetFunctionCallConv, LLVMGetGEPSourceElementType, LLVMGetICmpPredicate,
        LLVMGetIncomingBlock, LLVMGetIncomingValue, LLVMGetIndices, LLVMGetInstructionOpcode,
        LLVMGetLastInstruction, LLVMGetMaskValue, LLVMGetNSW, LLVMGetNUW, LLVMGetNextInstruction,
        LLVMGetNormalDest, LLVMGetNumArgOperands, LLVMGetNumIndices, LLVMGetNumMaskElements,
        LLVMGetNumOperands, LLVMGetNumSuccessors, LLVMGetOperand, LLVMGetOrdering,
        LLVMGetSuccessor, LLVMGetSwitchDefaultDest, LLVMGetUndefMaskElem, LLVMGetUnwindDest,
        LLVMGetVolatile, LLVMGetWeak, LLVMIsAtomicSingleThread, LLVMIsInBounds, LLVMTypeOf,
        LLVMValueAsBasicBlock, LLVMValueIsBasicBlock,
    },
    prelude::*,
    LLVMOpcode,
};

pub use llvm_sys::LLVMAtomicOrdering;
pub use llvm_sys::LLVMAtomicRMWBinOp;
pub use llvm_sys::LLVMIntPredicate;
pub use llvm_sys::LLVMRealPredicate;

use crate::{util::DebugLocation, Type, Value};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlock(LLVMBasicBlockRef);

impl BasicBlock {
    pub fn instructions(&self) -> InstructionIter {
        unsafe { InstructionIter::new(self.0) }
    }

    pub fn first_instruction(&self) -> Option<Instruction> {
        let value_ref = unsafe { LLVMGetFirstInstruction(self.0) };
        match value_ref.is_null() {
            true => None,
            false => Some(value_ref.into()),
        }
    }

    pub fn next_instruction(&self, instr: Instruction) -> Option<Instruction> {
        let value_ref = unsafe { LLVMGetNextInstruction(instr.get_ptr()) };
        match value_ref.is_null() {
            true => None,
            false => Some(value_ref.into()),
        }
    }

    pub fn terminator(&self) -> Option<Instruction> {
        let value_ref = unsafe { LLVMGetLastInstruction(self.0) };
        match value_ref.is_null() {
            true => None,
            false => Some(value_ref.into()),
        }
    }

    pub(crate) fn new(src: LLVMBasicBlockRef) -> Self {
        Self(src)
    }
}

pub struct InstructionIter(LLVMValueRef);

impl InstructionIter {
    pub(crate) unsafe fn new(bb: LLVMBasicBlockRef) -> Self {
        Self(unsafe { LLVMGetFirstInstruction(bb) })
    }
}

impl Iterator for InstructionIter {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_null() {
            None
        } else {
            let current = self.0;
            self.0 = unsafe { LLVMGetNextInstruction(self.0) };
            Some(current.into())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Instruction {
    Load(Load),
    Store(Store),
    Alloca(Alloca),
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    UDiv(UDiv),
    SDiv(SDiv),
    URem(URem),
    SRem(SRem),
    And(And),
    Or(Or),
    Xor(Xor),
    Shl(Shl),
    LShr(LShr),
    AShr(AShr),
    FAdd(FAdd),
    FSub(FSub),
    FMul(FMul),
    FDiv(FDiv),
    FRem(FRem),
    FNeg(FNeg),
    ExtractElement(ExtractElement),
    InsertElement(InsertElement),
    ShuffleVector(ShuffleVector),
    ExtractValue(ExtractValue),
    InsertValue(InsertValue),
    Fence(Fence),
    CmpXchg(CmpXchg),
    AtomicRMW(AtomicRMW),
    GetElementPtr(GetElementPtr),
    Trunc(Trunc),
    ZExt(ZExt),
    SExt(SExt),
    FPTrunc(FPTrunc),
    FPExt(FPExt),
    FPToUI(FPToUI),
    FPToSI(FPToSI),
    UIToFP(UIToFP),
    SIToFP(SIToFP),
    PtrToInt(PtrToInt),
    IntToPtr(IntToPtr),
    BitCast(BitCast),
    AddrSpaceCast(AddrSpaceCast),
    ICmp(ICmp),
    FCmp(FCmp),
    Phi(Phi),
    Select(Select),
    Freeze(Freeze),
    Call(Call),
    VAArg(VAArg),
    LandingPad(LandingPad),
    CatchPad(CatchPad),
    CleanupPad(CleanupPad),

    // Terminators.
    Ret(Ret),
    Br(Br),
    CondBr(CondBr),
    Switch(Switch),
    IndirectBr(IndirectBr),
    Invoke(Invoke),
    Resume(Resume),
    Unreachable(Unreachable),
    CleanupRet(CleanupRet),
    CatchRet(CatchRet),
    CatchSwitch(CatchSwitch),
    CallBr(CallBr),
}

impl Instruction {
    pub(crate) fn new(src: LLVMValueRef) -> Self {
        src.into()
    }

    pub fn result_name(&self) -> Option<&core::ffi::CStr> {
        match self {
            Instruction::Load(i) => i.result_name(),
            Instruction::Store(i) => i.result_name(),
            Instruction::Alloca(i) => i.result_name(),
            Instruction::Add(i) => i.result_name(),
            Instruction::Sub(i) => i.result_name(),
            Instruction::Mul(i) => i.result_name(),
            Instruction::UDiv(i) => i.result_name(),
            Instruction::SDiv(i) => i.result_name(),
            Instruction::URem(i) => i.result_name(),
            Instruction::SRem(i) => i.result_name(),
            Instruction::And(i) => i.result_name(),
            Instruction::Or(i) => i.result_name(),
            Instruction::Xor(i) => i.result_name(),
            Instruction::Shl(i) => i.result_name(),
            Instruction::LShr(i) => i.result_name(),
            Instruction::AShr(i) => i.result_name(),
            Instruction::FAdd(i) => i.result_name(),
            Instruction::FSub(i) => i.result_name(),
            Instruction::FMul(i) => i.result_name(),
            Instruction::FDiv(i) => i.result_name(),
            Instruction::FRem(i) => i.result_name(),
            Instruction::FNeg(i) => i.result_name(),
            Instruction::ExtractElement(i) => i.result_name(),
            Instruction::InsertElement(i) => i.result_name(),
            Instruction::ShuffleVector(i) => i.result_name(),
            Instruction::ExtractValue(i) => i.result_name(),
            Instruction::InsertValue(i) => i.result_name(),
            Instruction::Fence(i) => i.result_name(),
            Instruction::CmpXchg(i) => i.result_name(),
            Instruction::AtomicRMW(i) => i.result_name(),
            Instruction::GetElementPtr(i) => i.result_name(),
            Instruction::Trunc(i) => i.result_name(),
            Instruction::ZExt(i) => i.result_name(),
            Instruction::SExt(i) => i.result_name(),
            Instruction::FPTrunc(i) => i.result_name(),
            Instruction::FPExt(i) => i.result_name(),
            Instruction::FPToUI(i) => i.result_name(),
            Instruction::FPToSI(i) => i.result_name(),
            Instruction::UIToFP(i) => i.result_name(),
            Instruction::SIToFP(i) => i.result_name(),
            Instruction::PtrToInt(i) => i.result_name(),
            Instruction::IntToPtr(i) => i.result_name(),
            Instruction::BitCast(i) => i.result_name(),
            Instruction::AddrSpaceCast(i) => i.result_name(),
            Instruction::ICmp(i) => i.result_name(),
            Instruction::FCmp(i) => i.result_name(),
            Instruction::Phi(i) => i.result_name(),
            Instruction::Select(i) => i.result_name(),
            Instruction::Freeze(i) => i.result_name(),
            Instruction::Call(i) => i.result_name(),
            Instruction::VAArg(i) => i.result_name(),
            Instruction::LandingPad(i) => i.result_name(),
            Instruction::CatchPad(i) => i.result_name(),
            Instruction::CleanupPad(i) => i.result_name(),
            Instruction::Ret(i) => i.result_name(),
            Instruction::Br(i) => i.result_name(),
            Instruction::CondBr(i) => i.result_name(),
            Instruction::Switch(i) => i.result_name(),
            Instruction::IndirectBr(i) => i.result_name(),
            Instruction::Invoke(i) => i.result_name(),
            Instruction::Resume(i) => i.result_name(),
            Instruction::Unreachable(i) => i.result_name(),
            Instruction::CleanupRet(i) => i.result_name(),
            Instruction::CatchRet(i) => i.result_name(),
            Instruction::CatchSwitch(i) => i.result_name(),
            Instruction::CallBr(i) => i.result_name(),
        }
    }

    pub fn result_type(&self) -> Type {
        match self {
            Instruction::Load(i) => i.result_type(),
            Instruction::Store(i) => i.result_type(),
            Instruction::Alloca(i) => i.result_type(),
            Instruction::Add(i) => i.result_type(),
            Instruction::Sub(i) => i.result_type(),
            Instruction::Mul(i) => i.result_type(),
            Instruction::UDiv(i) => i.result_type(),
            Instruction::SDiv(i) => i.result_type(),
            Instruction::URem(i) => i.result_type(),
            Instruction::SRem(i) => i.result_type(),
            Instruction::And(i) => i.result_type(),
            Instruction::Or(i) => i.result_type(),
            Instruction::Xor(i) => i.result_type(),
            Instruction::Shl(i) => i.result_type(),
            Instruction::LShr(i) => i.result_type(),
            Instruction::AShr(i) => i.result_type(),
            Instruction::FAdd(i) => i.result_type(),
            Instruction::FSub(i) => i.result_type(),
            Instruction::FMul(i) => i.result_type(),
            Instruction::FDiv(i) => i.result_type(),
            Instruction::FRem(i) => i.result_type(),
            Instruction::FNeg(i) => i.result_type(),
            Instruction::ExtractElement(i) => i.result_type(),
            Instruction::InsertElement(i) => i.result_type(),
            Instruction::ShuffleVector(i) => i.result_type(),
            Instruction::ExtractValue(i) => i.result_type(),
            Instruction::InsertValue(i) => i.result_type(),
            Instruction::Fence(i) => i.result_type(),
            Instruction::CmpXchg(i) => i.result_type(),
            Instruction::AtomicRMW(i) => i.result_type(),
            Instruction::GetElementPtr(i) => i.result_type(),
            Instruction::Trunc(i) => i.result_type(),
            Instruction::ZExt(i) => i.result_type(),
            Instruction::SExt(i) => i.result_type(),
            Instruction::FPTrunc(i) => i.result_type(),
            Instruction::FPExt(i) => i.result_type(),
            Instruction::FPToUI(i) => i.result_type(),
            Instruction::FPToSI(i) => i.result_type(),
            Instruction::UIToFP(i) => i.result_type(),
            Instruction::SIToFP(i) => i.result_type(),
            Instruction::PtrToInt(i) => i.result_type(),
            Instruction::IntToPtr(i) => i.result_type(),
            Instruction::BitCast(i) => i.result_type(),
            Instruction::AddrSpaceCast(i) => i.result_type(),
            Instruction::ICmp(i) => i.result_type(),
            Instruction::FCmp(i) => i.result_type(),
            Instruction::Phi(i) => i.result_type(),
            Instruction::Select(i) => i.result_type(),
            Instruction::Freeze(i) => i.result_type(),
            Instruction::Call(i) => i.result_type(),
            Instruction::VAArg(i) => i.result_type(),
            Instruction::LandingPad(i) => i.result_type(),
            Instruction::CatchPad(i) => i.result_type(),
            Instruction::CleanupPad(i) => i.result_type(),
            Instruction::Ret(i) => i.result_type(),
            Instruction::Br(i) => i.result_type(),
            Instruction::CondBr(i) => i.result_type(),
            Instruction::Switch(i) => i.result_type(),
            Instruction::IndirectBr(i) => i.result_type(),
            Instruction::Invoke(i) => i.result_type(),
            Instruction::Resume(i) => i.result_type(),
            Instruction::Unreachable(i) => i.result_type(),
            Instruction::CleanupRet(i) => i.result_type(),
            Instruction::CatchRet(i) => i.result_type(),
            Instruction::CatchSwitch(i) => i.result_type(),
            Instruction::CallBr(i) => i.result_type(),
        }
    }

    pub fn debug_location(&self) -> Option<DebugLocation> {
        match self {
            Instruction::Load(i) => i.debug_location(),
            Instruction::Store(i) => i.debug_location(),
            Instruction::Alloca(i) => i.debug_location(),
            Instruction::Add(i) => i.debug_location(),
            Instruction::Sub(i) => i.debug_location(),
            Instruction::Mul(i) => i.debug_location(),
            Instruction::UDiv(i) => i.debug_location(),
            Instruction::SDiv(i) => i.debug_location(),
            Instruction::URem(i) => i.debug_location(),
            Instruction::SRem(i) => i.debug_location(),
            Instruction::And(i) => i.debug_location(),
            Instruction::Or(i) => i.debug_location(),
            Instruction::Xor(i) => i.debug_location(),
            Instruction::Shl(i) => i.debug_location(),
            Instruction::LShr(i) => i.debug_location(),
            Instruction::AShr(i) => i.debug_location(),
            Instruction::FAdd(i) => i.debug_location(),
            Instruction::FSub(i) => i.debug_location(),
            Instruction::FMul(i) => i.debug_location(),
            Instruction::FDiv(i) => i.debug_location(),
            Instruction::FRem(i) => i.debug_location(),
            Instruction::FNeg(i) => i.debug_location(),
            Instruction::ExtractElement(i) => i.debug_location(),
            Instruction::InsertElement(i) => i.debug_location(),
            Instruction::ShuffleVector(i) => i.debug_location(),
            Instruction::ExtractValue(i) => i.debug_location(),
            Instruction::InsertValue(i) => i.debug_location(),
            Instruction::Fence(i) => i.debug_location(),
            Instruction::CmpXchg(i) => i.debug_location(),
            Instruction::AtomicRMW(i) => i.debug_location(),
            Instruction::GetElementPtr(i) => i.debug_location(),
            Instruction::Trunc(i) => i.debug_location(),
            Instruction::ZExt(i) => i.debug_location(),
            Instruction::SExt(i) => i.debug_location(),
            Instruction::FPTrunc(i) => i.debug_location(),
            Instruction::FPExt(i) => i.debug_location(),
            Instruction::FPToUI(i) => i.debug_location(),
            Instruction::FPToSI(i) => i.debug_location(),
            Instruction::UIToFP(i) => i.debug_location(),
            Instruction::SIToFP(i) => i.debug_location(),
            Instruction::PtrToInt(i) => i.debug_location(),
            Instruction::IntToPtr(i) => i.debug_location(),
            Instruction::BitCast(i) => i.debug_location(),
            Instruction::AddrSpaceCast(i) => i.debug_location(),
            Instruction::ICmp(i) => i.debug_location(),
            Instruction::FCmp(i) => i.debug_location(),
            Instruction::Phi(i) => i.debug_location(),
            Instruction::Select(i) => i.debug_location(),
            Instruction::Freeze(i) => i.debug_location(),
            Instruction::Call(i) => i.debug_location(),
            Instruction::VAArg(i) => i.debug_location(),
            Instruction::LandingPad(i) => i.debug_location(),
            Instruction::CatchPad(i) => i.debug_location(),
            Instruction::CleanupPad(i) => i.debug_location(),
            Instruction::Ret(i) => i.debug_location(),
            Instruction::Br(i) => i.debug_location(),
            Instruction::CondBr(i) => i.debug_location(),
            Instruction::Switch(i) => i.debug_location(),
            Instruction::IndirectBr(i) => i.debug_location(),
            Instruction::Invoke(i) => i.debug_location(),
            Instruction::Resume(i) => i.debug_location(),
            Instruction::Unreachable(i) => i.debug_location(),
            Instruction::CleanupRet(i) => i.debug_location(),
            Instruction::CatchRet(i) => i.debug_location(),
            Instruction::CatchSwitch(i) => i.debug_location(),
            Instruction::CallBr(i) => i.debug_location(),
        }
    }

    pub(crate) fn get_ptr(&self) -> LLVMValueRef {
        match self {
            Instruction::Load(i) => i.0,
            Instruction::Store(i) => i.0,
            Instruction::Alloca(i) => i.0,
            Instruction::Add(i) => i.0,
            Instruction::Sub(i) => i.0,
            Instruction::Mul(i) => i.0,
            Instruction::UDiv(i) => i.0,
            Instruction::SDiv(i) => i.0,
            Instruction::URem(i) => i.0,
            Instruction::SRem(i) => i.0,
            Instruction::And(i) => i.0,
            Instruction::Or(i) => i.0,
            Instruction::Xor(i) => i.0,
            Instruction::Shl(i) => i.0,
            Instruction::LShr(i) => i.0,
            Instruction::AShr(i) => i.0,
            Instruction::FAdd(i) => i.0,
            Instruction::FSub(i) => i.0,
            Instruction::FMul(i) => i.0,
            Instruction::FDiv(i) => i.0,
            Instruction::FRem(i) => i.0,
            Instruction::FNeg(i) => i.0,
            Instruction::ExtractElement(i) => i.0,
            Instruction::InsertElement(i) => i.0,
            Instruction::ShuffleVector(i) => i.0,
            Instruction::ExtractValue(i) => i.0,
            Instruction::InsertValue(i) => i.0,
            Instruction::Fence(i) => i.0,
            Instruction::CmpXchg(i) => i.0,
            Instruction::AtomicRMW(i) => i.0,
            Instruction::GetElementPtr(i) => i.0,
            Instruction::Trunc(i) => i.0,
            Instruction::ZExt(i) => i.0,
            Instruction::SExt(i) => i.0,
            Instruction::FPTrunc(i) => i.0,
            Instruction::FPExt(i) => i.0,
            Instruction::FPToUI(i) => i.0,
            Instruction::FPToSI(i) => i.0,
            Instruction::UIToFP(i) => i.0,
            Instruction::SIToFP(i) => i.0,
            Instruction::PtrToInt(i) => i.0,
            Instruction::IntToPtr(i) => i.0,
            Instruction::BitCast(i) => i.0,
            Instruction::AddrSpaceCast(i) => i.0,
            Instruction::ICmp(i) => i.0,
            Instruction::FCmp(i) => i.0,
            Instruction::Phi(i) => i.0,
            Instruction::Select(i) => i.0,
            Instruction::Freeze(i) => i.0,
            Instruction::Call(i) => i.0,
            Instruction::VAArg(i) => i.0,
            Instruction::LandingPad(i) => i.0,
            Instruction::CatchPad(i) => i.0,
            Instruction::CleanupPad(i) => i.0,
            Instruction::Ret(i) => i.0,
            Instruction::Br(i) => i.0,
            Instruction::CondBr(i) => i.0,
            Instruction::Switch(i) => i.0,
            Instruction::IndirectBr(i) => i.0,
            Instruction::Invoke(i) => i.0,
            Instruction::Resume(i) => i.0,
            Instruction::Unreachable(i) => i.0,
            Instruction::CleanupRet(i) => i.0,
            Instruction::CatchRet(i) => i.0,
            Instruction::CatchSwitch(i) => i.0,
            Instruction::CallBr(i) => i.0,
        }
    }
}

impl From<LLVMValueRef> for Instruction {
    fn from(value: LLVMValueRef) -> Self {
        let op_code = unsafe { LLVMGetInstructionOpcode(value) };
        match op_code {
            LLVMOpcode::LLVMRet => Ret::new(value).into(),
            LLVMOpcode::LLVMBr => match unsafe { LLVMGetNumOperands(value) } {
                1 => Br::new(value).into(),
                _ => CondBr::new(value).into(),
            },
            LLVMOpcode::LLVMSwitch => Switch::new(value).into(),
            LLVMOpcode::LLVMIndirectBr => IndirectBr::new(value).into(),
            LLVMOpcode::LLVMInvoke => Invoke::new(value).into(),
            LLVMOpcode::LLVMUnreachable => Unreachable::new(value).into(),
            LLVMOpcode::LLVMCallBr => CallBr::new(value).into(),
            LLVMOpcode::LLVMFNeg => FNeg::new(value).into(),
            LLVMOpcode::LLVMAdd => Add::new(value).into(),
            LLVMOpcode::LLVMFAdd => FAdd::new(value).into(),
            LLVMOpcode::LLVMSub => Sub::new(value).into(),
            LLVMOpcode::LLVMFSub => FSub::new(value).into(),
            LLVMOpcode::LLVMMul => Mul::new(value).into(),
            LLVMOpcode::LLVMFMul => FMul::new(value).into(),
            LLVMOpcode::LLVMUDiv => UDiv::new(value).into(),
            LLVMOpcode::LLVMSDiv => SDiv::new(value).into(),
            LLVMOpcode::LLVMFDiv => FDiv::new(value).into(),
            LLVMOpcode::LLVMURem => URem::new(value).into(),
            LLVMOpcode::LLVMSRem => SRem::new(value).into(),
            LLVMOpcode::LLVMFRem => FRem::new(value).into(),
            LLVMOpcode::LLVMShl => Shl::new(value).into(),
            LLVMOpcode::LLVMLShr => LShr::new(value).into(),
            LLVMOpcode::LLVMAShr => AShr::new(value).into(),
            LLVMOpcode::LLVMAnd => And::new(value).into(),
            LLVMOpcode::LLVMOr => Or::new(value).into(),
            LLVMOpcode::LLVMXor => Xor::new(value).into(),
            LLVMOpcode::LLVMAlloca => Alloca::new(value).into(),
            LLVMOpcode::LLVMLoad => Load::new(value).into(),
            LLVMOpcode::LLVMStore => Store::new(value).into(),
            LLVMOpcode::LLVMGetElementPtr => GetElementPtr::new(value).into(),
            LLVMOpcode::LLVMTrunc => Trunc::new(value).into(),
            LLVMOpcode::LLVMZExt => ZExt::new(value).into(),
            LLVMOpcode::LLVMSExt => SExt::new(value).into(),
            LLVMOpcode::LLVMFPToUI => FPToUI::new(value).into(),
            LLVMOpcode::LLVMFPToSI => FPToSI::new(value).into(),
            LLVMOpcode::LLVMUIToFP => UIToFP::new(value).into(),
            LLVMOpcode::LLVMSIToFP => SIToFP::new(value).into(),
            LLVMOpcode::LLVMFPTrunc => FPTrunc::new(value).into(),
            LLVMOpcode::LLVMFPExt => FPExt::new(value).into(),
            LLVMOpcode::LLVMPtrToInt => PtrToInt::new(value).into(),
            LLVMOpcode::LLVMIntToPtr => IntToPtr::new(value).into(),
            LLVMOpcode::LLVMBitCast => BitCast::new(value).into(),
            LLVMOpcode::LLVMAddrSpaceCast => AddrSpaceCast::new(value).into(),
            LLVMOpcode::LLVMICmp => ICmp::new(value).into(),
            LLVMOpcode::LLVMFCmp => FCmp::new(value).into(),
            LLVMOpcode::LLVMPHI => Phi::new(value).into(),
            LLVMOpcode::LLVMCall => Call::new(value).into(),
            LLVMOpcode::LLVMSelect => Select::new(value).into(),
            LLVMOpcode::LLVMUserOp1 => todo!(),
            LLVMOpcode::LLVMUserOp2 => todo!(),
            LLVMOpcode::LLVMVAArg => VAArg::new(value).into(),
            LLVMOpcode::LLVMExtractElement => ExtractElement::new(value).into(),
            LLVMOpcode::LLVMInsertElement => InsertElement::new(value).into(),
            LLVMOpcode::LLVMShuffleVector => ShuffleVector::new(value).into(),
            LLVMOpcode::LLVMExtractValue => ExtractValue::new(value).into(),
            LLVMOpcode::LLVMInsertValue => InsertValue::new(value).into(),
            LLVMOpcode::LLVMFreeze => Freeze::new(value).into(),
            LLVMOpcode::LLVMFence => Fence::new(value).into(),
            LLVMOpcode::LLVMAtomicCmpXchg => todo!(),
            LLVMOpcode::LLVMAtomicRMW => AtomicRMW::new(value).into(),
            LLVMOpcode::LLVMResume => Resume::new(value).into(),
            LLVMOpcode::LLVMLandingPad => LandingPad::new(value).into(),
            LLVMOpcode::LLVMCleanupRet => CleanupRet::new(value).into(),
            LLVMOpcode::LLVMCatchRet => CatchRet::new(value).into(),
            LLVMOpcode::LLVMCatchPad => CatchPad::new(value).into(),
            LLVMOpcode::LLVMCleanupPad => CleanupPad::new(value).into(),
            LLVMOpcode::LLVMCatchSwitch => CatchSwitch::new(value).into(),
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Load(i) => write!(f, "{i}"),
            Instruction::Store(i) => write!(f, "{i}"),
            Instruction::Alloca(i) => write!(f, "{i}"),
            Instruction::Add(i) => write!(f, "{i}"),
            Instruction::Sub(i) => write!(f, "{i}"),
            Instruction::Mul(i) => write!(f, "{i}"),
            Instruction::UDiv(i) => write!(f, "{i}"),
            Instruction::SDiv(i) => write!(f, "{i}"),
            Instruction::URem(i) => write!(f, "{i}"),
            Instruction::SRem(i) => write!(f, "{i}"),
            Instruction::And(i) => write!(f, "{i}"),
            Instruction::Or(i) => write!(f, "{i}"),
            Instruction::Xor(i) => write!(f, "{i}"),
            Instruction::Shl(i) => write!(f, "{i}"),
            Instruction::LShr(i) => write!(f, "{i}"),
            Instruction::AShr(i) => write!(f, "{i}"),
            Instruction::FAdd(i) => write!(f, "{i}"),
            Instruction::FSub(i) => write!(f, "{i}"),
            Instruction::FMul(i) => write!(f, "{i}"),
            Instruction::FDiv(i) => write!(f, "{i}"),
            Instruction::FRem(i) => write!(f, "{i}"),
            Instruction::FNeg(i) => write!(f, "{i}"),
            Instruction::ExtractElement(i) => write!(f, "{i}"),
            Instruction::InsertElement(i) => write!(f, "{i}"),
            Instruction::ShuffleVector(i) => write!(f, "{i}"),
            Instruction::ExtractValue(i) => write!(f, "{i}"),
            Instruction::InsertValue(i) => write!(f, "{i}"),
            Instruction::Fence(i) => write!(f, "{i}"),
            Instruction::CmpXchg(i) => write!(f, "{i}"),
            Instruction::AtomicRMW(i) => write!(f, "{i}"),
            Instruction::GetElementPtr(i) => write!(f, "{i}"),
            Instruction::Trunc(i) => write!(f, "{i}"),
            Instruction::ZExt(i) => write!(f, "{i}"),
            Instruction::SExt(i) => write!(f, "{i}"),
            Instruction::FPTrunc(i) => write!(f, "{i}"),
            Instruction::FPExt(i) => write!(f, "{i}"),
            Instruction::FPToUI(i) => write!(f, "{i}"),
            Instruction::FPToSI(i) => write!(f, "{i}"),
            Instruction::UIToFP(i) => write!(f, "{i}"),
            Instruction::SIToFP(i) => write!(f, "{i}"),
            Instruction::PtrToInt(i) => write!(f, "{i}"),
            Instruction::IntToPtr(i) => write!(f, "{i}"),
            Instruction::BitCast(i) => write!(f, "{i}"),
            Instruction::AddrSpaceCast(i) => write!(f, "{i}"),
            Instruction::ICmp(i) => write!(f, "{i}"),
            Instruction::FCmp(i) => write!(f, "{i}"),
            Instruction::Phi(i) => write!(f, "{i}"),
            Instruction::Select(i) => write!(f, "{i}"),
            Instruction::Freeze(i) => write!(f, "{i}"),
            Instruction::Call(i) => write!(f, "{i}"),
            Instruction::VAArg(i) => write!(f, "{i}"),
            Instruction::LandingPad(i) => write!(f, "{i}"),
            Instruction::CatchPad(i) => write!(f, "{i}"),
            Instruction::CleanupPad(i) => write!(f, "{i}"),
            Instruction::Ret(i) => write!(f, "{i}"),
            Instruction::Br(i) => write!(f, "{i}"),
            Instruction::CondBr(i) => write!(f, "{i}"),
            Instruction::Switch(i) => write!(f, "{i}"),
            Instruction::IndirectBr(i) => write!(f, "{i}"),
            Instruction::Invoke(i) => write!(f, "{i}"),
            Instruction::Resume(i) => write!(f, "{i}"),
            Instruction::Unreachable(i) => write!(f, "{i}"),
            Instruction::CleanupRet(i) => write!(f, "{i}"),
            Instruction::CatchRet(i) => write!(f, "{i}"),
            Instruction::CatchSwitch(i) => write!(f, "{i}"),
            Instruction::CallBr(i) => write!(f, "{i}"),
        }
    }
}

macro_rules! impl_instruction {
    ($name:ident) => {
        impl $name {
            #[allow(unused)]
            pub(crate) fn new(src: LLVMValueRef) -> Self {
                Self(src)
            }

            pub fn result_name(&self) -> Option<&core::ffi::CStr> {
                let result_name = unsafe {
                    let mut len = 0;
                    let ptr = llvm_sys::core::LLVMGetValueName2(self.0, &mut len);
                    core::ffi::CStr::from_ptr(ptr)
                };

                match result_name.is_empty() {
                    false => Some(result_name),
                    true => None,
                }
            }

            pub fn result_type(&self) -> Type {
                Type::new(unsafe { LLVMTypeOf(self.0) })
            }

            pub fn debug_location(&self) -> Option<DebugLocation> {
                crate::util::debug_location_with_column(self.0)
            }
        }

        impl From<$name> for Instruction {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = {
                    let ptr = unsafe { llvm_sys::core::LLVMPrintValueToString(self.0) };
                    unsafe { core::ffi::CStr::from_ptr(ptr) }
                };
                write!(f, "{}", s.to_string_lossy())
            }
        }
    };
}

macro_rules! impl_binop {
    ($name:ident) => {
        impl $name {
            pub fn lhs(&self) -> Value {
                let value_ref = unsafe { LLVMGetOperand(self.0, 0) };
                Value::new(value_ref)
            }

            pub fn rhs(&self) -> Value {
                let value_ref = unsafe { LLVMGetOperand(self.0, 1) };
                Value::new(value_ref)
            }
        }
    };
}

macro_rules! impl_overflow {
    ($name:ident) => {
        impl $name {
            pub fn no_signed_wrap(&self) -> bool {
                unsafe { LLVMGetNSW(self.0) != 0 }
            }

            pub fn no_unsigned_wrap(&self) -> bool {
                unsafe { LLVMGetNUW(self.0) != 0 }
            }
        }
    };
}

macro_rules! impl_exact {
    ($name:ident) => {
        impl $name {
            pub fn exact(&self) -> bool {
                todo!()
            }
        }
    };
}

macro_rules! impl_unop {
    ($name:ident) => {
        impl $name {
            pub fn value(&self) -> Value {
                let value_ref = unsafe { LLVMGetOperand(self.0, 0) };
                Value::new(value_ref)
            }

            pub fn to_type(&self) -> Type {
                let type_ref = unsafe { LLVMTypeOf(self.0) };
                Type::new(type_ref)
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Load(LLVMValueRef);
impl_instruction!(Load);

impl Load {
    pub fn address(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn is_volatile(&self) -> bool {
        unsafe { LLVMGetVolatile(self.0) != 0 }
    }

    pub fn alignment(&self) -> u32 {
        unsafe { LLVMGetAlignment(self.0) }
    }

    pub fn atomic_ordering(&self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetOrdering(self.0) }
    }

    pub fn is_atomic_single_thread(&self) -> bool {
        unsafe { LLVMIsAtomicSingleThread(self.0) != 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Store(LLVMValueRef);
impl_instruction!(Store);

impl Store {
    pub fn value(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn address(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 1) })
    }

    pub fn is_volatile(&self) -> bool {
        unsafe { LLVMGetVolatile(self.0) != 0 }
    }

    pub fn alignment(&self) -> u32 {
        unsafe { LLVMGetAlignment(self.0) }
    }

    pub fn atomic_ordering(&self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetOrdering(self.0) }
    }

    pub fn is_atomic_single_thread(&self) -> bool {
        unsafe { LLVMIsAtomicSingleThread(self.0) != 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Alloca(LLVMValueRef);
impl_instruction!(Alloca);

impl Alloca {
    pub fn allocated_type(&self) -> Type {
        Type::new(unsafe { LLVMGetAllocatedType(self.0) })
    }

    pub fn num_elements(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn alignment(&self) -> u32 {
        unsafe { LLVMGetAlignment(self.0) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Add(LLVMValueRef);
impl_instruction!(Add);
impl_binop!(Add);
impl_overflow!(Add);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sub(LLVMValueRef);
impl_instruction!(Sub);
impl_binop!(Sub);
impl_overflow!(Sub);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mul(LLVMValueRef);
impl_instruction!(Mul);
impl_binop!(Mul);
impl_overflow!(Mul);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UDiv(LLVMValueRef);
impl_instruction!(UDiv);
impl_binop!(UDiv);
impl_exact!(UDiv);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SDiv(LLVMValueRef);
impl_instruction!(SDiv);
impl_binop!(SDiv);
impl_exact!(SDiv);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct URem(LLVMValueRef);
impl_instruction!(URem);
impl_binop!(URem);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SRem(LLVMValueRef);
impl_instruction!(SRem);
impl_binop!(SRem);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct And(LLVMValueRef);
impl_instruction!(And);
impl_binop!(And);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Or(LLVMValueRef);
impl_instruction!(Or);
impl_binop!(Or);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Xor(LLVMValueRef);
impl_instruction!(Xor);
impl_binop!(Xor);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Shl(LLVMValueRef);
impl_instruction!(Shl);
impl_binop!(Shl);
impl_overflow!(Shl);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LShr(LLVMValueRef);
impl_instruction!(LShr);
impl_binop!(LShr);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AShr(LLVMValueRef);
impl_instruction!(AShr);
impl_binop!(AShr);
impl_exact!(AShr);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FAdd(LLVMValueRef);
impl_instruction!(FAdd);
impl_binop!(FAdd);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FSub(LLVMValueRef);
impl_instruction!(FSub);
impl_binop!(FSub);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FMul(LLVMValueRef);
impl_instruction!(FMul);
impl_binop!(FMul);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FDiv(LLVMValueRef);
impl_instruction!(FDiv);
impl_binop!(FDiv);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FRem(LLVMValueRef);
impl_instruction!(FRem);
impl_binop!(FRem);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FNeg(LLVMValueRef);
impl_instruction!(FNeg);

impl FNeg {
    pub fn value(&self) -> LLVMValueRef {
        unsafe { LLVMGetOperand(self.0, 0) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExtractElement(LLVMValueRef);
impl_instruction!(ExtractElement);

impl ExtractElement {
    pub fn aggregate(&self) -> LLVMValueRef {
        unsafe { LLVMGetOperand(self.0, 0) }
    }

    pub fn index(&self) -> LLVMValueRef {
        unsafe { LLVMGetOperand(self.0, 1) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InsertElement(LLVMValueRef);
impl_instruction!(InsertElement);

impl InsertElement {
    pub fn aggregate(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn element(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 1) })
    }

    pub fn index(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 2) })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShuffleVector(LLVMValueRef);
impl_instruction!(ShuffleVector);

impl ShuffleVector {
    pub fn lhs(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn rhs(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 1) })
    }

    pub fn mask(&self) -> Option<Vec<Option<u32>>> {
        let type_ref = unsafe { LLVMTypeOf(self.0) };
        let Type::Vector(ty) = Type::new(type_ref) else {
            return None;
        };
        let is_scalable = ty.is_scalable();

        let num_elements = unsafe { LLVMGetNumMaskElements(self.0) };
        if num_elements == 0 {
            // TODO: checkme
            return None;
        }

        let undefined_element_value = unsafe { LLVMGetUndefMaskElem() };
        let mask = (0..num_elements)
            .map(|i| unsafe { LLVMGetMaskValue(self.0, i) })
            .map(|i| match is_scalable {
                false => match i {
                    v if v == undefined_element_value => None,
                    0 => panic!("fixed vector mask element is not 0 or undef"),
                    _ => Some(i),
                },
                true => match i {
                    v if v == undefined_element_value => None,
                    0 => Some(i),
                    _ => panic!("sclable vector mask element is not 0 or undef"),
                },
            })
            .map(|v| v.map(|v| v as u32))
            .collect::<Vec<_>>();

        Some(mask)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExtractValue(LLVMValueRef);
impl_instruction!(ExtractValue);

impl ExtractValue {
    pub fn aggregate(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn indices(&self) -> &[u32] {
        unsafe {
            let num_indices = LLVMGetNumIndices(self.0);
            let indices = LLVMGetIndices(self.0);
            std::slice::from_raw_parts(indices, num_indices as usize)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InsertValue(LLVMValueRef);
impl_instruction!(InsertValue);

impl InsertValue {
    pub fn aggregate(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn element(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 1) })
    }

    pub fn indices(&self) -> &[u32] {
        unsafe {
            let num_indices = LLVMGetNumIndices(self.0);
            let indices = LLVMGetIndices(self.0);
            std::slice::from_raw_parts(indices, num_indices as usize)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fence(LLVMValueRef);
impl_instruction!(Fence);

impl Fence {
    pub fn atomic_ordering(&self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetOrdering(self.0) }
    }

    pub fn is_atomic_single_thread(&self) -> bool {
        unsafe { LLVMIsAtomicSingleThread(self.0) != 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CmpXchg(LLVMValueRef);
impl_instruction!(CmpXchg);

impl CmpXchg {
    pub fn address(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn cmp(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 1) })
    }

    pub fn new_value(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 2) })
    }

    pub fn success_memory_ordering(&self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetCmpXchgSuccessOrdering(self.0) }
    }

    pub fn failure_memory_ordering(&self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetCmpXchgFailureOrdering(self.0) }
    }

    pub fn is_atomic_single_thread(&self) -> bool {
        unsafe { LLVMIsAtomicSingleThread(self.0) != 0 }
    }

    pub fn is_volatile(&self) -> bool {
        unsafe { LLVMGetVolatile(self.0) != 0 }
    }

    pub fn is_weak(&self) -> bool {
        unsafe { LLVMGetWeak(self.0) != 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AtomicRMW(LLVMValueRef);
impl_instruction!(AtomicRMW);

impl AtomicRMW {
    pub fn operation(&self) -> LLVMAtomicRMWBinOp {
        unsafe { LLVMGetAtomicRMWBinOp(self.0) }
    }

    pub fn address(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn value(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 1) })
    }

    pub fn atomic_ordering(&self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetOrdering(self.0) }
    }

    pub fn is_atomic_single_thread(&self) -> bool {
        unsafe { LLVMIsAtomicSingleThread(self.0) != 0 }
    }

    pub fn is_volatile(&self) -> bool {
        unsafe { LLVMGetVolatile(self.0) != 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GetElementPtr(LLVMValueRef);
impl_instruction!(GetElementPtr);

impl GetElementPtr {
    pub fn source_element_type(&self) -> Type {
        Type::new(unsafe { LLVMGetGEPSourceElementType(self.0) })
    }

    pub fn address(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn indices(&self) -> Vec<Value> {
        let num_indices = unsafe { LLVMGetNumIndices(self.0) };
        (1..=num_indices)
            .map(|i| unsafe { LLVMGetOperand(self.0, i) })
            .map(|v| Value::new(v))
            .collect()
    }

    pub fn in_bounds(&self) -> bool {
        unsafe { LLVMIsInBounds(self.0) != 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Trunc(LLVMValueRef);
impl_instruction!(Trunc);
impl_unop!(Trunc);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ZExt(LLVMValueRef);
impl_instruction!(ZExt);
impl_unop!(ZExt);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SExt(LLVMValueRef);
impl_instruction!(SExt);
impl_unop!(SExt);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FPTrunc(LLVMValueRef);
impl_instruction!(FPTrunc);
impl_unop!(FPTrunc);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FPExt(LLVMValueRef);
impl_instruction!(FPExt);
impl_unop!(FPExt);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FPToUI(LLVMValueRef);
impl_instruction!(FPToUI);
impl_unop!(FPToUI);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FPToSI(LLVMValueRef);
impl_instruction!(FPToSI);
impl_unop!(FPToSI);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UIToFP(LLVMValueRef);
impl_instruction!(UIToFP);
impl_unop!(UIToFP);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SIToFP(LLVMValueRef);
impl_instruction!(SIToFP);
impl_unop!(SIToFP);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PtrToInt(LLVMValueRef);
impl_instruction!(PtrToInt);
impl_unop!(PtrToInt);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntToPtr(LLVMValueRef);
impl_instruction!(IntToPtr);
impl_unop!(IntToPtr);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitCast(LLVMValueRef);
impl_instruction!(BitCast);
impl_unop!(BitCast);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AddrSpaceCast(LLVMValueRef);
impl_instruction!(AddrSpaceCast);
impl_unop!(AddrSpaceCast);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ICmp(LLVMValueRef);
impl_instruction!(ICmp);

impl ICmp {
    pub fn predicate(&self) -> LLVMIntPredicate {
        unsafe { LLVMGetICmpPredicate(self.0) }
    }

    pub fn lhs(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn rhs(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 1) })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FCmp(LLVMValueRef);
impl_instruction!(FCmp);

impl FCmp {
    pub fn predicate(&self) -> LLVMRealPredicate {
        unsafe { LLVMGetFCmpPredicate(self.0) }
    }

    pub fn lhs(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn rhs(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 1) })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Phi(LLVMValueRef);
impl_instruction!(Phi);

impl Phi {
    pub fn incoming(&self) -> Vec<(BasicBlock, Value)> {
        let num_incoming_values = unsafe { LLVMCountIncoming(self.0) };

        let incoming_values = (0..num_incoming_values)
            .map(|i| unsafe { LLVMGetIncomingValue(self.0, i) })
            .map(|v| Value::new(v));

        let incoming_blocks = (0..num_incoming_values)
            .map(|i| unsafe { LLVMGetIncomingBlock(self.0, i) })
            .map(|v| BasicBlock(v));

        incoming_blocks.zip(incoming_values).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Select(LLVMValueRef);
impl_instruction!(Select);

impl Select {
    pub fn condition(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn true_value(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 1) })
    }

    pub fn false_value(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 2) })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Freeze(LLVMValueRef);
impl_instruction!(Freeze);

impl Freeze {
    pub fn value(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call(LLVMValueRef);
impl_instruction!(Call);

impl Call {
    pub fn called_value(&self) -> Value {
        Value::new(unsafe { LLVMGetCalledValue(self.0) })
    }

    pub fn called_function_type(&self) -> Type {
        Type::new(unsafe { LLVMGetCalledFunctionType(self.0) })
    }

    pub fn arguments(&self) -> Vec<Value> {
        let num_arguments = unsafe { LLVMGetNumArgOperands(self.0) };
        (0..num_arguments)
            .map(|i| unsafe { LLVMGetOperand(self.0, i) })
            .map(|v| Value::new(v))
            .collect()
    }

    pub fn calling_convention(&self) -> u32 {
        unsafe { LLVMGetFunctionCallConv(self.0) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VAArg(LLVMValueRef);
impl_instruction!(VAArg);

impl VAArg {
    pub fn argument_list(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LandingPad(LLVMValueRef);
impl_instruction!(LandingPad);

impl LandingPad {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CatchPad(LLVMValueRef);
impl_instruction!(CatchPad);

impl CatchPad {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CleanupPad(LLVMValueRef);
impl_instruction!(CleanupPad);

impl CleanupPad {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ret(LLVMValueRef);
impl_instruction!(Ret);

impl Ret {
    pub fn return_value(&self) -> Option<Value> {
        let value_ref = unsafe { LLVMGetOperand(self.0, 0) };
        match value_ref.is_null() {
            false => Some(Value::new(value_ref)),
            true => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Br(LLVMValueRef);
impl_instruction!(Br);

impl Br {
    pub fn destination(&self) -> BasicBlock {
        let value_ref = unsafe { LLVMGetOperand(self.0, 0) };
        if !unsafe { LLVMValueIsBasicBlock(value_ref) != 0 } {
            panic!("not a basic block");
        }
        BasicBlock(unsafe { LLVMValueAsBasicBlock(value_ref) })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CondBr(LLVMValueRef);
impl_instruction!(CondBr);

impl CondBr {
    pub fn condition(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn false_destination(&self) -> BasicBlock {
        let value_ref = unsafe { LLVMGetOperand(self.0, 1) };
        if !unsafe { LLVMValueIsBasicBlock(value_ref) != 0 } {
            panic!("not a basic block");
        }
        BasicBlock(unsafe { LLVMValueAsBasicBlock(value_ref) })
    }

    pub fn true_destination(&self) -> BasicBlock {
        let value_ref = unsafe { LLVMGetOperand(self.0, 2) };
        if !unsafe { LLVMValueIsBasicBlock(value_ref) != 0 } {
            panic!("not a basic block");
        }
        BasicBlock(unsafe { LLVMValueAsBasicBlock(value_ref) })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Switch(LLVMValueRef);
impl_instruction!(Switch);

impl Switch {
    pub fn condition(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn default_destination(&self) -> BasicBlock {
        BasicBlock(unsafe { LLVMGetSwitchDefaultDest(self.0) })
    }

    pub fn cases(&self) -> Vec<(Value, BasicBlock)> {
        let num_cases = unsafe { LLVMGetNumSuccessors(self.0) };

        // Operands are stored in pairs: (value, destination).
        let case_value = (1..num_cases)
            .map(|i| unsafe { LLVMGetOperand(self.0, i * 2) })
            .map(|v| Value::new(v));

        // Get destination as successor instead, these are always basic blocks.
        let case_bb = (1..num_cases)
            .map(|i| unsafe { LLVMGetSuccessor(self.0, i) })
            .map(|v| BasicBlock(v));

        case_value.zip(case_bb).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IndirectBr(LLVMValueRef);
impl_instruction!(IndirectBr);

impl IndirectBr {
    pub fn address(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }

    pub fn destinations(&self) -> Vec<BasicBlock> {
        let num_destinations = unsafe { LLVMGetNumSuccessors(self.0) };
        (0..num_destinations)
            .map(|i| unsafe { LLVMGetSuccessor(self.0, i) })
            .map(|v| BasicBlock(v))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Invoke(LLVMValueRef);
impl_instruction!(Invoke);

impl Invoke {
    pub fn called_value(&self) -> Value {
        Value::new(unsafe { LLVMGetCalledValue(self.0) })
    }

    pub fn called_function_type(&self) -> Type {
        Type::new(unsafe { LLVMGetCalledFunctionType(self.0) })
    }

    pub fn arguments(&self) -> Vec<Value> {
        let num_arguments = unsafe { LLVMGetNumArgOperands(self.0) };
        (0..num_arguments)
            .map(|i| unsafe { LLVMGetOperand(self.0, i + 1) })
            .map(|v| Value::new(v))
            .collect()
    }

    pub fn normal_destination(&self) -> BasicBlock {
        BasicBlock(unsafe { LLVMGetNormalDest(self.0) })
    }

    pub fn unwind_destination(&self) -> BasicBlock {
        BasicBlock(unsafe { LLVMGetUnwindDest(self.0) })
    }

    pub fn calling_convention(&self) -> u32 {
        unsafe { LLVMGetFunctionCallConv(self.0) }
    }

    // pub fn function_attributes(&self) -> Option<()> {}
    // pub fn argument_attributes(&self) -> Option<()> {}
    // pub fn return_attributes(&self) -> Option<()> {}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Resume(LLVMValueRef);
impl_instruction!(Resume);

impl Resume {
    pub fn exception(&self) -> Value {
        Value::new(unsafe { LLVMGetOperand(self.0, 0) })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Unreachable(LLVMValueRef);
impl_instruction!(Unreachable);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CleanupRet(LLVMValueRef);
impl_instruction!(CleanupRet);

impl CleanupRet {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CatchRet(LLVMValueRef);
impl_instruction!(CatchRet);

impl CleanupRet {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CatchSwitch(LLVMValueRef);
impl_instruction!(CatchSwitch);

impl CleanupRet {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallBr(LLVMValueRef);
impl_instruction!(CallBr);

impl CallBr {
    pub fn called_value(&self) -> Value {
        Value::new(unsafe { LLVMGetCalledValue(self.0) })
    }

    pub fn called_function_type(&self) -> Type {
        Type::new(unsafe { LLVMGetCalledFunctionType(self.0) })
    }

    pub fn arguments(&self) -> Vec<Value> {
        let num_arguments = unsafe { LLVMGetNumArgOperands(self.0) };
        (0..num_arguments)
            .map(|i| unsafe { LLVMGetOperand(self.0, i + 1) })
            .map(|v| Value::new(v).into())
            .collect()
    }

    pub fn calling_convention(&self) -> u32 {
        unsafe { LLVMGetFunctionCallConv(self.0) }
    }

    // pub fn function_attributes(&self) -> Option<()> {}
    // pub fn argument_attributes(&self) -> Option<()> {}
    // pub fn return_attributes(&self) -> Option<()> {}
}
