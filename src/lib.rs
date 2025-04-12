use proc_macro::TokenStream;
use quote::quote;

/// Time the duration of code snippet, either to stdout or via `tracing`.
#[proc_macro]
pub fn howlast(input: TokenStream) -> TokenStream {

    // Convert the input TokenStream to a proc_macro2::TokenStream
    let block: proc_macro2::token_stream::TokenStream = input.into();

    let output = quote! {
        {
            let begin = line!();
            let start = std::time::Instant::now();
            let result =
                #block;
            let duration: std::time::Duration = start.elapsed();
            #[cfg(not(feature = "tracing"))]
            println!("{}:{} took {:?}.", file!(), begin, duration);
            #[cfg(feature = "tracing")]
            tracing::trace!("{}:{} took {:?}.", file!(), begin, duration);
            result
        }
    };

    output.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
