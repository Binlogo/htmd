#[cfg(test)]
mod debug_regex {
    use regex::Regex;
    use std::sync::OnceLock;
    use std::borrow::Cow;

    const TAGNAME: &str = "[A-Za-z][A-Za-z0-9-]*";
    const ATTRIBUTENAME: &str = "[a-zA-Z_:][a-zA-Z0-9:._-]*";
    const UNQUOTEDVALUE: &str = r#"[^\"'=<>`\\x00-\\x20]+"#;
    const SINGLEQUOTEDVALUE: &str = "'[^']*'";
    const DOUBLEQUOTEDVALUE: &str = r#""[^"]*""#;
    const ATTRIBUTEVALUE: &str = "(?:";
    const ATTRIBUTEVALUE_MIDDLE: &str = "|";
    const ATTRIBUTEVALUE_END: &str = ")";
    const OPENTAG: &str = "<[A-Za-z][A-Za-z0-9-]*(?:\\s+[a-zA-Z_:][a-zA-Z0-9:._-]*(?:\\s*=\\s*(?:[^\\\"'=<>`\\\\x00-\\\\x20]+|'[^']*'|\"[^\"]*\"))?)*\\s*/?>";
    const CLOSETAG: &str = "</[A-Za-z][A-Za-z0-9-]*\\s*[>]";
    const HTMLCOMMENT: &str = "<!-->|<!--->|<!--(?:[^-]+|-[^-]|--[^>])*-->";
    const PROCESSINGINSTRUCTION: &str = "[<][?][\\s\\S]*?[?][>]";
    const DECLARATION: &str = "<![A-Z]+[^>]*>";
    const CDATA: &str = "<!\\[CDATA\\[[\\s\\S]*?\\]\\]>";
    const HTMLTAG: &str = "(?:<[A-Za-z][A-Za-z0-9-]*(?:\\s+[a-zA-Z_:][a-zA-Z0-9:._-]*(?:\\s*=\\s*(?:[^\\\"'=<>`\\\\x00-\\\\x20]+|'[^']*'|\"[^\"]*\"))?)*\\s*/?>|</[A-Za-z][A-Za-z0-9-]*\\s*[>]|<!-->|<!--->|<!--(?:[^-]+|-[^-]|--[^>])*-->|[<][?][\\s\\S]*?[?][>]|<![A-Z]+[^>]*>|<!\\[CDATA\\[[\\s\\S]*?\\]\\]>)";
    const HTML_BLOCK_1: &str = r#"(?i:^<(?:script|pre|textarea|style)(?:\s|>|$))"#;
    const HTML_BLOCK_6: &str = r#"(?i:^<[/]?(?:address|article|aside|base|basefont|blockquote|body|caption|center|col|colgroup|dd|details|dialog|dir|div|dl|dt|fieldset|figcaption|figure|footer|form|frame|frameset|h[123456]|head|header|hr|html|iframe|legend|li|link|main|menu|menuitem|nav|noframes|ol|optgroup|option|p|param|section|source|summary|table|tbody|td|tfoot|th|thead|title|tr|track|ul)(?:\s|[/]?[>]|$))"#;
    const COMBINED_HTML: &str = "(?:(?:<[A-Za-z][A-Za-z0-9-]*(?:\\s+[a-zA-Z_:][a-zA-Z0-9:._-]*(?:\\s*=\\s*(?:[^\\\"'=<>`\\\\x00-\\\\x20]+|'[^']*'|\"[^\"]*\"))?)*\\s*/?>|</[A-Za-z][A-Za-z0-9-]*\\s*[>]|<!-->|<!--->|<!--(?:[^-]+|-[^-]|--[^>])*-->|[<][?][\\s\\S]*?[?][>]|<![A-Z]+[^>]*>|<!\\[CDATA\\[[\\s\\S]*?\\]\\]>)|(?i:^<(?:script|pre|textarea|style)(?:\\s|>|$))|(?i:^<[/]?(?:address|article|aside|base|basefont|blockquote|body|caption|center|col|colgroup|dd|details|dialog|dir|div|dl|dt|fieldset|figcaption|figure|footer|form|frame|frameset|h[123456]|head|header|hr|html|iframe|legend|li|link|main|menu|menuitem|nav|noframes|ol|optgroup|option|p|param|section|source|summary|table|tbody|td|tfoot|th|thead|title|tr|track|ul)(?:\\s|[/]?[>]|$)))";
    
    static COMBINED_HTML_REGEX: OnceLock<Regex> = OnceLock::new();

    fn escape_html_original(text: Cow<'_, str>) -> Cow<'_, str> {
        let regex = COMBINED_HTML_REGEX.get_or_init(|| Regex::new(COMBINED_HTML).unwrap());
        let replaced_text = regex.replace_all(&text, "\\$0");
        if let Cow::Owned(o) = replaced_text {
            Cow::Owned(o)
        } else {
            text
        }
    }

    #[test]
    fn test_original_regex() {
        let input = "<![CDATA[character data]]>";
        let result = escape_html_original(input.into());
        println!("Original regex result: {}", result);
        
        let full_input = "Test <code>tags</code>, <!-- comments -->, <?processing instructions?>, <!A declaration>, and <![CDATA[character data]]>.";
        let full_result = escape_html_original(full_input.into());
        println!("Full original result: {}", full_result);
    }
}