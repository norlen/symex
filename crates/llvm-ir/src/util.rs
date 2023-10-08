use std::ffi::CString;

use llvm_sys::{
    core::{
        LLVMGetDebugLocColumn, LLVMGetDebugLocDirectory, LLVMGetDebugLocFilename,
        LLVMGetDebugLocLine,
    },
    prelude::LLVMValueRef,
};

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DebugLocation {
    pub line: u32,
    pub column: Option<u32>,
    pub filename: CString,
    pub directory: Option<CString>,
}

impl std::fmt::Display for DebugLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Fixme
        match &self.directory {
            Some(directory) => write!(
                f,
                "{}:{}:{}",
                directory.to_str().unwrap(),
                self.filename.to_str().unwrap(),
                self.line
            ),
            None => write!(f, "{}:{}", self.filename.to_str().unwrap(), self.line),
        }
    }
}

impl DebugLocation {
    pub fn new(
        line: u32,
        column: Option<u32>,
        filename: CString,
        directory: Option<CString>,
    ) -> Self {
        Self {
            line,
            column,
            filename,
            directory,
        }
    }
}

pub(crate) fn debug_location_with_column(value_ref: LLVMValueRef) -> Option<DebugLocation> {
    let filename = unsafe {
        let mut len = 0;
        let ptr = LLVMGetDebugLocFilename(value_ref, &mut len);
        match ptr.is_null() {
            true => None,
            false => Some(core::ffi::CStr::from_ptr(ptr)),
        }
    };

    if let Some(filename) = filename {
        let filename = filename.to_owned();
        let line = unsafe { LLVMGetDebugLocLine(value_ref) };
        let column = unsafe { LLVMGetDebugLocColumn(value_ref) };
        let directory = unsafe {
            let mut len = 0;
            let ptr = LLVMGetDebugLocDirectory(value_ref, &mut len);
            match ptr.is_null() {
                true => None,
                false => Some(core::ffi::CStr::from_ptr(ptr).to_owned()),
            }
        };

        Some(DebugLocation::new(line, Some(column), filename, directory))
    } else {
        None
    }
}

pub(crate) fn debug_location_without_column(value_ref: LLVMValueRef) -> Option<DebugLocation> {
    let filename = unsafe {
        let mut len = 0;
        let ptr = LLVMGetDebugLocFilename(value_ref, &mut len);
        match ptr.is_null() {
            true => None,
            false => Some(core::ffi::CStr::from_ptr(ptr)),
        }
    };

    if let Some(filename) = filename {
        let filename = filename.to_owned();
        let line = unsafe { LLVMGetDebugLocLine(value_ref) };
        let directory = unsafe {
            let mut len = 0;
            let ptr = LLVMGetDebugLocDirectory(value_ref, &mut len);
            match ptr.is_null() {
                true => None,
                false => Some(core::ffi::CStr::from_ptr(ptr).to_owned()),
            }
        };

        Some(DebugLocation::new(line, None, filename, directory))
    } else {
        None
    }
}
