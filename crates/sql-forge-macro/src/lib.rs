// =============================================================================
// Imports
// =============================================================================

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Span, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    Expr, ExprBlock, ExprGroup, ExprLit, ExprParen, Ident, Lit, LitStr, Pat, Stmt, Token, Type,
};

// =============================================================================
// Input types
// =============================================================================

mod kw {
    syn::custom_keyword!(scalar);
}

/// A `:name = expr` parameter binding.
#[derive(Clone)]
struct ParamAssign {
    name: Ident,
    expr: Expr,
}

impl Parse for ParamAssign {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        input.parse::<Token![:]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let expr: Expr = input.parse()?;
        Ok(Self { name, expr })
    }
}

/// A single section value: SQL string + optional local parameter bindings.
#[derive(Clone)]
struct SectionFragment {
    sql: String,
    span: Span,
    params: ParamsSource,
}

/// One arm in a `match { ... }` inside a section assignment.
#[derive(Clone)]
struct SectionMatchArm {
    pat: Pat,
    guard: Option<Expr>,
    value: SectionValue,
}

/// The right-hand side of a section `#name = ...` assignment.
#[derive(Clone)]
enum SectionValue {
    /// A plain string (or `(string, params)` tuple).
    Single(SectionFragment),
    /// A tuple of fragments, one per section when using `#(a, b) = ...`.
    Grouped(Vec<SectionFragment>),
    /// A `match expr { arm => ..., arm => ... }` expression.
    Match {
        expr: Expr,
        arms: Vec<SectionMatchArm>,
    },
}

/// A full `#name = value` or `#(a, b) = value` section assignment.
#[derive(Clone)]
struct SectionAssign {
    names: Vec<Ident>,
    value: SectionValue,
}

impl Parse for SectionAssign {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        input.parse::<Token![#]>()?;

        // Parse `#(a, b)` for grouped sections, or `#name` for single.
        let names = if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let mut out = Vec::new();
            while !content.is_empty() {
                out.push(content.parse::<Ident>()?);
                if content.is_empty() {
                    break;
                }
                content.parse::<Token![,]>()?;
            }
            if out.is_empty() {
                return Err(input.error("sql_forge!: grouped section key list cannot be empty"));
            }
            out
        } else {
            vec![input.parse::<Ident>()?]
        };

        input.parse::<Token![=]>()?;
        let value = parse_section_value(input, names.len())?;
        Ok(Self { names, value })
    }
}

/// The fully-parsed macro invocation.
struct EnhancedQueryInput {
    db: Option<Type>,
    result: ResultSpec,
    force_scalar: bool,
    sql: SqlTemplate,
    params: ParamsSource,
    sections: Vec<SectionAssign>,
}

/// One entry in a result map `(>key = Model, ...)`.
#[derive(Clone)]
struct ResultAssign {
    name: Ident,
    model: Type,
    force_scalar: bool,
}

#[derive(Clone)]
enum ResultSpec {
    /// Execute-only (no model), e.g. `sql_forge!("SQL", ...)`
    None,
    /// e.g. `sql_forge!(User, ...)`
    Single(Type),
    /// e.g. `sql_forge!((>a = X, >b = Y), ...)`
    Group(Vec<ResultAssign>),
}

#[derive(Clone)]
enum ParamsSource {
    None,
    /// `( :name = expr, ... )`
    Map(Vec<ParamAssign>),
    /// A struct expression whose fields are matched to `:name` placeholders.
    Struct(Expr),
}

/// The SQL template. Only string literals are supported.
enum SqlTemplate {
    Literal(LitStr),
}

impl SqlTemplate {
    fn span(&self) -> Span {
        match self {
            Self::Literal(lit) => lit.span(),
        }
    }

    /// Parse the SQL into a sequence of textual segments and `{#section}` slots.
    fn into_segments(self) -> Result<Vec<Segment>, String> {
        match self {
            Self::Literal(lit) => parse_literal_segments(&lit.value()),
        }
    }
}

fn parse_sql_template(input: ParseStream<'_>) -> syn::Result<SqlTemplate> {
    if input.peek(LitStr) {
        Ok(SqlTemplate::Literal(input.parse::<LitStr>()?))
    } else {
        Err(input.error("sql_forge!: SQL template must be a string literal"))
    }
}

/// One piece of the parsed SQL template.
#[derive(Clone)]
enum Segment {
    /// Plain SQL text (may contain `:param` placeholders).
    Text(String),
    /// A `{#section_name}` placeholder.
    Section { name: String },
}

/// A fragment of SQL text after splitting on `:param`.
enum TextPart {
    /// Literal SQL text.
    Lit(String),
    /// A `:param` placeholder.
    Param { name: String, is_list: bool },
}

// =============================================================================
// Parsing helpers
// =============================================================================

/// Used by `detect_parenthesized_map_kind` to identify what a `(...)` argument is.
enum MapKind {
    Results,
    Params,
    Sections,
}

// =============================================================================
// Section value parsing (string fragments, match, grouped tuples)
// =============================================================================

fn detect_parenthesized_map_kind(input: ParseStream<'_>) -> syn::Result<Option<MapKind>> {
    let fork = input.fork();
    let content;
    syn::parenthesized!(content in fork);

    if content.is_empty() {
        return Err(input.error("sql_forge!: map argument cannot be empty"));
    }

    if content.peek(Token![>]) {
        Ok(Some(MapKind::Results))
    } else if content.peek(Token![:]) {
        Ok(Some(MapKind::Params))
    } else if content.peek(Token![#]) {
        Ok(Some(MapKind::Sections))
    } else {
        Ok(None)
    }
}

impl Parse for ResultAssign {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        input.parse::<Token![>]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let (force_scalar, model) = if input.peek(kw::scalar) {
            input.parse::<kw::scalar>()?;
            (true, input.parse::<Type>()?)
        } else {
            (false, input.parse::<Type>()?)
        };
        Ok(Self {
            name,
            model,
            force_scalar,
        })
    }
}

fn parse_result_map(input: ParseStream<'_>) -> syn::Result<Vec<ResultAssign>> {
    let content;
    syn::parenthesized!(content in input);

    let mut results = Vec::new();
    while !content.is_empty() {
        results.push(content.parse::<ResultAssign>()?);
        if content.is_empty() {
            break;
        }
        content.parse::<Token![,]>()?;
    }

    if results.is_empty() {
        return Err(input.error("sql_forge!: result map cannot be empty"));
    }

    Ok(results)
}

fn parse_param_map(input: ParseStream<'_>) -> syn::Result<Vec<ParamAssign>> {
    let content;
    syn::parenthesized!(content in input);

    let mut params = Vec::new();
    while !content.is_empty() {
        params.push(content.parse::<ParamAssign>()?);
        if content.is_empty() {
            break;
        }
        content.parse::<Token![,]>()?;
    }

    Ok(params)
}

fn parse_section_map(input: ParseStream<'_>) -> syn::Result<Vec<SectionAssign>> {
    let content;
    syn::parenthesized!(content in input);

    let mut sections = Vec::new();
    while !content.is_empty() {
        sections.push(content.parse::<SectionAssign>()?);
        if content.is_empty() {
            break;
        }
        content.parse::<Token![,]>()?;
    }

    Ok(sections)
}

fn parse_params_source_expr(
    input: ParseStream<'_>,
    allow_sections: bool,
) -> syn::Result<ParamsSource> {
    if input.peek(syn::token::Paren) {
        match detect_parenthesized_map_kind(input)? {
            Some(MapKind::Results) => Err(input
                .error("sql_forge!: result maps are only allowed as the macro result argument")),
            Some(MapKind::Params) => Ok(ParamsSource::Map(parse_param_map(input)?)),
            Some(MapKind::Sections) if allow_sections => {
                Err(input.error("sql_forge!: section maps are not allowed here"))
            }
            Some(MapKind::Sections) => Err(input.error(
                "sql_forge!: use :name = expr for section-local parameters, not #name = expr",
            )),
            None => Ok(ParamsSource::Struct(input.parse::<Expr>()?)),
        }
    } else {
        Ok(ParamsSource::Struct(input.parse::<Expr>()?))
    }
}

