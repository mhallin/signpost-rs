#![warn(missing_docs)]

//! Compile-time convenience macros for the `signpost` crate

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, LitByteStr, LitStr, Result, Token,
};

/// Compile-time construct a logger for points of interest.
///
/// ```ignore
/// use signpost::{OsLog, poi_logger};
/// static LOGGER: OsLog = poi_logger!("Subsystem name");
/// ```
#[proc_macro]
pub fn const_poi_logger(input: TokenStream) -> TokenStream {
    let PoiLoggerArgs { name } = parse_macro_input!(input as PoiLoggerArgs);

    let name_bstr = match str_lit_to_static_cstr(name) {
        Ok(name_bstr) => name_bstr,
        Err(name) => {
            return TokenStream::from(
                syn::Error::new_spanned(name, "The logger name can not contain NULL bytes")
                    .into_compile_error(),
            )
        }
    };

    let call = quote! {
        signpost::OsLog::new(
            #name_bstr,
            signpost::OsLog::CATEGORY_POINTS_OF_INTEREST
        )
    };

    TokenStream::from(call)
}

/// Emit an event on a logger.
///
/// The arguments are `logger`, `id`, `name`:
///
/// * `id` needs to be a non-zero positive integer, preferably unique
///   per type of event logged
/// * `name` is a string literal that will identify the event in Instruments.
///
/// ```ignore
/// use signpost::{OsLog, const_poi_logger};
/// static LOGGER: OsLog = const_poi_logger!("Subsystem name")
///
/// fn myfunc() {
///     signpost::emit_event!(LOGGER, 1, "My event");
/// }
/// ```
#[proc_macro]
pub fn emit_event(input: TokenStream) -> TokenStream {
    let EventArgs { log, id, name } = parse_macro_input!(input as EventArgs);

    let name_bstr = match str_lit_to_static_cstr(name) {
        Ok(name_bstr) => name_bstr,
        Err(name) => {
            return TokenStream::from(
                syn::Error::new_spanned(name, "The event name can not contain NULL bytes")
                    .into_compile_error(),
            )
        }
    };

    let call = quote! { #log.emit_event(#id, #name_bstr) };

    TokenStream::from(call)
}

/// Start a signpost interval on a logger
///
/// Similar to `emit_event` but this function stars an interval with a scope
/// guard that automatically ends the interval
///
/// ```ignore
/// use signpost::{OsLog, const_poi_logger};
/// static LOGGER: OsLog = const_poi_logger!("Subsystem name")
///
/// fn myfunc() {
///     let _interval = signpost::begin_interval!(LOGGER, 2, "Compute result");
///     // do work
///     // `_interval` will end the interval when it is dropped
/// }
/// ```
#[proc_macro]
pub fn begin_interval(input: TokenStream) -> TokenStream {
    let EventArgs { log, id, name } = parse_macro_input!(input as EventArgs);

    let name_bstr = match str_lit_to_static_cstr(name) {
        Ok(name_bstr) => name_bstr,
        Err(name) => {
            return TokenStream::from(
                syn::Error::new_spanned(name, "The event name can not contain NULL bytes")
                    .into_compile_error(),
            )
        }
    };

    let call = quote! { #log.begin_interval(#id, #name_bstr) };

    TokenStream::from(call)
}

struct EventArgs {
    log: Expr,
    id: Expr,
    name: LitStr,
}

impl Parse for EventArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let log = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let id = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let name = input.parse::<LitStr>()?;
        input.parse::<Option<Token![,]>>()?;

        Ok(EventArgs { log, id, name })
    }
}

struct PoiLoggerArgs {
    name: LitStr,
}

impl Parse for PoiLoggerArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<LitStr>()?;
        input.parse::<Option<Token![,]>>()?;

        Ok(PoiLoggerArgs { name })
    }
}

fn str_lit_to_static_cstr(input: LitStr) -> std::result::Result<impl ToTokens, LitStr> {
    let mut name_str = input.value();
    if name_str.contains('\0') {
        return Err(input);
    }
    name_str.push('\0');
    let name_bstr = LitByteStr::new(name_str.as_bytes(), input.span());

    Ok(quote! { unsafe { &*(#name_bstr as *const [u8] as *const std::ffi::CStr) } })
}
