use colored::*;
use core::fmt::{self, Write};
use indenter::indented;

use crate::*;

impl fmt::Display for ConcreteValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ConcreteValue::*;

        match self {
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
                        writeln!(indented(f), "{field}")?;
                    }
                    write!(f, "}}")
                }
            },
        }
    }
}

impl fmt::Display for PathResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH {} ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            self.path
        )?;

        match &self.result {
            PathStatus::Ok(None) => {
                writeln!(f, "{}: returned void", "Success".green())?;
            }
            PathStatus::Ok(Some(value)) => {
                writeln!(f, "{}: returned {}", "Success".green(), value.value)?;
            }
            PathStatus::Failed(err) => {
                writeln!(f, "{}: {}", "Error".red(), err.error_message)?;
                if let Some(error_location) = &err.error_location {
                    writeln!(f, "    at {error_location}\n")?;
                }

                writeln!(f, "Stacktrace:")?;
                for (n, line) in err.stack_trace.iter().enumerate() {
                    writeln!(f, "{n:4}: {}", line.function_name)?;
                    if let Some(line) = &line.line {
                        writeln!(f, "      at {line}")?;
                    }
                }
            }
        }

        if !self.symbolics.is_empty() {
            writeln!(f, "\nSymbolic:")?;
            for value in self.symbolics.iter() {
                let name = if let Some(name) = value.name.as_ref() {
                    name
                } else {
                    "_"
                };
                if matches!(value.value, ConcreteValue::Struct { .. }) {
                    writeln!(f, "{name:>4}: ")?;
                    writeln!(indented(f), "{}", value.value)?;
                } else {
                    writeln!(f, "{name:>4}: {}", value.value)?;
                }
            }
        }

        if !self.inputs.is_empty() {
            writeln!(f, "\nInputs:")?;
            for (n, value) in self.inputs.iter().enumerate() {
                writeln!(f, "{n:4}: {}", value.value)?;
            }
        }
        Ok(())
    }
}

// impl fmt::Display for Value {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Value::Unsolved(_) => write!(f, "<Unsolved>"),
//             Value::Solved(value) => write!(f, "{value}"),
//         }
//     }
// }
