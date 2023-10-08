use std::ffi::CStr;

use llvm_sys::{
    core::{
        LLVMGetAlignment, LLVMGetDLLStorageClass, LLVMGetFirstBasicBlock, LLVMGetFunctionCallConv,
        LLVMGetGC, LLVMGetIntrinsicID, LLVMGetLinkage, LLVMGetPersonalityFn, LLVMGetSection,
        LLVMGetThreadLocalMode, LLVMGetUnnamedAddress, LLVMGetValueKind, LLVMGetValueName2,
        LLVMGetVisibility, LLVMGlobalGetValueType, LLVMHasPersonalityFn, LLVMIntrinsicGetName,
        LLVMIntrinsicIsOverloaded, LLVMIsExternallyInitialized, LLVMIsGlobalConstant,
        LLVMIsThreadLocal, LLVMPrintValueToString, LLVMTypeOf,
    },
    prelude::*,
    LLVMDLLStorageClass, LLVMLinkage, LLVMThreadLocalMode, LLVMUnnamedAddr, LLVMValueKind,
    LLVMVisibility,
};

use crate::{instruction::BasicBlock, types::Type, Value};

pub trait GlobalValue {
    fn is_declaration(&self) -> bool;

    fn value_type(&self) -> Type;

    fn linkage(&self) -> LLVMLinkage;

    fn visibility(&self) -> LLVMVisibility;

    fn section(&self) -> Option<&CStr>;

    fn dll_storage_class(&self) -> LLVMDLLStorageClass;

    fn unnamed_addr(&self) -> LLVMUnnamedAddr;

    fn alignment(&self) -> u32;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Global {
    // Function(Function),
    Variable(GlobalVariable),
    Alias(GlobalAlias),
    IFunc(GlobalIFunc),
}

impl Global {
    pub(crate) fn new(value_ref: LLVMValueRef) -> Self {
        let value_kind = unsafe { LLVMGetValueKind(value_ref) };

        match value_kind {
            // LLVMValueKind::LLVMFunctionValueKind => Self::Function(Function::new(value_ref)),
            LLVMValueKind::LLVMGlobalAliasValueKind => Self::Alias(GlobalAlias::new(value_ref)),
            LLVMValueKind::LLVMGlobalIFuncValueKind => Self::IFunc(GlobalIFunc::new(value_ref)),
            LLVMValueKind::LLVMGlobalVariableValueKind => {
                Self::Variable(GlobalVariable::new(value_ref))
            }
            _ => panic!("Not a global value"),
        }
    }

    pub fn ty(&self) -> Type {
        match self {
            Global::Variable(g) => g.ty(),
            Global::Alias(g) => g.ty(),
            Global::IFunc(g) => g.ty(),
        }
    }
}

impl GlobalValue for Global {
    fn is_declaration(&self) -> bool {
        match self {
            // Global::Function(g) => g.is_declaration(),
            Global::Variable(g) => g.is_declaration(),
            Global::Alias(g) => g.is_declaration(),
            Global::IFunc(g) => g.is_declaration(),
        }
    }

    fn value_type(&self) -> Type {
        match self {
            // Global::Function(g) => g.value_type(),
            Global::Variable(g) => g.value_type(),
            Global::Alias(g) => g.value_type(),
            Global::IFunc(g) => g.value_type(),
        }
    }

    fn linkage(&self) -> LLVMLinkage {
        match self {
            // Global::Function(g) => g.linkage(),
            Global::Variable(g) => g.linkage(),
            Global::Alias(g) => g.linkage(),
            Global::IFunc(g) => g.linkage(),
        }
    }

    fn visibility(&self) -> LLVMVisibility {
        match self {
            // Global::Function(g) => g.visibility(),
            Global::Variable(g) => g.visibility(),
            Global::Alias(g) => g.visibility(),
            Global::IFunc(g) => g.visibility(),
        }
    }