fn parse_section_fragment(input: ParseStream<'_>) -> syn::Result<SectionFragment> {
    if input.peek(syn::token::Paren) {
        let fork = input.fork();
        let content;
        syn::parenthesized!(content in fork);

        if let Ok(first_expr) = content.parse::<Expr>() {
            if extract_lit_str(&first_expr).is_some() && content.parse::<Token![,]>().is_ok() {
                let _ = parse_params_source_expr(&content, false)?;
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
                if content.is_empty() {
                    let content;
                    syn::parenthesized!(content in input);
                    let first_expr: Expr = content.parse()?;
                    let sql = extract_lit_str(&first_expr).ok_or_else(|| {
                        input.error("sql_forge!: section tuple must start with a string literal")
                    })?;
                    let span = first_expr.span();
                    content.parse::<Token![,]>()?;
                    let params = parse_params_source_expr(&content, false)?;
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                    if !content.is_empty() {
                        return Err(content.error(
                            "sql_forge!: unexpected tokens after section-local parameter source",
                        ));
                    }
                    return Ok(SectionFragment { sql, span, params });
                }
            }
        }
    }

    let expr: Expr = input.parse()?;
    let sql = extract_lit_str(&expr).ok_or_else(|| {
        input
            .error("sql_forge!: section values must be string literals or (string literal, params)")
    })?;
    Ok(SectionFragment {
        sql,
        span: expr.span(),
        params: ParamsSource::None,
    })
}

fn parse_section_value(input: ParseStream<'_>, width: usize) -> syn::Result<SectionValue> {
    if input.peek(Token![match]) {
        input.parse::<Token![match]>()?;
        let expr: Expr = input.call(Expr::parse_without_eager_brace)?;
        let content;
        syn::braced!(content in input);
        let mut arms = Vec::new();
        while !content.is_empty() {
            let pat = content.call(Pat::parse_multi_with_leading_vert)?;
            let guard = if content.peek(Token![if]) {
                content.parse::<Token![if]>()?;
                Some(content.parse::<Expr>()?)
            } else {
                None
            };
            content.parse::<Token![=>]>()?;
            let value = parse_section_value(&content, width)?;
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
            arms.push(SectionMatchArm { pat, guard, value });
        }
        return Ok(SectionValue::Match { expr, arms });
    }

    if width == 1 {
        return Ok(SectionValue::Single(parse_section_fragment(input)?));
    }

    let content;
    syn::parenthesized!(content in input);
    let mut items = Vec::new();
    while !content.is_empty() {
        items.push(parse_section_fragment(&content)?);
        if content.is_empty() {
            break;
        }
        content.parse::<Token![,]>()?;
    }

    if items.len() != width {
        return Err(input.error(format!(
            "sql_forge!: grouped section value must provide exactly {} items",
            width,
        )));
    }

    Ok(SectionValue::Grouped(items))
}

// =============================================================================
// Top-level macro input parsing (EnhancedQueryInput::parse)
// =============================================================================

impl Parse for EnhancedQueryInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let (db, result, force_scalar, sql) = if input.peek(LitStr) {
            let sql = parse_sql_template(input)?;
            (None, ResultSpec::None, false, sql)
        } else if input.peek(kw::scalar) {
            input.parse::<kw::scalar>()?;
            let model: Type = input.parse()?;
            input.parse::<Token![,]>()?;
            let sql = parse_sql_template(input)?;
            (None, ResultSpec::Single(model), true, sql)
        } else if input.peek(syn::token::Paren) {
            let result_map_kind = detect_parenthesized_map_kind(input)?;
            match result_map_kind {
                Some(MapKind::Results) => {
                    let result = ResultSpec::Group(parse_result_map(input)?);
                    input.parse::<Token![,]>()?;
                    let sql = parse_sql_template(input)?;
                    (None, result, false, sql)
                }
                _ => {
                    return Err(input.error(
                        "sql_forge!: expected a result map like (>name = Model, ...) or a model type",
                    ));
                }
            }
        } else {
            let first_ty: Type = input.parse()?;
            input.parse::<Token![,]>()?;

            if input.peek(LitStr) {
                let model = first_ty;
                let sql = parse_sql_template(input)?;
                (None, ResultSpec::Single(model), false, sql)
            } else if input.peek(kw::scalar) {
                input.parse::<kw::scalar>()?;
                let model: Type = input.parse()?;
                input.parse::<Token![,]>()?;
                let sql = parse_sql_template(input)?;
                (Some(first_ty), ResultSpec::Single(model), true, sql)
            } else if input.peek(syn::token::Paren)
                && matches!(
                    detect_parenthesized_map_kind(input)?,
                    Some(MapKind::Results)
                )
            {
                let result = ResultSpec::Group(parse_result_map(input)?);
                input.parse::<Token![,]>()?;
                let sql = parse_sql_template(input)?;
                (Some(first_ty), result, false, sql)
            } else {
                let db = Some(first_ty);
                let model: Type = input.parse()?;
                input.parse::<Token![,]>()?;
                let sql = parse_sql_template(input)?;
                (db, ResultSpec::Single(model), false, sql)
            }
        };

        let mut params = ParamsSource::None;
        let mut sections = Vec::new();
        let mut seen_params = false;
        let mut seen_sections = false;

        if input.parse::<Token![,]>().is_ok() {
            while !input.is_empty() {
                if input.peek(syn::token::Paren) {
                    match detect_parenthesized_map_kind(input)? {
                        Some(MapKind::Results) => {
                            return Err(input.error(
                                "sql_forge!: result maps are only allowed as the macro result argument",
                            ));
                        }
                        Some(MapKind::Params) => {
                            if seen_params {
                                return Err(
                                    input.error("sql_forge!: only one parameter source is allowed")
                                );
                            }
                            params = ParamsSource::Map(parse_param_map(input)?);
                            seen_params = true;
                        }
                        Some(MapKind::Sections) => {
                            if seen_sections {
                                return Err(
                                    input.error("sql_forge!: duplicate section map argument")
                                );
                            }
                            sections = parse_section_map(input)?;
                            seen_sections = true;
                        }
                        None => {
                            if seen_params {
                                return Err(
                                    input.error("sql_forge!: only one parameter source is allowed")
                                );
                            }
                            params = ParamsSource::Struct(input.parse::<Expr>()?);
                            seen_params = true;
                        }
                    }
                } else {
                    if seen_params {
                        return Err(input.error("sql_forge!: only one parameter source is allowed"));
                    }
                    params = ParamsSource::Struct(input.parse::<Expr>()?);
                    seen_params = true;
                }

                if input.parse::<Token![,]>().is_ok() {
                    continue;
                }
                break;
            }
        }

        if !input.is_empty() {
            return Err(input.error("sql_forge!: unexpected tokens in macro invocation"));
        }

        Ok(Self {
            db,
            result,
            force_scalar,
            sql,
            params,
            sections,
        })
    }
}

// =============================================================================
// Database type resolution
// =============================================================================

fn resolve_db_from_cargo_toml() -> Result<Type, String> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|_| {
        "sql_forge!: could not read CARGO_MANIFEST_DIR; pass DB as first macro argument"
    })?;
    let manifest_path = Path::new(&manifest_dir).join("Cargo.toml");

    let cargo_toml = fs::read_to_string(&manifest_path).map_err(|err| {
        format!(
            "sql_forge!: failed to read {}: {}",
            manifest_path.display(),
            err
        )
    })?;

    let value: toml::Value = toml::from_str(&cargo_toml)
        .map_err(|err| format!("sql_forge!: failed to parse Cargo.toml: {}", err))?;

    let db_str = value
        .get("package")
        .and_then(|v| v.get("metadata"))
        .and_then(|v| v.get("sql_forge"))
        .and_then(|v| v.get("db"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            "sql_forge!: missing [package.metadata.sql_forge] db = \"...\" in Cargo.toml; pass DB as first macro argument or configure metadata"
        })?;

    syn::parse_str::<Type>(db_str).map_err(|err| {
        format!(
            "sql_forge!: invalid DB type `{}` in Cargo.toml metadata: {}",
            db_str, err
        )
    })
}

