//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyArrowFunctionParameters;
impl Format for JsAnyArrowFunctionParameters {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsParameters(node) => node.format(formatter),
            Self::JsAnyBinding(node) => node.format(formatter),
        }
    }
}