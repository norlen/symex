use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    os::unix::prelude::OsStrExt,
    path::Path,
};

use llvm_sys::{
    bit_reader::LLVMParseBitcodeInContext2,
    core::{
        LLVMContextCreate, LLVMCreateMemoryBufferWithContentsOfFile, LLVMDisposeMemoryBuffer,
        LLVMGetDataLayoutStr, LLVMGetFirstFunction, LLVMGetFirstGlobal, LLVMGetFirstGlobalAlias,
        LLVMGetFirstGlobalIFunc, LLVMGetModuleIdentifier, LLVMGetNextFunction, LLVMGetNextGlobal,
        LLVMGetNextGlobalAlias, LLVMGetNextGlobalIFunc, LLVMGetSourceFileName, LLVMGetTarget,
    },
    prelude::*,
};

use crate::{Function, GlobalAlias, GlobalIFunc, GlobalVariable};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ModuleError {
    /// Failed to load LLVM bitcode or IR file.
    #[error("Failed to load LLVM ir: {0:?}")]
    FailedToLoad(CString),
}

pub struct Module(LLVMModuleRef);

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Module").field(&self.0).finish()
    }
}

impl Module {
    pub fn load_bc(path: impl AsRef<Path>) -> Result<Self, ModuleError> {
        let path = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();

        let module = unsafe {
            let mut buffer = std::ptr::null_mut();
            let mut error = std::mem::zeroed();

            let return_code = LLVMCreateMemoryBufferWithContentsOfFile(
                path.as_ptr() as *const _,
                &mut buffer,
                &mut error,
            );
            if return_code != 0 {
                return Err(ModuleError::FailedToLoad(CString::from_raw(error)));
            }

            let ctx = LLVMContextCreate();
            let mut module: MaybeUninit<LLVMModuleRef> = MaybeUninit::uninit();
            let return_code = LLVMParseBitcodeInContext2(ctx, buffer, module.as_mut_ptr());
            LLVMDisposeMemoryBuffer(buffer);
            if return_code != 0 {
                todo!("Failed to parse bitcode");
            }

            module.assume_init()
        };

        if module.is_null() {
            panic!("");
        }
        Ok(Self(module))
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ModuleError> {
        let path = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();

        let mut memory_buffer = core::ptr::null_mut();
        let mut err_string = MaybeUninit::uninit();

        let success = unsafe {
            LLVMCreateMemoryBufferWithContentsOfFile(
                path.as_ptr() as *const _,
                &mut memory_buffer,
                err_string.as_mut_ptr(),
            )
        };
        if success != 0 {
            eprintln!("LLVMCreateMemoryBufferWithContentsOfFile failed");
            unsafe {
                return Err(ModuleError::FailedToLoad(CString::from_raw(
                    err_string.assume_init(),
                )));
            }
        }

        let mut module: MaybeUninit<LLVMModuleRef> = MaybeUninit::uninit();
        let success = unsafe {
            let ctx = LLVMContextCreate();
            let return_code = LLVMParseBitcodeInContext2(ctx, memory_buffer, module.as_mut_ptr());
            // LLVMDisposeMemoryBuffer(buffer);
            return_code
        };

        if success != 0 {
            todo!("Failed to parse bitcode");
        }

        let module = unsafe { module.assume_init() };
        Ok(Self(module))
    }

    // fn load_module(path: &Path, is_bc: bool) -> LLVMModuleRef {}

    pub fn identifier(&self) -> &CStr {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetModuleIdentifier(self.0, &mut len);
            CStr::from_ptr(ptr)
        }
    }

    pub fn source_filename(&self) -> &CStr {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetSourceFileName(self.0, &mut len);
            CStr::from_ptr(ptr)
        }
    }

    pub fn datalayout_str(&self) -> &CStr {
        unsafe {
            let ptr = LLVMGetDataLayoutStr(self.0);
            CStr::from_ptr(ptr)
        }
    }

    pub fn target_triple(&self) -> &CStr {
        unsafe {
            let ptr = LLVMGetTarget(self.0);
            CStr::from_ptr(ptr)
        }
    }

    pub fn functions(&self) -> FunctionIter {
        unsafe { FunctionIter::new(self.0) }
    }

    pub fn globals(&self) -> GlobalIter {
        unsafe { GlobalIter::new(self.0) }
    }
}

macro_rules! impl_iter {
    ($name:ident, $src:ty, $item:ty, $init:ident, $next:ident, $target:ty) => {
        pub struct $name(LLVMValueRef);

        impl $name {
            pub unsafe fn new(src: $src) -> Self {
                Self(unsafe { $init(src) })
            }
        }

        impl Iterator for $name {
            type Item = $target;

            fn next(&mut self) -> Option<Self::Item> {
                if self.0.is_null() {
                    None
                } else {
                    let current = self.0;
                    self.0 = unsafe { $next(self.0) };
                    Some(current.into())
                }
            }
        }
    };
}

impl_iter!(
    FunctionIter,
    LLVMModuleRef,
    LLVMValueRef,
    LLVMGetFirstFunction,
    LLVMGetNextFunction,
    Function
);
impl_iter!(
    GlobalIter,
    LLVMModuleRef,
    LLVMValueRef,
    LLVMGetFirstGlobal,
    LLVMGetNextGlobal,
    GlobalVariable
);
impl_iter!(
    GlobalAliasIter,
    LLVMModuleRef,
    LLVMValueRef,
    LLVMGetFirstGlobalAlias,
    LLVMGetNextGlobalAlias,
    GlobalAlias
);
impl_iter!(
    GlobalIFuncIter,
    LLVMModuleRef,
    LLVMValueRef,
    LLVMGetFirstGlobalIFunc,
    LLVMGetNextGlobalIFunc,
    GlobalIFunc
);