fn is_builtin_scalar_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    if type_path.qself.is_some()
        || type_path.path.leading_colon.is_some()
        || type_path.path.segments.len() != 1
    {
        return false;
    }

    let ident = &type_path.path.segments[0].ident;
    ident == "i8"
        || ident == "i16"
        || ident == "i32"
        || ident == "i64"
        || ident == "isize"
        || ident == "u8"
        || ident == "u16"
        || ident == "u32"
        || ident == "u64"
        || ident == "usize"
        || ident == "f32"
        || ident == "f64"
        || ident == "bool"
        || ident == "String"
}

fn scalar_output_type(model: &Type) -> Option<&Type> {
    if is_builtin_scalar_type(model) {
        return Some(model);
    }
    None
}

fn push_text_segment(out: &mut Vec<Segment>, text: String) {
    if text.is_empty() {
        return;
    }
    match out.last_mut() {
        Some(Segment::Text(existing)) => existing.push_str(&text),
        _ => out.push(Segment::Text(text)),
    }
}

fn parse_literal_segments(sql: &str) -> Result<Vec<Segment>, String> {
    let mut out = Vec::new();
    let mut text = String::new();
    let mut chars = sql.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '{' {
            text.push(ch);
            continue;
        }

        if chars.peek() != Some(&'#') {
            text.push(ch);
            continue;
        }

        chars.next();
        push_text_segment(&mut out, std::mem::take(&mut text));

        let mut name = String::new();
        loop {
            let Some(next) = chars.next() else {
                return Err("sql_forge!: section placeholder without closing }".to_string());
            };
            if next == '}' {
                break;
            }
            name.push(next);
        }

        if name.is_empty() {
            return Err("sql_forge!: empty section placeholder name".to_string());
        }

        out.push(Segment::Section { name });
    }

    push_text_segment(&mut out, text);
    Ok(out)
}

// =============================================================================
// SQL template parsing: {#sections} and :param placeholders
// =============================================================================

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    is_ident_start(ch) || ch.is_ascii_digit()
}

fn sanitize_backticked_alias_ident(content: &str) -> String {
    let mut split_at = content.len();
    for (idx, ch) in content.char_indices() {
        if ch == '!' || ch == '?' || ch == ':' {
            split_at = idx;
            break;
        }
    }

    if split_at == content.len() {
        return content.to_string();
    }

    let base = content[..split_at].trim_end();
    if base.is_empty() {
        content.to_string()
    } else {
        base.to_string()
    }
}

fn sanitize_runtime_sql_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '`' {
            out.push(ch);
            continue;
        }

        let mut content = String::new();
        let mut closed = false;

        while let Some(next) = chars.next() {
            if next == '`' {
                closed = true;
                break;
            }
            content.push(next);
        }

        if closed {
            out.push('`');
            out.push_str(&sanitize_backticked_alias_ident(&content));
            out.push('`');
        } else {
            out.push('`');
            out.push_str(&content);
            break;
        }
    }

    out
}

fn parse_text_parts(text: &str) -> Vec<TextPart> {
    let mut parts = Vec::new();
    let mut last = 0usize;
    let mut iter = text.char_indices().peekable();

    while let Some((idx, ch)) = iter.next() {
        if ch != ':' {
            continue;
        }

        let Some(&(next_idx, next_ch)) = iter.peek() else {
            continue;
        };

        if !is_ident_start(next_ch) {
            continue;
        }

        if text[..idx].chars().next_back() == Some(':') {
            continue;
        }

        if last < idx {
            parts.push(TextPart::Lit(text[last..idx].to_string()));
        }

        iter.next();

        let mut name = String::new();
        name.push(next_ch);
        let mut end = next_idx + next_ch.len_utf8();

        while let Some(&(j, c)) = iter.peek() {
            if is_ident_continue(c) {
                name.push(c);
                end = j + c.len_utf8();
                iter.next();
            } else {
                break;
            }
        }

        let mut is_list = false;
        if text[end..].starts_with("[]") {
            is_list = true;
            end += 2;
        }

        parts.push(TextPart::Param { name, is_list });
        last = end;
    }

    if last < text.len() {
        parts.push(TextPart::Lit(text[last..].to_string()));
    }

    parts
}

fn render_validator_text(text: &str) -> (String, Vec<(String, bool)>) {
    let mut out_sql = String::new();
    let mut occurrences = Vec::new();

    for part in parse_text_parts(text) {
        match part {
            TextPart::Lit(lit) => out_sql.push_str(&lit),
            TextPart::Param { name, is_list } => {
                out_sql.push('?');
                occurrences.push((name, is_list));
            }
        }
    }

    (out_sql, occurrences)
}

fn strip_expr(expr: &Expr) -> &Expr {
    match expr {
        Expr::Paren(ExprParen { expr, .. }) => strip_expr(expr),
        Expr::Group(ExprGroup { expr, .. }) => strip_expr(expr),
        Expr::Block(ExprBlock { block, .. }) => {
            if block.stmts.len() != 1 {
                return expr;
            }
            match &block.stmts[0] {
                Stmt::Expr(inner, None) => strip_expr(inner),
                _ => expr,
            }
        }
        _ => expr,
    }
}

fn extract_lit_str(expr: &Expr) -> Option<String> {
    match strip_expr(expr) {
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit), ..
        }) => Some(lit.value()),
        _ => None,
    }
}

// =============================================================================
// Preprocessing: {>key} compile-time result flags
// =============================================================================

fn result_flag_ident(name: &str) -> syn::Ident {
    format_ident!("__enhanced_result_flag_{}", name)
}

/// Replaces `{>key}` tokens inside braced groups with `__enhanced_result_flag_key`
/// identifiers. This is a preprocessing step so that the rest of the parser sees
/// plain identifiers instead of braced groups it does not understand.
fn preprocess_result_key_placeholders(input: TokenStream2) -> TokenStream2 {
    fn walk(stream: TokenStream2) -> TokenStream2 {
        let mut out = TokenStream2::new();
        let mut iter = stream.into_iter().peekable();

        while let Some(token) = iter.next() {
            match token {
                TokenTree::Group(group) => {
                    if group.delimiter() == Delimiter::Brace {
                        let mut inner = group.stream().into_iter();
                        let first = inner.next();
                        let second = inner.next();
                        let third = inner.next();

                        if let (
                            Some(TokenTree::Punct(p)),
                            Some(TokenTree::Ident(name_ident)),
                            None,
                        ) = (first, second, third)
                        {
                            if p.as_char() == '>' {
                                let ident = result_flag_ident(&name_ident.to_string());
                                out.extend(std::iter::once(TokenTree::Ident(ident)));
                                continue;
                            }
                        }
                    }

                    let new_inner = walk(group.stream());
                    let mut new_group = Group::new(group.delimiter(), new_inner);
                    new_group.set_span(group.span());
                    out.extend(std::iter::once(TokenTree::Group(new_group)));
                }
                other => out.extend(std::iter::once(other)),
            }
        }

        out
    }

    walk(input)
}

