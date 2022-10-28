use yara_derive::Error;

use crate::parser::Span;

/// An warning raised while parsing YARA rules.
#[rustfmt::skip]
#[derive(Error, Debug)]
pub enum Warning {
    #[warning("consecutive jumps in hex pattern `{pattern_ident}`")]
    #[label("these consecutive jumps will be treated as {coalesced_jump}", jumps_span)]
    ConsecutiveJumps {
        detailed_report: String,
        pattern_ident: String,
        coalesced_jump: String,
        jumps_span: Span,
    },

    #[warning("potentially wrong expression")]
    #[label("this implies that multiple patterns must match", quantifier_span)]
    #[label("but all of them must match at the same offset", at_span)]
    PotentiallyWrongExpression {
        detailed_report: String,
        quantifier_span: Span,
        at_span: Span,
    }
}
