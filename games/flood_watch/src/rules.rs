//! Rule-application skeleton for Flood Watch.

use engine_core::Diagnostic;

pub fn not_implemented_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "flood_watch_not_implemented".to_owned(),
        message: "flood_watch rules land in later Gate 12 tickets".to_owned(),
    }
}