fn build_result_flag_bindings(keys: &[String], active_key: Option<&str>) -> Vec<TokenStream2> {
    keys.iter()
        .map(|key| {
            let ident = result_flag_ident(key);
            let enabled = Some(key.as_str()) == active_key;
            quote! { let #ident: bool = #enabled; }
        })
        .collect()
}

// Returns Vec<Vec<SectionFragment>> indexed [section_idx][case_idx].
// =============================================================================
// Section variant collection
// =============================================================================

/// Collects all possible `SectionFragment` values per section index. Each
/// returned `Vec<Vec<SectionFragment>>` is indexed `[section_idx][case_idx]`,
/// listing every variant that section can take across all match arms.
/// Used for full validation (cycling strategy over all variants).
fn collect_section_variants(
    value: SectionValue,
    width: usize,
) -> Result<Vec<Vec<SectionFragment>>, String> {
    match value {
        SectionValue::Single(fragment) => {
            if width != 1 {
                return Err(
                    "sql_forge!: grouped sections must return one item per section".to_string(),
                );
            }
            Ok(vec![vec![fragment]])
        }
        SectionValue::Grouped(fragments) => {
            if fragments.len() != width {
                return Err(
                    "sql_forge!: grouped sections must return one item per section".to_string(),
                );
            }
            Ok(fragments.into_iter().map(|f| vec![f]).collect())
        }
        SectionValue::Match { arms, .. } => {
            let mut per_section: Vec<Vec<SectionFragment>> =
                (0..width).map(|_| Vec::new()).collect();
            for arm in arms {
                let arm_sections = collect_section_variants(arm.value, width)?;
                for (sec_idx, cases) in arm_sections.into_iter().enumerate() {
                    per_section[sec_idx].extend(cases);
                }
            }
            if per_section.is_empty() || per_section[0].is_empty() {
                return Err("sql_forge!: section match must have at least one arm".to_string());
            }
            Ok(per_section)
        }
    }
}

fn expr_result_flag_key(expr: &Expr) -> Option<String> {
    match strip_expr(expr) {
        Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => {
            let name = path.path.segments[0].ident.to_string();
            name.strip_prefix("__enhanced_result_flag_")
                .map(|v| v.to_string())
        }
        _ => None,
    }
}

fn pattern_matches_bool(pat: &Pat, value: bool) -> Option<bool> {
    match pat {
        Pat::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Bool(lit_bool) => Some(lit_bool.value == value),
            _ => None,
        },
        Pat::Wild(_) => Some(true),
        _ => None,
    }
}

/// Like `collect_section_variants`, but filters `match` arms by the active
/// result key when the match expression is a `{>key}` flag. When building the
/// query for a specific key, only the matching arm (true/false) is included;
/// arms with guards or non-flag expressions include all variants as usual.
fn collect_section_variants_for_result(
    value: SectionValue,
    width: usize,
    active_key: Option<&str>,
) -> Result<Vec<Vec<SectionFragment>>, String> {
    match value {
        SectionValue::Single(fragment) => {
            if width != 1 {
                return Err(
                    "sql_forge!: grouped sections must return one item per section".to_string(),
                );
            }
            Ok(vec![vec![fragment]])
        }
        SectionValue::Grouped(fragments) => {
            if fragments.len() != width {
                return Err(
                    "sql_forge!: grouped sections must return one item per section".to_string(),
                );
            }
            Ok(fragments.into_iter().map(|f| vec![f]).collect())
        }
        SectionValue::Match { expr, arms } => {
            let mut per_section: Vec<Vec<SectionFragment>> =
                (0..width).map(|_| Vec::new()).collect();

            let mut selected_arms: Vec<SectionMatchArm> = Vec::new();
            if let Some(key) = expr_result_flag_key(&expr) {
                let target = active_key == Some(key.as_str());
                for arm in arms {
                    if arm.guard.is_some() {
                        selected_arms.push(arm);
                        continue;
                    }
                    match pattern_matches_bool(&arm.pat, target) {
                        Some(true) => selected_arms.push(arm),
                        Some(false) => {}
                        None => selected_arms.push(arm),
                    }
                }
            } else {
                selected_arms = arms;
            }

            for arm in selected_arms {
                let arm_sections =
                    collect_section_variants_for_result(arm.value, width, active_key)?;
                for (sec_idx, cases) in arm_sections.into_iter().enumerate() {
                    per_section[sec_idx].extend(cases);
                }
            }

            if per_section.is_empty() || per_section[0].is_empty() {
                return Err("sql_forge!: section match must have at least one arm".to_string());
            }
            Ok(per_section)
        }
    }
}

// =============================================================================
// Parameter binding generation
// =============================================================================

/// Generates `let` bindings for all parameters used in the SQL or section
/// fragments. Returns a map of param-name → local ident for later reference,
/// and a list of `let` statements.
fn build_param_bindings(
    params: &ParamsSource,
    used_param_names: &[String],
    prefix: &str,
    for_validator: bool,
    enforce_usage_check: bool,
) -> Result<(HashMap<String, syn::Ident>, Vec<TokenStream2>), TokenStream> {
    let mut declared_params = HashMap::<String, syn::Ident>::new();
    let mut bindings = Vec::<TokenStream2>::new();

    match params {
        ParamsSource::None => {}
        ParamsSource::Map(entries) => {
            for entry in entries {
                let key = entry.name.to_string();
                if declared_params.contains_key(&key) {
                    return Err(syn::Error::new(
                        entry.name.span(),
                        "sql_forge!: duplicated parameter mapping",
                    )
                    .to_compile_error()
                    .into());
                }
                if enforce_usage_check && !used_param_names.iter().any(|n| n == &key) {
                    return Err(syn::Error::new(
                        entry.name.span(),
                        format!(
                            "sql_forge!: parameter :{} is unused in the SQL template",
                            key,
                        ),
                    )
                    .to_compile_error()
                    .into());
                }
                let local_ident = format_ident!("__enhanced_{}_{}", prefix, key);
                let expr = &entry.expr;
                if for_validator {
                    bindings.push(quote! {
                        let #local_ident = &(#expr);
                    });
                } else {
                    bindings.push(quote! {
                        let #local_ident = #expr;
                    });
                }
                declared_params.insert(key, local_ident);
            }
        }
        ParamsSource::Struct(expr) => {
            let source_ident = format_ident!("__enhanced_source_{}", prefix);
            bindings.push(quote! {
                let #source_ident = &(#expr);
            });
            for name in used_param_names {
                let local_ident = format_ident!("__enhanced_{}_{}", prefix, name);
                let field_ident = format_ident!("{}", name);
                bindings.push(quote! {
                    let #local_ident = #source_ident.#field_ident;
                });
                declared_params.insert(name.to_string(), local_ident);
            }
        }
    }

    Ok((declared_params, bindings))
}

