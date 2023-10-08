use std::ffi::CStr;

use llvm_sys::{
    core::{
        LLVMCountParamTypes, LLVMCountStructElementTypes, LLVMGetArrayLength2, LLVMGetElementType,
        LLVMGetIntTypeWidth, LLVMGetParamTypes, LLVMGetPointerAddressSpace, LLVMGetReturnType,
        LLVMGetStructElementTypes, LLVMGetStructName, LLVMGetTypeKind, LLVMGetVectorSize,
        LLVMIsFunctionVarArg, LLVMIsLiteralStruct, LLVMIsOpaqueStruct,
    },
    prelude::*,
    LLVMTypeKind,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Void,
    Integer(IntegerType),
    Float(FloatingPointType),
    Pointer(PointerType),
    Vector(VectorType),
    Array(ArrayType),
    Structure(StructureType),
    OpaqueStructure,
    Function(FunctionType),
    Label,
    Token,
    Metadata,
    X86Amx,
    X86Mmx,
    TargetExtension(TargetExtensionType),
}

impl Type {
    pub(crate) fn new(ty: LLVMTypeRef) -> Self {
        let kind = unsafe { LLVMGetTypeKind(ty) };
        match kind {
            LLVMTypeKind::LLVMVoidTypeKind => Type::Void,
            LLVMTypeKind::LLVMHalfTypeKind => Type::Float(FloatingPointType::Half),
            LLVMTypeKind::LLVMFloatTypeKind => Type::Float(FloatingPointType::Float),
            LLVMTypeKind::LLVMDoubleTypeKind => Type::Float(FloatingPointType::Double),
            LLVMTypeKind::LLVMX86_FP80TypeKind => Type::Float(FloatingPointType::X86Fp80),
            LLVMTypeKind::LLVMFP128TypeKind => Type::Float(FloatingPointType::Fp128),
            LLVMTypeKind::LLVMPPC_FP128TypeKind => Type::Float(FloatingPointType::PpcFp128),
            LLVMTypeKind::LLVMLabelTypeKind => Type::Label,
            LLVMTypeKind::LLVMIntegerTypeKind => Type::Integer(IntegerType::new(ty)),
            LLVMTypeKind::LLVMFunctionTypeKind => Type::Function(FunctionType::new(ty)),
            LLVMTypeKind::LLVMStructTypeKind => match unsafe { LLVMIsOpaqueStruct(ty) != 0 } {
                true => Type::OpaqueStructure,
                false => Type::Structure(StructureType::new(ty)),
            },
            LLVMTypeKind::LLVMArrayTypeKind => Type::Array(ArrayType::new(ty)),
            LLVMTypeKind::LLVMPointerTypeKind => Type::Pointer(PointerType::new(ty)),
            LLVMTypeKind::LLVMVectorTypeKind => Type::Vector(VectorType::new(ty, false)),
            LLVMTypeKind::LLVMMetadataTypeKind => Type::Metadata,
            LLVMTypeKind::LLVMX86_MMXTypeKind => Type::X86Mmx,
            LLVMTypeKind::LLVMTokenTypeKind => Type::Token,
            LLVMTypeKind::LLVMScalableVectorTypeKind => Type::Vector(VectorType::new(ty, true)),
            LLVMTypeKind::LLVMBFloatTypeKind => Type::Float(FloatingPointType::BFloat),
            LLVMTypeKind::LLVMX86_AMXTypeKind => Type::X86Amx,
            LLVMTypeKind::LLVMTargetExtTypeKind => {
                Type::TargetExtension(TargetExtensionType::new(ty))
            }
        }
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, Type::Pointer(_))
    }

    /// Returns `true` if the type is an integer.
    pub fn is_integer(&self) -> bool {
        matches!(self, Type::Integer(_))
    }

    /// Returns `true` if the type is an integer with the specified number of bits.
    pub fn is_integer_with_size(&self, bits: u32) -> bool {
        matches!(self, Type::Integer(t) if t.bits() == bits)
    }

    /// Returns `true` if the type is a vector of integers.
    pub fn is_vector_int(&self) -> bool {
        matches!(self, Type::Vector(t) if t.element_type().is_integer())
    }

    /// Returns `true` if the type is a floating point type.
    pub fn is_fp(&self) -> bool {
        matches!(self, Type::Float(_))
    }

    /// Returns `true` if the type is a vector of floating point numbers.
    pub fn is_vector_fp(&self) -> bool {
        matches!(self, Type::Vector(t) if t.element_type().is_fp())
    }

    /// Returns `true` if the type is ["first class"](https://llvm.org/docs/LangRef.html#t-firstclass)
    /// , i.e., it is a valid type for a `Value`.
    ///
    /// A first class value is currently all types except for `void` and `function`.
    pub fn is_first_class(&self) -> bool {
        !matches!(self, Type::Void | Type::Function(_))
    }

    /// Returns `true` if the type can be part of an array.
    pub fn is_valid_element_type(&self) -> bool {
        !matches!(
            self,
            Type::Void | Type::Label | Type::Metadata | Type::Function(_) | Type::X86Amx
        ) || !matches!(self, Type::Vector(t) if t.is_scalable())
    }

    /// Returns `true` if the type is a function.
    pub fn is_function(&self) -> bool {
        matches!(self, Type::Function(_))
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Integer(t) => write!(f, "{}", t),
            Type::Float(t) => write!(f, "{}", t),
            Type::Pointer(t) => write!(f, "{}", t),
            Type::Vector(t) => write!(f, "{}", t),
            Type::Array(t) => write!(f, "{}", t),
            Type::Structure(t) => write!(f, "{}", t),
            Type::OpaqueStructure => write!(f, "type opaque"),
            Type::Function(t) => write!(f, "{}", t),
            Type::Label => write!(f, "label"),
            Type::Token => write!(f, "token"),
            Type::Metadata => write!(f, "metadata"),
            Type::X86Amx => write!(f, "x86_amx"),
            Type::X86Mmx => write!(f, "x86_mmx"),
            Type::TargetExtension(t) => write!(f, "{}", t),
        }
    }
}

