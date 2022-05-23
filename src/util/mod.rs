use std::fmt;

/// A concrete value from an expression.
///
/// Used to display concrete values to the end-user.
#[derive(Debug, Clone, PartialEq)]
pub struct ConcreteValue {
    pub raw: String,

    pub ty: ConcreteValueType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConcreteValueType {
    /// Integer value of size.
    ///
    /// Note that values are always treated as unsigned integers.
    Value {
        /// The integer value.
        value: u128,
        /// Size in bits of the integer value.
        bits: u32,
    },

    /// Array or vector of values
    Array(Vec<ConcreteValue>),

    /// Structure with fields.
    Struct(Vec<ConcreteValue>),

    /// Value of unknown type.
    Unknown(String),
}

impl fmt::Display for ConcreteValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ConcreteValueType::*;

        match &self.ty {
            Value { value, bits } => {
                let bits_str = if *bits == 1 { "bit" } else { "bits" };
                match *bits % 8 {
                    0 => {
                        let width = (*bits / 4) as usize + 2;
                        write!(f, "{value:#0width$x} ({bits}-{bits_str})")
                    }
                    _ => {
                        let width = *bits as usize + 2;
                        write!(f, "{value:#0width$b} ({bits}-{bits_str})")
                    }
                }
            }
            Array(elements) => {
                let elements = elements
                    .iter()
                    .map(|e| format!("{e}"))
                    .reduce(|acc, s| format!("{acc}, {s}"));

                match elements {
                    Some(elements) => write!(f, "[{elements}]"),
                    None => write!(f, "[]"),
                }
            }
            Struct(fields) => match fields.len() {
                0 => {
                    write!(f, "Struct {{}}")
                }
                _ => {
                    writeln!(f, "Struct {{")?;
                    for field in fields {
                        writeln!(f, "{field}")?;
                    }
                    write!(f, "}}")
                }
            },
            Unknown(bits) => write!(f, "Unknown: {bits}"),
        }
    }
}