/// Builds the `?`-style SQL string and argument list for the compile-time
/// validator (sqlx::query_as! / query_scalar!). Each `:param` in the SQL is
/// replaced by `?`, and the corresponding value expression is collected into
/// the args list.
fn render_validator_args(
    sql: &str,
    local_params: &HashMap<String, syn::Ident>,
    top_level_params: &HashMap<String, syn::Ident>,
    allow_top_level_fallback: bool,
    sql_span: Span,
) -> Result<(String, Vec<TokenStream2>), TokenStream> {
    let (rendered_sql, occurrences) = render_validator_text(sql);
    let mut args = Vec::<TokenStream2>::new();

    for (name, is_list) in occurrences {
        let local_ident = if allow_top_level_fallback {
            local_params
                .get(&name)
                .or_else(|| top_level_params.get(&name))
        } else {
            local_params.get(&name)
        };

        let Some(local_ident) = local_ident else {
            return Err(syn::Error::new(
                sql_span,
                format!("sql_forge!: parameter :{} has no mapping", name),
            )
            .to_compile_error()
            .into());
        };

        if is_list {
            args.push(quote! { (#local_ident).as_slice()[0] });
        } else {
            args.push(quote! { #local_ident });
        }
    }

    Ok((rendered_sql, args))
}

// =============================================================================
// Runtime code generation (QueryBuilder-based)
// =============================================================================

/// Generates the `push()` / `push_bind()` calls for a single section fragment
/// at runtime using `sqlx::QueryBuilder`.
fn render_runtime_fragment(
    fragment: &SectionFragment,
    local_params: &HashMap<String, syn::Ident>,
) -> Result<TokenStream2, TokenStream> {
    let mut steps = Vec::<TokenStream2>::new();

    for part in parse_text_parts(&fragment.sql) {
        match part {
            TextPart::Lit(lit) => {
                let lit_str = LitStr::new(&lit, fragment.span);
                steps.push(quote! { __builder.push(#lit_str); });
            }
            TextPart::Param { name, is_list } => {
                let local_ident = local_params
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| format_ident!("__enhanced_runtime_{}", name));

                if is_list {
                    steps.push(quote! {
                        let __enhanced_values = #local_ident;
                        let mut __separated = __builder.separated(", ");
                        for __value in __enhanced_values {
                            __separated.push_bind(__value);
                        }
                    });
                } else {
                    steps.push(quote! {
                        __builder.push_bind(#local_ident);
                    });
                }
            }
        }
    }

    Ok(quote! { #( #steps )* })
}

fn build_section_runtime_action(
    value: &SectionValue,
    section_idx: usize,
    prefix: &str,
) -> Result<TokenStream2, TokenStream> {
    match value {
        SectionValue::Single(fragment) => {
            let used_param_names = collect_used_param_names_in_sql(&fragment.sql);
            let (local_params, bindings) =
                build_param_bindings(&fragment.params, &used_param_names, prefix, false, true)?;
            let body = render_runtime_fragment(fragment, &local_params)?;
            Ok(quote! {{ #( #bindings )* #body }})
        }
        SectionValue::Grouped(fragments) => {
            let fragment = &fragments[section_idx];
            let used_param_names = collect_used_param_names_in_sql(&fragment.sql);
            let (local_params, bindings) =
                build_param_bindings(&fragment.params, &used_param_names, prefix, false, true)?;
            let body = render_runtime_fragment(fragment, &local_params)?;
            Ok(quote! {{ #( #bindings )* #body }})
        }
        SectionValue::Match { expr, arms } => {
            let arm_tokens: Result<Vec<TokenStream2>, TokenStream> = arms
                .iter()
                .enumerate()
                .map(|(arm_idx, arm)| {
                    let pat = &arm.pat;
                    let guard_tokens = arm.guard.as_ref().map(|guard| quote! { if #guard });
                    let body = build_section_runtime_action(
                        &arm.value,
                        section_idx,
                        &format!("{}_{}", prefix, arm_idx),
                    )?;
                    Ok::<TokenStream2, TokenStream>(quote! { #pat #guard_tokens => #body })
                })
                .collect();
            let arm_tokens = arm_tokens?;
            Ok(quote! {
                match #expr {
                    #( #arm_tokens ),*
                }
            })
        }
    }
}

fn collect_used_param_names(segments: &[Segment]) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = HashSet::<String>::new();

    for segment in segments {
        if let Segment::Text(text) = segment {
            for name in collect_used_param_names_in_sql(text) {
                if seen.insert(name.clone()) {
                    names.push(name);
                }
            }
        }
    }

    names
}

fn collect_used_param_names_in_sql(sql: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = HashSet::<String>::new();
    for part in parse_text_parts(sql) {
        if let TextPart::Param { name, .. } = part {
            if seen.insert(name.to_string()) {
                names.push(name);
            }
        }
    }
    names
}

/// Builds a parameterized SQL query with compile-time type-checking and a
/// runtime [`sqlx::QueryBuilder`] for dynamic SQL.
///
/// Combines `sqlx::query_as!` / `sqlx::query_scalar!` validation (never called
/// at runtime) with `QueryBuilder::push_bind` for safe value binding.
///
/// # Syntax
///
/// ```text
/// sql_forge!(
///     [DB,]        // optional: sqlx::MySql | sqlx::Postgres | sqlx::Sqlite
///     Model,       // return type (struct or scalar primitive)
///     SQL,         // string literal  or  { braced token template }
///     [params,]    // optional: ( :name = expr, ... )  or  struct_expr
///     [(sections)] // optional: ( #name = ..., ... )
/// )
///
/// ### Multiple results
///
/// ```ignore
/// sql_forge!(
///     [DB,]
///     ( >key1 = ModelA, >key2 = scalar i64 ),  // result map
///     SQL,
///     [params,]
///     [(sections)]                             // sections may use {>key} match
/// )
/// ```
///
/// The DB type may be omitted when `[package.metadata.sql_forge] db = "..."` is
/// set in `Cargo.toml`.
///
/// # Parameters
///
/// Named parameters are written `:name` in the SQL. At runtime each occurrence
/// is replaced by a `push_bind` call; at compile time it becomes `?` for
/// `query_as!` / `query_scalar!`.
///
/// **Inline map** – bind individual expressions:
/// ```rust,ignore
/// sql_forge!(User, "SELECT ... WHERE id <= :max_id", ( :max_id = filter.max_id ))
/// ```
///
/// **Struct source** – field names are matched to `:name` placeholders automatically:
/// ```rust,ignore
/// sql_forge!(User, "SELECT ... WHERE id <= :max_id LIMIT :limit", filter)
/// ```
///
/// # Sections (`{#name}`)
///
/// Sections are runtime SQL slots; each section's variants are validated at
/// compile time via `query_as!` / `query_scalar!`, though not every combination
/// of variants across sections is checked. The section map is a second parenthesised
/// argument starting with `#`:
///
/// ```rust,ignore
/// sql_forge!(
///     User,
///     "SELECT * FROM users {#join_org}",
///     (
///         #join_org = match include_org {
///             true  => " JOIN organisations o ON o.id = users.org_id ",
///             false => "",
///         }
///     )
/// )
/// ```
///
/// A section arm can also carry local parameters as a tuple `("sql", params)`:
///
/// ```rust,ignore
/// sql_forge!(
///     User,
///     "SELECT * FROM users {#filter}",
///     (
///         #filter = (
///             " WHERE id <= :max_id AND status = :status ",
///             ( :max_id = max_id, :status = "active" ),
///         )
///     )
/// )
/// ```
///
/// Multiple placeholders driven by one `match` use `#(a, b)` with each arm
/// returning a tuple of the same width:
///
/// ```rust,ignore
/// sql_forge!(
///     User,
///     "SELECT * FROM users {#join_org} {#filter_org}",
///     (
///         #(join_org, filter_org) = match include_org {
///             true  => (
///                 " JOIN organisations o ON o.id = users.org_id ",
///                 (
///                     " AND o.active = :active ",
///                     ( :active = true ),
///                 ),
///             ),
///             false => ("", ""),
///         }
///     )
/// )
/// ```
///
/// # `IN (...)` with list parameters
///
/// Wrap the placeholder in parentheses to expand a `Vec` into multiple bound
/// values:
///
/// ```rust,ignore
/// sql_forge!(User, "SELECT * FROM users WHERE id IN (:ids[])", ( :ids = ids ))
/// ```
///
/// **Empty lists** are not rewritten; `IN ()` is a database syntax error.
/// Guard against empty inputs explicitly, e.g. with a dynamic section:
///
/// ```rust,ignore
/// sql_forge!(
///     User,
///     "SELECT id, name FROM users WHERE {#filter}",
///     (
///         #filter = match ids.is_empty() {
///             true  => "1 = 0",
///             false => ("id IN (:ids[])", ( :ids = ids )),
///         }
///     )
/// )
/// ```
///
/// # Scalar output
///
/// When `Model` is a primitive (`i32`, `i64`, `String`, etc.) the macro uses
/// `query_scalar!` for validation and `build_query_scalar` for execution.
///
/// # Multiple results
///
/// A result map produces an `EnhancedQueryGroup` with one query per key.
/// Each key can be a struct or a primitive (used as a scalar):
///
/// ```rust,ignore
/// sql_forge!(
///     (
///         >count = i64,
///         >items = Item,
///     ),
///     "SELECT {#fields} FROM items WHERE category_id = :cat",
///     ( :cat = category_id ),
///     (
///         #fields = match {>count} {           // {>key} is true when building
///             true  => "COUNT(*) AS total",    //   the query for that key
///             false => "id, name, price",       //   and false otherwise
///         }
///     )
/// )
/// ```
///
/// The generated struct has one field per key (`group.count`, `group.items`),
/// each implementing `EnhancedQuery<T, Db = DB>` and usable with any SQLx
/// executor method (`fetch_one`, `fetch_all`, etc.).
///
/// # Caveats
///
/// **String literals containing `:`**
///
/// In this case, the template scanner cannot distinguish
/// a colon inside a SQL string literal from a `:name` placeholder. Embedding
/// `"abc:def"` directly in the template will fail.
/// Pass such values as bind parameters instead:
///
/// ```rust,ignore
/// // ❌  sql_forge!(User, r#"WHERE name = "abc:def""#);
/// // ✅
/// sql_forge!(User, r#"WHERE name = :name"#, ( :name = "abc:def" ))
/// ```
///
/// **String literals containing `{#`**
///
/// Similarly, a `{#` sequence inside a
/// SQL string literal is treated as a section slot. Pass the value as a bind
/// parameter:
///
/// ```rust,ignore
/// // ❌  sql_forge!(User, r#"WHERE name = "abc{#def""#);
/// // ✅
/// sql_forge!(User, r#"WHERE name = :name"#, ( :name = "abc{#def" ))
/// ```
#[proc_macro]
#[allow(clippy::too_many_lines)]
pub fn sql_forge(input: TokenStream) -> TokenStream {
    // ---- Phase 1: Parse the macro input into structured data ----
    let preprocessed = preprocess_result_key_placeholders(TokenStream2::from(input));
    let EnhancedQueryInput {
        db,
        result,
        force_scalar,
        sql,
        params,
        sections,
    } = match syn::parse2::<EnhancedQueryInput>(preprocessed) {
        Ok(v) => v,
        Err(err) => return err.to_compile_error().into(),
    };

    // ---- Phase 2: Resolve database type (from macro arg or Cargo.toml) ----
    let db = match db {
        Some(db) => db,
        None => match resolve_db_from_cargo_toml() {
            Ok(db) => db,
            Err(msg) => {
                return syn::Error::new(Span::call_site(), msg)
                    .to_compile_error()
                    .into();
            }
        },
    };

    // ---- Phase 3: Build result case definitions ----
    // Each result case is (optional_key, model_type, optional_scalar_type).
    // Scalar type is set for primitives and `scalar`-marked types.
    let result_cases: Vec<(Option<String>, Option<Type>, Option<Type>)> = match result {
        ResultSpec::None => {
            vec![(None, None, None)]
        }
        ResultSpec::Single(ref model) => {
            let scalar = if force_scalar {
                Some(model.clone())
            } else {
                scalar_output_type(model).cloned()
            };
            vec![(None, Some(model.clone()), scalar)]
        }
        ResultSpec::Group(ref cases) => {
            if force_scalar {
                return syn::Error::new(
                    Span::call_site(),
                    "sql_forge!: scalar mode is not supported for grouped result maps",
                )
                .to_compile_error()
                .into();
            }

            let mut out = Vec::new();
            let mut seen = HashSet::new();
            for case in cases {
                let key = case.name.to_string();
                if !seen.insert(key.clone()) {
                    return syn::Error::new(
                        case.name.span(),
                        "sql_forge!: duplicated key in result map",
                    )
                    .to_compile_error()
                    .into();
                }

                let scalar = if case.force_scalar {
                    Some(case.model.clone())
                } else {
                    scalar_output_type(&case.model).cloned()
                };
                out.push((Some(key), Some(case.model.clone()), scalar));
            }
            out
        }
    };
    let group_result_keys: Vec<String> = result_cases
        .iter()
        .filter_map(|(key, _, _)| key.clone())
        .collect();
    let is_grouped_result = !group_result_keys.is_empty();
    let sql_span = sql.span();

    // ---- Phase 4: Parse SQL into segments (text + {#section} slots) ----
    let segments = match sql.into_segments() {
        Ok(segments) => segments,
        Err(msg) => {
            return syn::Error::new(sql_span, msg).to_compile_error().into();
        }
    };

    let used_param_names = collect_used_param_names(&segments);

    // ---- Phase 5: Build parameter bindings for the top-level params ----
    let (declared_params, validator_param_bindings) =
        match build_param_bindings(&params, &used_param_names, "top_level", true, true) {
            Ok(v) => v,
            Err(err) => return err,
        };

    let sections_for_validation = sections.clone();
    let mut runtime_section_actions = HashMap::<String, TokenStream2>::new();
    let mut section_variants = HashMap::<String, Vec<SectionFragment>>::new();

    // ---- Phase 6: Process sections — build runtime actions and collect validation variants ----
    for assign in sections {
        let SectionAssign { names, value } = assign;

        // Build runtime actions first, while `value` is still available by reference.
        let mut named_actions: Vec<(String, TokenStream2)> = Vec::new();
        for (section_idx, name_ident) in names.iter().enumerate() {
            let name = name_ident.to_string();
            if runtime_section_actions.contains_key(&name) {
                return syn::Error::new(
                    name_ident.span(),
                    "sql_forge!: duplicated section mapping",
                )
                .to_compile_error()
                .into();
            }
            let action = match build_section_runtime_action(
                &value,
                section_idx,
                &format!("section_{}", name),
            ) {
                Ok(action) => action,
                Err(err) => return err,
            };
            named_actions.push((name, action));
        }

        // Consume `value` to collect per-section case variants without cloning.
        let variants_by_section = match collect_section_variants(value, names.len()) {
            Ok(v) => v,
            Err(msg) => {
                return syn::Error::new(names[0].span(), msg)
                    .to_compile_error()
                    .into();
            }
        };

        for ((name, action), section_cases) in named_actions
            .into_iter()
            .zip(variants_by_section.into_iter())
        {
            runtime_section_actions.insert(name.clone(), action);
            section_variants.insert(name, section_cases);
        }
    }

    let sql_section_names: std::collections::HashSet<&str> = segments
        .iter()
        .filter_map(|seg| {
            if let Segment::Section { name } = seg {
                Some(name.as_str())
            } else {
                None
            }
        })
        .collect();
    for name in runtime_section_actions.keys() {
        if !sql_section_names.contains(name.as_str()) {
            return syn::Error::new(
                sql_span,
                format!(
                    "sql_forge!: section `#{}` is declared in the section map but `{{#{}}}` never appears in the SQL",
                    name, name,
                ),
            )
            .to_compile_error()
            .into();
        }
    }

    // ---- Phase 8: For each result case, generate validator + runtime tokens ----
    let mut generated_query_defs = Vec::<TokenStream2>::new();
    let mut generated_query_values = Vec::<TokenStream2>::new();
    let mut group_field_defs = Vec::<TokenStream2>::new();
    let mut group_method_defs = Vec::<TokenStream2>::new();
    let mut group_field_idents = Vec::<syn::Ident>::new();
    let mut group_field_tys = Vec::<TokenStream2>::new();
    let mut group_trait_impls = Vec::<TokenStream2>::new();

    let mut grouped_validator_invocations = Vec::<TokenStream2>::new();

    for (_idx, (result_key, model_opt, scalar_model_ty)) in result_cases.iter().enumerate() {
        let suffix = result_key.clone().unwrap_or_else(|| "single".to_string());
        let query_ident = format_ident!("__EnhancedQuery_{}", suffix);
        let query_value_ident = format_ident!("__sql_forge_value_{}", suffix);

        let flag_bindings = build_result_flag_bindings(&group_result_keys, result_key.as_deref());

        let mut section_variants_for_validation = HashMap::<String, Vec<SectionFragment>>::new();
        for assign in sections_for_validation.iter().cloned() {
            let SectionAssign { names, value } = assign;
            let variants_by_section = match collect_section_variants_for_result(
                value,
                names.len(),
                result_key.as_deref(),
            ) {
                Ok(v) => v,
                Err(msg) => {
                    return syn::Error::new(names[0].span(), msg)
                        .to_compile_error()
                        .into();
                }
            };

            for (name_ident, section_cases) in
                names.into_iter().zip(variants_by_section.into_iter())
            {
                section_variants_for_validation.insert(name_ident.to_string(), section_cases);
            }
        }

        let mut nmax = 1usize;
        for segment in &segments {
            if let Segment::Section { name } = segment {
                if let Some(variants) = section_variants_for_validation.get(name) {
                    if variants.is_empty() {
                        return syn::Error::new(
                            sql_span,
                            format!("sql_forge!: section {{#{}}} has no possible variants", name),
                        )
                        .to_compile_error()
                        .into();
                    }
                    nmax = nmax.max(variants.len());
                } else {
                    return syn::Error::new(
                        sql_span,
                        format!("sql_forge!: section {{#{}}} has no mapping", name),
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }

        let mut validator_cases = Vec::<(LitStr, Vec<TokenStream2>, Vec<TokenStream2>)>::new();
        for case_idx in 0..nmax {
            let mut sql_case = String::new();
            let mut case_setup = Vec::<TokenStream2>::new();
            let mut case_args = Vec::<TokenStream2>::new();
            let empty_params = HashMap::<String, syn::Ident>::new();

            for segment in &segments {
                match segment {
                    Segment::Text(text) => {
                        let (chunk_sql, chunk_args) = match render_validator_args(
                            text,
                            &empty_params,
                            &declared_params,
                            true,
                            sql_span,
                        ) {
                            Ok(value) => value,
                            Err(err) => return err,
                        };
                        sql_case.push_str(&chunk_sql);
                        case_args.extend(chunk_args);
                    }
                    Segment::Section { name } => {
                        let Some(variants) = section_variants_for_validation.get(name) else {
                            return syn::Error::new(
                                sql_span,
                                format!("sql_forge!: section {{#{}}} has no mapping", name),
                            )
                            .to_compile_error()
                            .into();
                        };

                        let fragment = &variants[case_idx % variants.len()];
                        let used_param_names = collect_used_param_names_in_sql(&fragment.sql);
                        let (local_params, bindings) = match build_param_bindings(
                            &fragment.params,
                            &used_param_names,
                            &format!("section_case_{}_{}_{}", suffix, case_idx, name),
                            true,
                            true,
                        ) {
                            Ok(value) => value,
                            Err(err) => return err,
                        };
                        let (chunk_sql, chunk_args) = match render_validator_args(
                            &fragment.sql,
                            &local_params,
                            &declared_params,
                            true,
                            fragment.span,
                        ) {
                            Ok(value) => value,
                            Err(err) => return err,
                        };
                        sql_case.push_str(&chunk_sql);
                        case_setup.extend(bindings);
                        case_args.extend(chunk_args);
                    }
                }
            }

            validator_cases.push((LitStr::new(&sql_case, sql_span), case_setup, case_args));
        }

        let mut validator_invocations = Vec::<TokenStream2>::new();
        for (sql_lit, case_setup, args) in &validator_cases {
            if model_opt.is_none() {
                if args.is_empty() {
                    validator_invocations.push(quote! {
                        {
                            #( #case_setup )*
                            let _ = sqlx::query_scalar!(
                                #sql_lit,
                            );
                        }
                    });
                } else {
                    validator_invocations.push(quote! {
                        {
                            #( #case_setup )*
                            let _ = sqlx::query_scalar!(
                                #sql_lit,
                                #( #args ),*
                            );
                        }
                    });
                }
            } else if let Some(scalar_ty) = scalar_model_ty {
                if args.is_empty() {
                    validator_invocations.push(quote! {
                        {
                            #( #case_setup )*
                            let _ = sqlx::query_scalar!(
                                #sql_lit,
                            );
                        }
                    });
                } else {
                    validator_invocations.push(quote! {
                        {
                            #( #case_setup )*
                            let _ = sqlx::query_scalar!(
                                #sql_lit,
                                #( #args ),*
                            );
                        }
                    });
                }
                let _ = scalar_ty;
            } else if args.is_empty() {
                validator_invocations.push(quote! {
                    {
                        #( #case_setup )*
                        let _ = sqlx::query_as!(
                            __EnhancedModel,
                            #sql_lit,
                        );
                    }
                });
            } else {
                validator_invocations.push(quote! {
                    {
                        #( #case_setup )*
                        let _ = sqlx::query_as!(
                            __EnhancedModel,
                            #sql_lit,
                            #( #args ),*
                        );
                    }
                });
            }
        }

        let model_alias = if let Some(model) = model_opt {
            if scalar_model_ty.is_none() {
                quote! { type __EnhancedModel = #model; }
            } else {
                quote! {}
            }
        } else {
            quote! {}
        };
        grouped_validator_invocations.push(quote! {
            {
                #( #flag_bindings )*
                #model_alias
                #( #validator_invocations )*
            }
        });

        let (runtime_declared_params, runtime_param_bindings) =
            match build_param_bindings(&params, &used_param_names, "runtime", false, false) {
                Ok(v) => v,
                Err(err) => return err,
            };

        let mut runtime_steps = Vec::<TokenStream2>::new();
        for (seg_idx, segment) in segments.iter().enumerate() {
            match segment {
                Segment::Text(text) => {
                    for part in parse_text_parts(text) {
                        match part {
                            TextPart::Lit(lit) => {
                                let lit = sanitize_runtime_sql_text(&lit);
                                let lit_str = LitStr::new(&lit, sql_span);
                                runtime_steps.push(quote! {
                                    __builder.push(#lit_str);
                                });
                            }
                            TextPart::Param { name, is_list } => {
                                let Some(local_ident) = runtime_declared_params.get(&name) else {
                                    return syn::Error::new(
                                        sql_span,
                                        format!("sql_forge!: parameter :{} has no mapping", name),
                                    )
                                    .to_compile_error()
                                    .into();
                                };

                                if is_list {
                                    runtime_steps.push(quote! {
                                        let __enhanced_values = #local_ident;
                                        let mut __separated = __builder.separated(", ");
                                        for __value in __enhanced_values {
                                            __separated.push_bind(__value);
                                        }
                                    });
                                } else {
                                    runtime_steps.push(quote! {
                                        __builder.push_bind(#local_ident);
                                    });
                                }
                            }
                        }
                    }
                }
                Segment::Section { name } => {
                    let Some(section_action) = runtime_section_actions.get(name) else {
                        let _ = seg_idx;
                        return syn::Error::new(
                            sql_span,
                            format!("sql_forge!: section {{#{}}} has no mapping", name),
                        )
                        .to_compile_error()
                        .into();
                    };
                    runtime_steps.push(quote! {
                        #section_action
                    });
                }
            }
        }

        let exec_methods = if model_opt.is_none() {
            quote! {
                async fn execute<'e, E>(mut self, executor: E) -> Result<<#db as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = #db>,
                {
                    self.inner.build().execute(executor).await
                }
            }
        } else if let Some(scalar_ty) = scalar_model_ty {
            quote! {
                async fn fetch_all<'e, E>(mut self, executor: E) -> Result<Vec<#scalar_ty>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = #db>,
                {
                    self.inner
                        .build_query_scalar::<#scalar_ty>()
                        .fetch_all(executor)
                        .await
                }

                async fn fetch_one<'e, E>(mut self, executor: E) -> Result<#scalar_ty, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = #db>,
                {
                    self.inner
                        .build_query_scalar::<#scalar_ty>()
                        .fetch_one(executor)
                        .await
                }

                async fn fetch_optional<'e, E>(mut self, executor: E) -> Result<Option<#scalar_ty>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = #db>,
                {
                    self.inner
                        .build_query_scalar::<#scalar_ty>()
                        .fetch_optional(executor)
                        .await
                }

                async fn execute<'e, E>(mut self, executor: E) -> Result<<#db as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = #db>,
                {
                    self.inner.build().execute(executor).await
                }
            }
        } else {
            let model = model_opt.as_ref().unwrap();
            quote! {
                async fn fetch_all<'e, E>(mut self, executor: E) -> Result<Vec<#model>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = #db>,
                {
                    self.inner.build_query_as::<#model>().fetch_all(executor).await
                }

                async fn fetch_one<'e, E>(mut self, executor: E) -> Result<#model, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = #db>,
                {
                    self.inner.build_query_as::<#model>().fetch_one(executor).await
                }

                async fn fetch_optional<'e, E>(mut self, executor: E) -> Result<Option<#model>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = #db>,
                {
                    self.inner
                        .build_query_as::<#model>()
                        .fetch_optional(executor)
                        .await
                }

                async fn execute<'e, E>(mut self, executor: E) -> Result<<#db as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = #db>,
                {
                    self.inner.build().execute(executor).await
                }
            }
        };

        let final_type: TokenStream2 = if let Some(model) = model_opt {
            if let Some(scalar_ty) = scalar_model_ty {
                quote! { #scalar_ty }
            } else {
                quote! { #model }
            }
        } else {
            quote! {}
        };
        let trait_impl = if model_opt.is_none() {
            quote! {
                impl<'args> sql_forge::EnhancedQueryExecute
                    for #query_ident<'args>
                {
                    type Db = #db;

                    fn execute<'e, E>(self, executor: E) -> impl std::future::Future<Output = Result<<#db as sqlx::Database>::QueryResult, sqlx::Error>> + Send + 'e
                    where
                        Self: Sized + 'e,
                        E: sqlx::Executor<'e, Database = #db> + Send + 'e,
                        #db: 'e,
                    {
                        #query_ident::execute(self, executor)
                    }
                }
            }
        } else {
            quote! {
                impl<'args> sql_forge::EnhancedQuery<#final_type>
                    for #query_ident<'args>
                {
                    type Db = #db;

                    fn fetch_all<'e, E>(self, executor: E) -> impl std::future::Future<Output = Result<Vec<#final_type>, sqlx::Error>> + Send + 'e
                    where
                        Self: Sized + 'e,
                        E: sqlx::Executor<'e, Database = #db> + Send + 'e,
                        #db: 'e,
                    {
                        #query_ident::fetch_all(self, executor)
                    }

                    fn fetch_one<'e, E>(self, executor: E) -> impl std::future::Future<Output = Result<#final_type, sqlx::Error>> + Send + 'e
                    where
                        Self: Sized + 'e,
                        E: sqlx::Executor<'e, Database = #db> + Send + 'e,
                        #db: 'e,
                    {
                        #query_ident::fetch_one(self, executor)
                    }

                    fn fetch_optional<'e, E>(self, executor: E) -> impl std::future::Future<Output = Result<Option<#final_type>, sqlx::Error>> + Send + 'e
                    where
                        Self: Sized + 'e,
                        E: sqlx::Executor<'e, Database = #db> + Send + 'e,
                        #db: 'e,
                    {
                        #query_ident::fetch_optional(self, executor)
                    }

                    fn execute<'e, E>(self, executor: E) -> impl std::future::Future<Output = Result<<#db as sqlx::Database>::QueryResult, sqlx::Error>> + Send + 'e
                    where
                        Self: Sized + 'e,
                        E: sqlx::Executor<'e, Database = #db> + Send + 'e,
                        #db: 'e,
                    {
                        #query_ident::execute(self, executor)
                    }
                }
            }
        };

        generated_query_defs.push(quote! {
            struct #query_ident<'args> {
                inner: sqlx::QueryBuilder<'args, #db>,
            }

            impl<'args> #query_ident<'args> {
                #exec_methods
            }

            #trait_impl
        });

        generated_query_values.push(quote! {
            #( #runtime_param_bindings )*
            #( #flag_bindings )*
            let mut __builder: sqlx::QueryBuilder<#db> = sqlx::QueryBuilder::new("");
            #( #runtime_steps )*
            let #query_value_ident = #query_ident { inner: __builder };
        });

        if let Some(key) = result_key {
            let method_ident = format_ident!("{}", key);
            group_field_defs.push(quote! {
                #method_ident: #query_ident<'args>
            });
            group_field_idents.push(method_ident.clone());
            group_field_tys.push(quote! { #query_ident<'args> });
            group_method_defs.push(quote! {
                pub fn #method_ident(self) -> #query_ident<'args> {
                    self.#method_ident
                }
            });

            let key_ty_ident = format_ident!("__EnhancedQueryGroupKey_{}", key);
            group_trait_impls.push(quote! {
                struct #key_ty_ident;

                impl<'args> sql_forge::EnhancedQueryGroupGet<#key_ty_ident, #final_type> for __EnhancedQueryGroup<'args> {
                    type Query = #query_ident<'args>;

                    fn get(self, _: #key_ty_ident) -> Self::Query {
                        self.#method_ident
                    }
                }
            });
        }
    }

    // ---- Phase 8: Emit the final token stream ----
    let validator_tokens = quote! {
        let _sql_forge_validator = || {
            #( #validator_param_bindings )*
            #( #grouped_validator_invocations )*
        };
    };

    if !is_grouped_result {
        let single_query_value_ident = format_ident!("__sql_forge_value_single");
        return quote! {
            {
                #validator_tokens
                #( #generated_query_defs )*
                #( #generated_query_values )*
                #single_query_value_ident
            }
        }
        .into();
    }

    let group_field_inits: Vec<TokenStream2> = result_cases
        .iter()
        .filter_map(|(key, _, _)| key.as_ref())
        .map(|key| {
            let method_ident = format_ident!("{}", key);
            let query_value_ident = format_ident!("__sql_forge_value_{}", key);
            quote! { #method_ident: #query_value_ident }
        })
        .collect();

    quote! {
        {
            #validator_tokens

            #( #generated_query_defs )*
            #( #generated_query_values )*

            struct __EnhancedQueryGroup<'args> {
                #( #group_field_defs, )*
            }

            impl<'args> __EnhancedQueryGroup<'args> {
                #( #group_method_defs )*

                pub fn into_parts(self) -> ( #( #group_field_tys ),* ) {
                    ( #( self.#group_field_idents ),* )
                }
            }

            impl<'args> sql_forge::EnhancedQueryGroup for __EnhancedQueryGroup<'args> {
                type Db = #db;
            }

            #( #group_trait_impls )*

            __EnhancedQueryGroup {
                #( #group_field_inits, )*
            }
        }
    }
    .into()
}

/// Expands to the database type configured in `[package.metadata.sql_forge]`
/// of the caller's `Cargo.toml`.
///
/// ```toml
/// [package.metadata.sql_forge]
/// db = "sqlx::MySql"
/// ```
///
/// ```rust,ignore
/// use sql_forge::db_type;
///
/// pub type AppDb = db_type!();
/// // expands to: pub type AppDb = sqlx::MySql;
/// ```
#[proc_macro]
pub fn db_type(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        return syn::Error::new(Span::call_site(), "db_type!() takes no arguments")
            .to_compile_error()
            .into();
    }

    match resolve_db_from_cargo_toml() {
        Ok(db) => quote! { #db }.into(),
        Err(msg) => syn::Error::new(Span::call_site(), msg)
            .to_compile_error()
            .into(),
    }
}
