use crate::DExpr;

/// Result for a single path of execution.
///
/// This contains which path it was, if it succeded or not. If it failed the error will have a
/// stack trace to where the error occured.
///
/// All input variables and variables used in `symbolic` calls will also have solutions available.
#[derive(Debug)]
pub struct PathResult {
    /// Which path this is.
    pub path: usize,

    /// The final value from the path.
    ///
    /// If the path failed the reason vill be in the error. Otherwise there will be a value
    /// unless the analyzed function returned void.
    pub result: PathStatus,

    /// Input variables passed to the analyzed function.
    pub inputs: Vec<Variable>,

    /// Variables explicitly marked as symbolic.
    pub symbolics: Vec<Variable>,
}

/// Status of the path.
///
/// If the path succeeded the return value (if any) is contained in that variant. Otherwise,
/// the reason for failure is contained in the `Failure` variant.
#[derive(Debug, Clone, PartialEq)]
pub enum PathStatus {
    /// The path finished successfully.
    Ok(Option<Variable>),

    /// The path failed.
    Failed(ErrorReason),
}

/// Detailed description of why a run failed.
///
/// Contains the error message, where the error happend and the stack trace from the point of failure.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorReason {
    /// Error message from the received error.
    pub error_message: String,

    /// For which line in the execution the error was encountered.
    pub error_location: Option<String>,

    /// The stack trace to where the error was encountered.
    ///
    /// The stack trace is in the order of the innermost call to the outermost.
    pub stack_trace: Vec<LineTrace>,
}

/// One line in the stack trace. Contains the name of the function and the line where it occurred.
#[derive(Debug, Clone, PartialEq)]
pub struct LineTrace {
    /// Name of the function.
    pub function_name: String,

    /// Source location if available.
    pub line: Option<String>,
}

/// A concrete solution to a symbol.
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    /// `name` is the source name of the variable, if it exists.
    pub name: Option<String>,

    /// Expression of the variable. This can be multiple values, and the solver should be invoked
    /// before presenting to the end-user. This allows to skip a (possible expensive) solve if not
    /// required.
    pub value: DExpr,

    /// Simple representation of the variable.
    pub ty: ExpressionType,
}

/// Type information for a an expression. This should be generic enough for all kinds of executor
/// to support.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    /// Integer value of a certain size in bits.
    Integer(u32),

    /// Floating point of a certain size in bits.
    Float(u32),

    /// Array or vector of a certain type with a specific number of values.
    Array(Box<ExpressionType>, u32),

    /// Structure
    Struct(Vec<ExpressionType>),

    /// Type is unknown.
    Unknown,
}

// /// A concrete value from an expression.
// ///
// /// Used to display concrete values to the end-user.
// #[derive(Debug, Clone, PartialEq)]
// pub struct ConcreteValue {
//     pub raw: String,

//     pub ty: ConcreteValueType,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum ConcreteValueType {
//     /// Integer value of size.
//     ///
//     /// Note that values are always treated as unsigned integers.
//     Value {
//         /// The integer value.
//         value: u128,
//         /// Size in bits of the integer value.
//         bits: u32,
//     },

//     /// Array or vector of values
//     Array(Vec<ConcreteValue>),

//     /// Structure with fields.
//     Struct(Vec<ConcreteValue>),

//     /// Value of unknown type.
//     Unknown(String),
// }

// impl fmt::Display for ConcreteValue {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         use ConcreteValueType::*;

//         match &self.ty {
//             Value { value, bits } => {
//                 let bits_str = if *bits == 1 { "bit" } else { "bits" };
//                 match *bits % 8 {
//                     0 => {
//                         let width = (*bits / 4) as usize + 2;
//                         write!(f, "{value:#0width$x} ({bits}-{bits_str})")
//                     }
//                     _ => {
//                         let width = *bits as usize + 2;
//                         write!(f, "{value:#0width$b} ({bits}-{bits_str})")
//                     }
//                 }
//             }
//             Array(elements) => {
//                 let elements = elements
//                     .iter()
//                     .map(|e| format!("{e}"))
//                     .reduce(|acc, s| format!("{acc}, {s}"));

//                 match elements {
//                     Some(elements) => write!(f, "[{elements}]"),
//                     None => write!(f, "[]"),
//                 }
//             }
//             Struct(fields) => match fields.len() {
//                 0 => {
//                     write!(f, "Struct {{}}")
//                 }
//                 _ => {
//                     writeln!(f, "Struct {{")?;
//                     for field in fields {
//                         writeln!(f, "{field}")?;
//                     }
//                     write!(f, "}}")
//                 }
//             },
//             Unknown(bits) => write!(f, "Unknown: {bits}"),
//         }
//     }
// }