/// Integer type.
///
/// Simple type that specifies an arbitrary bid-width for the integer type. Bit widths from `1` to
/// `2^23` can be specified.
///
/// [LLVM Reference](https://llvm.org/docs/LangRef.html#integer-type)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntegerType(LLVMTypeRef);

impl IntegerType {
    pub(crate) fn new(type_ref: LLVMTypeRef) -> Self {
        Self(type_ref)
    }

    /// Minimum number of bits for an integer type.
    pub const MIN_BITS: u32 = 1;

    /// Maximum number of bits for an integer type.
    pub const MAX_BITS: u32 = 8388608;

    pub fn bits(&self) -> u32 {
        unsafe { LLVMGetIntTypeWidth(self.0) }
    }
}

impl std::fmt::Display for IntegerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "i{}", self.bits())
    }
}

/// Floating-point type.
///
/// [LLVM Reference](https://llvm.org/docs/LangRef.html#floating-point-types)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatingPointType {
    /// 16-bit floating point type.
    Half,

    /// 16-bit floating point type (7-bit significant).
    BFloat,

    /// 32-bit floating point type.
    Float,

    /// 64-bit floating point type.
    Double,

    /// 128-bit floating point type (112-bit significand).
    Fp128,

    /// 80-bit floating point type (X87).
    X86Fp80,

    /// 128-bit floating point type (two 64-bits, PowerPC).
    PpcFp128,
}

impl FloatingPointType {
    pub fn is_ieee_like(&self) -> bool {
        match self {
            FloatingPointType::Half
            | FloatingPointType::BFloat
            | FloatingPointType::Float
            | FloatingPointType::Double
            | FloatingPointType::Fp128 => true,
            FloatingPointType::X86Fp80 | FloatingPointType::PpcFp128 => false,
        }
    }

    pub fn bits(&self) -> u32 {
        match self {
            FloatingPointType::Half => 16,
            FloatingPointType::BFloat => 16,
            FloatingPointType::Float => 32,
            FloatingPointType::Double => 64,
            FloatingPointType::Fp128 => 128,
            FloatingPointType::X86Fp80 => 80,
            FloatingPointType::PpcFp128 => 128,
        }
    }
}

impl std::fmt::Display for FloatingPointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatingPointType::Half => write!(f, "half"),
            FloatingPointType::BFloat => write!(f, "bfloat"),
            FloatingPointType::Float => write!(f, "float"),
            FloatingPointType::Double => write!(f, "double"),
            FloatingPointType::Fp128 => write!(f, "fp128"),
            FloatingPointType::X86Fp80 => write!(f, "x86_fp80"),
            FloatingPointType::PpcFp128 => write!(f, "ppc_fp128"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointerType(LLVMTypeRef);

impl PointerType {
    pub(crate) fn new(type_ref: LLVMTypeRef) -> Self {
        Self(type_ref)
    }

    pub fn address_space(&self) -> u32 {
        unsafe { LLVMGetPointerAddressSpace(self.0) }
    }
}

impl std::fmt::Display for PointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.address_space() {
            0 => write!(f, "ptr"),
            n => write!(f, "ptr addrspace({})", n),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType(LLVMTypeRef);

impl ArrayType {
    pub(crate) fn new(type_ref: LLVMTypeRef) -> Self {
        Self(type_ref)
    }

    pub fn element_type(&self) -> Type {
        let type_ref = unsafe { LLVMGetElementType(self.0) };
        Type::new(type_ref)
    }

    pub fn num_elements(&self) -> u64 {
        unsafe { LLVMGetArrayLength2(self.0) }
    }
}

impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let element_type = self.element_type();
        let num_elements = self.num_elements();
        write!(f, "[{} x {}]", num_elements, element_type)
    }
}

