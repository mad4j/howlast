use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Token, parse::Parse, parse_macro_input};

/// A struct to hold the arguments for the macro
/// The first argument is the variable name to hold the duration.
/// The second argument is the block of code to be executed.
/// The `duration_holder` is of type `Expr` to allow any valid Rust expression.
/// The `_comma` is a token to ensure that the arguments are separated by a comma.
/// The `code_block` is also of type `Expr` to allow any valid Rust expression.
struct MacroArgs {
    duration_holder: Expr,
    _comma: Token![,],
    code_block: Expr,
}

/// The `Parse` trait is implemented to parse the input TokenStream into our struct.
impl Parse for MacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            duration_holder: input.parse()?,
            _comma: input.parse()?,
            code_block: input.parse()?,
        })
    }
}

/// A macro to measure the duration of a code block
/// and store the result in a variable.
///
/// # Examples
/// ```
/// use howlast::howlast;
///
/// fn it_works() {
///     howlast!(step_duration,
///         {
///             let x = 1 + 1;
///             std::thread::sleep(std::time::Duration::from_secs(1));
///             x
///         }
///     );
///     print!("{:?}", step_duration);
/// }
/// ```
/// This macro takes two arguments:
/// 1. A variable name to hold the duration.
/// 2. A block of code to be executed.
/// The macro will measure the time taken to execute the block
/// and store the duration in the variable.
/// The block of code can be any valid Rust expression.
/// The duration is measured using `std::time::Instant`.
/// The macro will return the result of the block of code.
/// The duration is stored in the variable passed as the first argument.
/// The variable name must be a valid Rust identifier.
#[proc_macro]
pub fn howlast(input: TokenStream) -> TokenStream {
    // Parse the input TokenStream into our MacroArgs struct
    let input = parse_macro_input!(input as MacroArgs);

    let duration_holder = &input.duration_holder;
    let code_block = &input.code_block;

    // Create a new TokenStream with the timing code
    // and the original block of code
    let output = quote! {
        let #duration_holder = std::time::Instant::now();
        let result =
                #code_block;
        let #duration_holder: std::time::Duration = #duration_holder.elapsed();
        result
    };

    // return the generated code as a TokenStream
    output.into()
}
