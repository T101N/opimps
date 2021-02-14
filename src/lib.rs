extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{ quote, ToTokens };

use syn::{Attribute, parse_macro_input};

/// Implements the unary operators for the specified type.
/// 
/// ```
/// use core::ops::Not;
/// 
/// struct A {
///     val: bool
/// }
/// 
/// #[opimps::impl_uni_op(Not)]
/// fn not(self: A) -> A {
///     return A { val: !self.val };
/// }
/// 
/// let a = A { val: false };
/// let b = !a;
/// 
/// assert_eq!(true, b.val);
/// ```
#[proc_macro_attribute]
pub fn impl_uni_op(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_path = parse_macro_input!(attr as syn::TypePath);
    let fn_item = parse_macro_input!(item as syn::ItemFn);
    let fn_name = fn_item.sig.ident;
    let fn_generics = fn_item.sig.generics;
    let mut fn_args = fn_item.sig.inputs.into_iter();

    const INSUFFICIENT_ARGS_MSG: &str = "Function definition requires an argument (self: T).";

    let lhs = fn_args.next().expect(INSUFFICIENT_ARGS_MSG);

    let attrs = fn_item.attrs;

    let lhs = match lhs {
        syn::FnArg::Typed(e) => e,
        _ => { panic!("Error processing first argument.") }
    };

    let mut other_tkns = proc_macro2::TokenStream::new();
    
    attrs.into_iter().fold(
        &mut other_tkns,
        |tkn, attr|{ tkn.extend(attr.to_token_stream()); tkn }
    );

    let lhs_type = lhs.clone().ty;

    let fn_body = fn_item.block;
    
    let fn_type = match fn_item.sig.output {
        syn::ReturnType::Type(_, typ) => typ,
        _ => { panic!("Function must contain a return type.") }
    };

    let token = quote! {
        impl #fn_generics #trait_path for #lhs_type {
            type Output = #fn_type;
            #other_tkns
            fn #fn_name (self) -> Self::Output 
                #fn_body
        }
    };

    TokenStream::from(token)
}

/// Implements the unary operators for the specified type.
/// 
/// ```
/// use std::ops::Not;
/// 
/// struct A {
///     val: bool
/// }
/// 
/// #[opimps::impl_uni_ops(Not)]
/// fn not(self: A) -> bool {
///     return !self.val;
/// }
/// 
/// let a = A { val: false };
/// 
/// let b = !&a;
/// assert_eq!(true, b);
/// 
/// let b = !a;
/// assert_eq!(true, b);
/// ```
#[proc_macro_attribute]
pub fn impl_uni_ops(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_path = parse_macro_input!(attr as syn::TypePath);
    let fn_item = parse_macro_input!(item as syn::ItemFn);
    let fn_name = fn_item.sig.ident;
    let fn_generics = fn_item.sig.generics;

    let mut fn_args = fn_item.sig.inputs.into_iter();
    const INSUFFICIENT_ARGS_MSG: &str = "Function definition requires an argument (self: T).";

    let lhs = fn_args.next().expect(INSUFFICIENT_ARGS_MSG);

    let attrs = fn_item.attrs;

    let lhs = match lhs {
        syn::FnArg::Typed(e) => e,
        _ => { panic!("Error processing first argument.")}
    };

    let (comments, other_tkns) = extract_comments(&attrs);

    let lhs_pat = lhs.clone().pat;
    let lhs_type = lhs.clone().ty;

    let fn_body = fn_item.block;
    
    let fn_output = match fn_item.sig.output {
        syn::ReturnType::Type(_, typ) => typ,
        _ => { panic!("Function must contain a return type.") }
    };

    let token = quote! {
        #comments
        #other_tkns
        #[opimps::impl_uni_op(#trait_path)]
        fn #fn_name #fn_generics (#lhs) -> #fn_output 
            #fn_body

        #other_tkns
        #[opimps::impl_uni_op(#trait_path)]
        fn #fn_name #fn_generics (#lhs_pat: &#lhs_type) -> #fn_output 
            #fn_body
    };

    TokenStream::from(token)
}