/// Vector type is a simple type that represents a vector of elements ([Vector Type](https://llvm.org/docs/LangRef.html#vector-type))
///
/// Vector types are used when multiple primitive values are operated on in parallel using SIMD
/// instructions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorType {
    type_ref: LLVMTypeRef,
    is_scalable: bool,
}

impl VectorType {
    pub(crate) fn new(type_ref: LLVMTypeRef, is_scalable: bool) -> Self {
        Self {
            type_ref,
            is_scalable,
        }
    }

    pub fn element_type(&self) -> Type {
        let type_ref = unsafe { LLVMGetElementType(self.type_ref) };
        Type::new(type_ref)
    }

    pub fn num_elements(&self) -> u32 {
        unsafe { LLVMGetVectorSize(self.type_ref) }
    }

    pub fn is_scalable(&self) -> bool {
        self.is_scalable
    }
}

impl std::fmt::Display for VectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let element_type = self.element_type();
        let num_elements = self.num_elements();
        if self.is_scalable {
            write!(f, "<vscale x {}>", element_type)
        } else {
            write!(f, "<{} x {}>", num_elements, element_type)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructureType(LLVMTypeRef);

impl StructureType {
    pub(crate) fn new(type_ref: LLVMTypeRef) -> Self {
        Self(type_ref)
    }

    pub fn name(&self) -> Option<&CStr> {
        let name = unsafe { LLVMGetStructName(self.0) };
        if name.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(name) })
        }
    }

    pub fn is_opaque(&self) -> bool {
        unsafe { LLVMIsOpaqueStruct(self.0) != 0 }
    }

    pub fn is_literal_struct(&self) -> bool {
        unsafe { LLVMIsLiteralStruct(self.0) != 0 }
    }

    pub fn fields(&self) -> Vec<Type> {
        let num_elements = unsafe { LLVMCountStructElementTypes(self.0) };

        let mut fields = Vec::with_capacity(num_elements as usize);
        unsafe {
            LLVMGetStructElementTypes(self.0, fields.as_mut_ptr());
            fields.set_len(num_elements as usize);
        };
        fields.into_iter().map(Type::new).collect()
    }
}

impl std::fmt::Display for StructureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name();
        let fields = self.fields();

        if let Some(name) = name {
            write!(
                f,
                "%{} = type {{ {} }}",
                name.to_str().unwrap(),
                fields
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            write!(
                f,
                "{{ {} }}",
                fields
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

/// Function declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType(LLVMTypeRef);

impl FunctionType {
    pub(crate) fn new(type_ref: LLVMTypeRef) -> Self {
        Self(type_ref)
    }

    pub fn return_type(&self) -> Type {
        let type_ref = unsafe { LLVMGetReturnType(self.0) };
        Type::new(type_ref)
    }

    pub fn param_types(&self) -> Vec<Type> {
        let num_params = unsafe { LLVMCountParamTypes(self.0) };
        let mut param_types = Vec::with_capacity(num_params as usize);
        unsafe {
            LLVMGetParamTypes(self.0, param_types.as_mut_ptr());
            param_types.set_len(num_params as usize);
        }
        param_types.into_iter().map(Type::new).collect()
    }

    pub fn is_var_arg(&self) -> bool {
        unsafe { LLVMIsFunctionVarArg(self.0) != 0 }
    }
}

impl std::fmt::Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let return_type = self.return_type();
        let param_types = self.param_types();
        write!(
            f,
            "{} ({})",
            return_type,
            param_types
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// Target extension type represent a type that must be preserved through optimization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetExtensionType(LLVMTypeRef);

impl TargetExtensionType {
    pub(crate) fn new(type_ref: LLVMTypeRef) -> Self {
        Self(type_ref)
    }
}

impl std::fmt::Display for TargetExtensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<target extension>")
    }
}
