use crate::{FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsUnknownMember;
use rome_rowan::AstNode;

impl FormatNode for JsUnknownMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_unknown(self.syntax()))
    }
}