/// The direct implementation for binary operators. This is used when you only need one implementation.
/// 
/// ```
/// pub struct TestObj {
///     val: i32
/// }
/// 
/// #[opimps::impl_op(std::ops::Mul)]
/// fn mul(self: TestObj, rhs: TestObj) -> i32 {
///    return self.val * rhs.val;
/// }
/// 
/// #[opimps::impl_op(std::ops::Mul)]
/// fn mul(self: &TestObj, rhs: TestObj) -> i32 {
///    return self.val * rhs.val;
/// }
/// 
/// #[opimps::impl_op(std::ops::Mul)]
/// fn mul(self: &TestObj, rhs: &TestObj) -> i32 {
///    return self.val * rhs.val;
/// }
/// 
/// let a = TestObj { val: 4 };
/// let b = TestObj { val: 7 };
/// 
/// assert_eq!(28, &a * &b);
/// assert_eq!(28, a * b);
/// 
/// 
/// let a = TestObj { val: 4 };
/// let b = TestObj { val: 7 };
/// 
/// assert_eq!(28, &a * b);
/// ```
#[proc_macro_attribute]
pub fn impl_op(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_path = parse_macro_input!(attr as syn::TypePath);
    let fn_item = parse_macro_input!(item as syn::ItemFn);
    let fn_name = fn_item.sig.ident;
    let fn_generics = fn_item.sig.generics;
    let mut fn_args = fn_item.sig.inputs.into_iter();

    const INSUFFICIENT_ARGS_MSG: &str = "Requires two arguments (self: T1, rhs: T2).";

    let lhs = fn_args.next().expect(INSUFFICIENT_ARGS_MSG);
    let rhs = fn_args.next().expect(INSUFFICIENT_ARGS_MSG);

    let attrs = fn_item.attrs;

    let lhs = match lhs {
        syn::FnArg::Typed(e) => e,
        _ => { panic!("Error processing first argument.")}
    };

    let rhs = match rhs {
        syn::FnArg::Typed(e) => e,
        _ => { panic!("Error processing second argument.")}
    };

    let mut other_tkns = proc_macro2::TokenStream::new();

    attrs.into_iter().fold(
        &mut other_tkns,
        |tkn, attr|{ tkn.extend(attr.to_token_stream()); tkn }
    );

    let lhs_type = lhs.clone().ty;
    let rhs_type = rhs.clone().ty;

    let fn_body = fn_item.block;
    
    let fn_output = match fn_item.sig.output {
        syn::ReturnType::Type(_, typ) => typ,
        _ => { panic!("Function must contain a return type.") }
    };

    let token = quote! {
        impl #fn_generics #trait_path<#rhs_type> for #lhs_type {
            type Output = #fn_output;
            #other_tkns
            fn #fn_name (self, #rhs) -> Self::Output {
                #fn_body
            }
        }
    };

    TokenStream::from(token)
}

/// Implements the permutations of owned and borrowed data.
/// 
/// ```
/// use std::ops::Mul;
///
/// pub struct ANumber {
///     val: i32
/// }
///
/// #[opimps::impl_ops(Mul)] 
/// fn mul(self: ANumber, rhs: i32) -> i32 {
///     return self.val * rhs;
/// }
/// 
/// let a = ANumber { val: 4 };
/// let b = 7;
/// 
/// assert_eq!(28, &a * &b);
/// assert_eq!(28, a * b);
/// ```
#[proc_macro_attribute]
pub fn impl_ops(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_path = parse_macro_input!(attr as syn::TypePath);
    let fn_item = parse_macro_input!(item as syn::ItemFn);
    let fn_name = fn_item.sig.ident;
    let fn_generics= fn_item.sig.generics;
    let mut fn_args = fn_item.sig.inputs.into_iter();

    const INSUFFICIENT_ARGS_MSG: &str = "Requires two arguments (self: T1, rhs: T2).";

    let lhs = fn_args.next().expect(INSUFFICIENT_ARGS_MSG);
    let rhs = fn_args.next().expect(INSUFFICIENT_ARGS_MSG);
    
    let lhs = match lhs {
        syn::FnArg::Typed(e) => e,
        _ => { panic!("Error processing first argument.")}
    };

    let rhs = match rhs {
        syn::FnArg::Typed(e) => e,
        _ => { panic!("Error processing second argument.")}
    };

    let lhs_pat = lhs.clone().pat;
    let lhs_type = lhs.clone().ty;
    let rhs_pat = rhs.clone().pat;
    let rhs_type = rhs.clone().ty;
    
    let fn_body = fn_item.block;    
    let fn_output = match fn_item.sig.output {
        syn::ReturnType::Type(_, typ) => typ,
        _ => { panic!("Function must contain a return type.") }
    };

    let attrs = fn_item.attrs;

    let (comments, other_tkns) = extract_comments(&attrs);

    let token = quote!{
        #comments
        #other_tkns
        #[opimps::impl_op(#trait_path)]
        fn #fn_name #fn_generics (#lhs, #rhs) -> #fn_output 
            #fn_body

        #other_tkns
        #[opimps::impl_op(#trait_path)]
        fn #fn_name #fn_generics (#lhs_pat: &#lhs_type, #rhs_pat: &#rhs_type) -> #fn_output 
            #fn_body

        #other_tkns
        #[opimps::impl_op(#trait_path)]
        fn #fn_name #fn_generics (#lhs_pat: #lhs_type, #rhs_pat: &#rhs_type) -> #fn_output 
            #fn_body

        #other_tkns
        #[opimps::impl_op(#trait_path)]
        fn #fn_name #fn_generics (#lhs_pat: &#lhs_type, #rhs_pat: #rhs_type) -> #fn_output
            #fn_body
    };
    
    TokenStream::from(token)
}