    fn section(&self) -> Option<&CStr> {
        match self {
            // Global::Function(g) => g.section(),
            Global::Variable(g) => g.section(),
            Global::Alias(g) => g.section(),
            Global::IFunc(g) => g.section(),
        }
    }

    fn dll_storage_class(&self) -> LLVMDLLStorageClass {
        match self {
            // Global::Function(g) => g.dll_storage_class(),
            Global::Variable(g) => g.dll_storage_class(),
            Global::Alias(g) => g.dll_storage_class(),
            Global::IFunc(g) => g.dll_storage_class(),
        }
    }

    fn unnamed_addr(&self) -> LLVMUnnamedAddr {
        match self {
            // Global::Function(g) => g.unnamed_addr(),
            Global::Variable(g) => g.unnamed_addr(),
            Global::Alias(g) => g.unnamed_addr(),
            Global::IFunc(g) => g.unnamed_addr(),
        }
    }

    fn alignment(&self) -> u32 {
        match self {
            // Global::Function(g) => g.alignment(),
            Global::Variable(g) => g.alignment(),
            Global::Alias(g) => g.alignment(),
            Global::IFunc(g) => g.alignment(),
        }
    }
}

impl std::fmt::Display for Global {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Global::Variable(v) => write!(f, "{}", v),
            Global::Alias(v) => write!(f, "{}", v),
            Global::IFunc(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Argument(LLVMValueRef);

impl From<Argument> for Value {
    fn from(value: Argument) -> Self {
        Value::Argument(value)
    }
}

impl Argument {
    pub(crate) fn new(value_ref: LLVMValueRef) -> Self {
        Self(value_ref)
    }

    pub fn ty(&self) -> Type {
        Type::new(unsafe { LLVMTypeOf(self.0) })
    }
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = {
            let ptr = unsafe { LLVMPrintValueToString(self.0) };
            unsafe { CStr::from_ptr(ptr) }
        };
        write!(f, "{}", s.to_string_lossy())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Function(LLVMValueRef);

impl Function {
    pub(crate) fn new(value_ref: LLVMValueRef) -> Self {
        Self(value_ref)
    }

    pub fn ty(&self) -> Type {
        Type::new(unsafe { LLVMTypeOf(self.0) })
    }

    pub fn name(&self) -> &CStr {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetValueName2(self.0, &mut len);
            CStr::from_ptr(ptr)
        }
    }

    pub fn has_personality_function(&self) -> bool {
        unsafe { LLVMHasPersonalityFn(self.0) != 0 }
    }

    pub fn personality_fn(&self) -> Option<LLVMValueRef> {
        if self.has_personality_function() {
            let personality_fn = unsafe { LLVMGetPersonalityFn(self.0) };
            Some(personality_fn)
        } else {
            None
        }
    }

    pub fn calling_convention(&self) -> u32 {
        unsafe { LLVMGetFunctionCallConv(self.0) }
    }

    pub fn instrinsic_id(&self) -> u32 {
        unsafe { LLVMGetIntrinsicID(self.0) }
    }

    pub fn intrinsic_name(&self) -> &CStr {
        let id = self.instrinsic_id();
        unsafe {
            let mut len = 0;
            let ptr = LLVMIntrinsicGetName(id, &mut len);
            // TODO: null?
            CStr::from_ptr(ptr)
        }
    }

    pub fn is_intrinsic_overloaded(&self) -> bool {
        let id = self.instrinsic_id();
        unsafe { LLVMIntrinsicIsOverloaded(id) != 0 }
    }

    pub fn gc(&self) -> Option<&CStr> {
        unsafe {
            let ptr = LLVMGetGC(self.0);
            if !ptr.is_null() {
                Some(CStr::from_ptr(ptr))
            } else {
                None
            }
        }
    }

    pub fn first_basic_block(&self) -> Option<BasicBlock> {
        let bb = unsafe { LLVMGetFirstBasicBlock(self.0) };
        if bb.is_null() {
            None
        } else {
            Some(BasicBlock::new(bb))
        }
    }
}

impl From<LLVMValueRef> for Function {
    fn from(value: LLVMValueRef) -> Self {
        Self(value)
    }
}

impl GlobalValue for Function {
    fn is_declaration(&self) -> bool {
        todo!()
    }

    fn value_type(&self) -> Type {
        let type_ref = unsafe { LLVMGlobalGetValueType(self.0) };
        Type::new(type_ref)
    }

    fn linkage(&self) -> LLVMLinkage {
        unsafe { LLVMGetLinkage(self.0) }
    }

    fn visibility(&self) -> LLVMVisibility {
        unsafe { LLVMGetVisibility(self.0) }
    }

    fn section(&self) -> Option<&CStr> {
        unsafe {
            let ptr = LLVMGetSection(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    fn dll_storage_class(&self) -> LLVMDLLStorageClass {
        unsafe { LLVMGetDLLStorageClass(self.0) }
    }

    fn unnamed_addr(&self) -> LLVMUnnamedAddr {
        unsafe { LLVMGetUnnamedAddress(self.0) }
    }

    fn alignment(&self) -> u32 {
        unsafe { LLVMGetAlignment(self.0) }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = {
            let ptr = unsafe { LLVMPrintValueToString(self.0) };
            unsafe { CStr::from_ptr(ptr) }
        };
        write!(f, "{}", s.to_string_lossy())
    }
}

/// Global variables define regions of memory allocated at compile time.
///
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalVariable(LLVMValueRef);

impl GlobalVariable {
    pub(crate) fn new(value_ref: LLVMValueRef) -> Self {
        Self(value_ref)
    }
}

impl From<LLVMValueRef> for GlobalVariable {
    fn from(value: LLVMValueRef) -> Self {
        Self(value)
    }
}

impl GlobalVariable {
    pub fn name(&self) -> &CStr {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetValueName2(self.0, &mut len);
            CStr::from_ptr(ptr)
        }
    }

    pub fn ty(&self) -> Type {
        Type::new(unsafe { LLVMTypeOf(self.0) })
    }

    pub fn is_constant(&self) -> bool {
        unsafe { LLVMIsGlobalConstant(self.0) != 0 }
    }

    pub fn is_thread_local(&self) -> bool {
        unsafe { LLVMIsThreadLocal(self.0) != 0 }
    }

    pub fn is_externally_initialized(&self) -> bool {
        unsafe { LLVMIsExternallyInitialized(self.0) != 0 }
    }

    pub fn address_space(&self) -> u32 {
        let t = self.ty();
        let Type::Pointer(t) = t else {
            panic!("Global is not a pointer");
        };
        t.address_space()
    }

    pub fn thread_local_mode(&self) -> LLVMThreadLocalMode {
        unsafe { LLVMGetThreadLocalMode(self.0) }
    }

    pub fn preemption_specifier(&self) -> u32 {
        todo!()
    }

    pub fn initializer(&self) -> Option<Value> {
        todo!()
    }

    pub fn comdat(&self) -> Option<&CStr> {
        todo!()
    }

    pub fn partition(&self) -> Option<&CStr> {
        todo!()
    }

    pub fn debug_location(&self) -> Option<&CStr> {
        todo!()
    }
}

impl GlobalValue for GlobalVariable {
    fn is_declaration(&self) -> bool {
        todo!()
    }

    fn value_type(&self) -> Type {
        let type_ref = unsafe { LLVMGlobalGetValueType(self.0) };
        Type::new(type_ref)
    }

    fn linkage(&self) -> LLVMLinkage {
        unsafe { LLVMGetLinkage(self.0) }
    }

    fn visibility(&self) -> LLVMVisibility {
        unsafe { LLVMGetVisibility(self.0) }
    }

    fn section(&self) -> Option<&CStr> {
        unsafe {
            let ptr = LLVMGetSection(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    fn dll_storage_class(&self) -> LLVMDLLStorageClass {
        unsafe { LLVMGetDLLStorageClass(self.0) }
    }

    fn unnamed_addr(&self) -> LLVMUnnamedAddr {
        unsafe { LLVMGetUnnamedAddress(self.0) }
    }

    fn alignment(&self) -> u32 {
        unsafe { LLVMGetAlignment(self.0) }
    }
}

impl std::fmt::Display for GlobalVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = {
            let ptr = unsafe { LLVMPrintValueToString(self.0) };
            unsafe { CStr::from_ptr(ptr) }
        };
        write!(f, "{}", s.to_string_lossy())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalAlias(LLVMValueRef);

impl GlobalAlias {
    pub(crate) fn new(value_ref: LLVMValueRef) -> Self {
        Self(value_ref)
    }

    pub fn ty(&self) -> Type {
        Type::new(unsafe { LLVMTypeOf(self.0) })
    }
}

impl From<LLVMValueRef> for GlobalAlias {
    fn from(value: LLVMValueRef) -> Self {
        Self(value)
    }
}

impl GlobalValue for GlobalAlias {
    fn is_declaration(&self) -> bool {
        todo!()
    }

    fn value_type(&self) -> Type {
        let type_ref = unsafe { LLVMGlobalGetValueType(self.0) };
        Type::new(type_ref)
    }

    fn linkage(&self) -> LLVMLinkage {
        unsafe { LLVMGetLinkage(self.0) }
    }

    fn visibility(&self) -> LLVMVisibility {
        unsafe { LLVMGetVisibility(self.0) }
    }

    fn section(&self) -> Option<&CStr> {
        unsafe {
            let ptr = LLVMGetSection(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    fn dll_storage_class(&self) -> LLVMDLLStorageClass {
        unsafe { LLVMGetDLLStorageClass(self.0) }
    }

    fn unnamed_addr(&self) -> LLVMUnnamedAddr {
        unsafe { LLVMGetUnnamedAddress(self.0) }
    }

    fn alignment(&self) -> u32 {
        unsafe { LLVMGetAlignment(self.0) }
    }
}

impl std::fmt::Display for GlobalAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = {
            let ptr = unsafe { LLVMPrintValueToString(self.0) };
            unsafe { CStr::from_ptr(ptr) }
        };
        write!(f, "{}", s.to_string_lossy())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalIFunc(LLVMValueRef);

impl GlobalIFunc {
    pub(crate) fn new(value_ref: LLVMValueRef) -> Self {
        Self(value_ref)
    }

    pub fn ty(&self) -> Type {
        Type::new(unsafe { LLVMTypeOf(self.0) })
    }
}

impl From<LLVMValueRef> for GlobalIFunc {
    fn from(value: LLVMValueRef) -> Self {
        Self(value)
    }
}

impl GlobalValue for GlobalIFunc {
    fn is_declaration(&self) -> bool {
        todo!()
    }

    fn value_type(&self) -> Type {
        let type_ref = unsafe { LLVMGlobalGetValueType(self.0) };
        Type::new(type_ref)
    }

    fn linkage(&self) -> LLVMLinkage {
        unsafe { LLVMGetLinkage(self.0) }
    }

    fn visibility(&self) -> LLVMVisibility {
        unsafe { LLVMGetVisibility(self.0) }
    }

    fn section(&self) -> Option<&CStr> {
        unsafe {
            let ptr = LLVMGetSection(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    fn dll_storage_class(&self) -> LLVMDLLStorageClass {
        unsafe { LLVMGetDLLStorageClass(self.0) }
    }

    fn unnamed_addr(&self) -> LLVMUnnamedAddr {
        unsafe { LLVMGetUnnamedAddress(self.0) }
    }

    fn alignment(&self) -> u32 {
        unsafe { LLVMGetAlignment(self.0) }
    }
}

impl std::fmt::Display for GlobalIFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = {
            let ptr = unsafe { LLVMPrintValueToString(self.0) };
            unsafe { CStr::from_ptr(ptr) }
        };
        write!(f, "{}", s.to_string_lossy())
    }
}