/// Implements the permutations of owned and borrowed data, with `rhs` being a 
/// primitive value and `self` being a structure.
/// 
/// ```
/// use std::ops::Mul;
///
/// pub struct ANumber {
///     val: i32
/// }
///
/// #[opimps::impl_ops(Mul)] 
/// fn mul(self: ANumber, rhs: i32) -> i32 {
///     return self.val * rhs;
/// }
/// 
/// let a = ANumber { val: 4 };
/// let b = 7;
/// 
/// assert_eq!(28, &a * b);
/// assert_eq!(28, a * b);
/// ```
#[proc_macro_attribute]
pub fn impl_ops_rprim(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_path = parse_macro_input!(attr as syn::TypePath);
    let fn_item = parse_macro_input!(item as syn::ItemFn);
    let fn_name = fn_item.sig.ident;
    let fn_generics = fn_item.sig.generics;
    let mut fn_args = fn_item.sig.inputs.into_iter();

    const INSUFFICIENT_ARGS_MSG: &str = "Requires two arguments (self: T1, rhs: T2).";

    let lhs = fn_args.next().expect(INSUFFICIENT_ARGS_MSG);
    let rhs = fn_args.next().expect(INSUFFICIENT_ARGS_MSG);
    
    let lhs = match lhs {
        syn::FnArg::Typed(e) => e,
        _ => { panic!("Error processing first argument.")}
    };

    let rhs = match rhs {
        syn::FnArg::Typed(e) => e,
        _ => { panic!("Error processing second argument.")}
    };

    let lhs_pat = lhs.clone().pat;
    let lhs_type = lhs.clone().ty;
    let rhs_pat = rhs.clone().pat;
    let rhs_type = rhs.clone().ty;
    
    let fn_body = fn_item.block;    
    let fn_output = match fn_item.sig.output {
        syn::ReturnType::Type(_, typ) => typ,
        _ => { panic!("Function must contain a return type.") }
    };

    let attrs = fn_item.attrs;
    
    let (comments, other_tkns) = extract_comments(&attrs);

    let token = quote!{
        #comments
        #other_tkns
        #[opimps::impl_op(#trait_path)]
        fn #fn_name #fn_generics (#lhs, #rhs) -> #fn_output 
            #fn_body

        #other_tkns
        #[opimps::impl_op(#trait_path)]
        fn #fn_name #fn_generics (#lhs_pat: &#lhs_type, #rhs_pat: #rhs_type) -> #fn_output 
            #fn_body
    };
    
    TokenStream::from(token)
}

/// Implements the permutations of owned and borrowed data, with `self` being a 
/// primitive value and `rhs` being a structure.
/// 
/// ```
/// use std::ops::Mul;
///
/// pub struct ANumber {
///     val: i32
/// }
///
/// #[opimps::impl_ops_lprim(Mul)] 
/// fn mul(self: i32, rhs: ANumber) -> i32 {
///     return self * rhs.val;
/// }
/// 
/// let a = 7;
/// let b = ANumber { val: 4 };
/// 
/// assert_eq!(28, a * &b);
/// assert_eq!(28, a * b);
/// ```
#[proc_macro_attribute]
pub fn impl_ops_lprim(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_path = parse_macro_input!(attr as syn::TypePath);
    let fn_item = parse_macro_input!(item as syn::ItemFn);
    let fn_name = fn_item.sig.ident;
    let fn_generics = fn_item.sig.generics;
    let mut fn_args = fn_item.sig.inputs.into_iter();

    const INSUFFICIENT_ARGS_MSG: &str = "Requires two arguments (self: T1, rhs: T2).";

    let lhs = fn_args.next().expect(INSUFFICIENT_ARGS_MSG);
    let rhs = fn_args.next().expect(INSUFFICIENT_ARGS_MSG);
    
    let lhs = match lhs {
        syn::FnArg::Typed(e) => e,
        _ => { panic!("Error processing first argument.")}
    };

    let rhs = match rhs {
        syn::FnArg::Typed(e) => e,
        _ => { panic!("Error processing second argument.")}
    };

    let lhs_pat = lhs.clone().pat;
    let lhs_type = lhs.clone().ty;
    let rhs_pat = rhs.clone().pat;
    let rhs_type = rhs.clone().ty;
    
    let fn_body = fn_item.block;    
    let fn_output = match fn_item.sig.output {
        syn::ReturnType::Type(_, typ) => typ,
        _ => { panic!("Function must contain a return type.") }
    };

    let attrs = fn_item.attrs;

    let (comments, other_tkns) = extract_comments(&attrs);
    
    let token = quote!{
        #comments
        #other_tkns
        #[opimps::impl_op(#trait_path)]
        fn #fn_name #fn_generics (#lhs, #rhs) -> #fn_output 
            #fn_body
        
        #other_tkns
        #[opimps::impl_op(#trait_path)]
        fn #fn_name #fn_generics (#lhs_pat: #lhs_type, #rhs_pat: &#rhs_type) -> #fn_output 
            #fn_body
    };
    
    TokenStream::from(token)
}

fn extract_comments(attrs: &Vec<Attribute>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut other_tkns = proc_macro2::TokenStream::new();
    let mut comments = proc_macro2::TokenStream::new();

    for attr in attrs.into_iter() {
        if attr.path.is_ident("doc") {
            comments.extend(attr.to_token_stream());
        } else {
            other_tkns.extend(attr.to_token_stream());
        }
    }

    (comments, other_tkns)
}
