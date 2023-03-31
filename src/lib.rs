#![feature(if_let_guard)]
#![feature(extend_one)]

extern crate proc_macro;
//use proc_macro::{TokenStream};
use proc_macro2::{Ident, TokenStream};
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields};
use syn::*;
use quote::{quote, ToTokens};

// TODO: use https://docs.rs/regex/latest/regex/#modules
use regex::Regex;



fn for_any_number(field_ident : Option<Ident>, field_ty : Type, num : (String, usize)) -> TokenStream {
    // testing, injection
    let field_name_to_string = ident_opt_to_to_string(&field_ident);
    let field_ty_string = field_ty.to_token_stream().to_string();

    let (str_ty,bits) = num;

    quote!{
        println!("{} is {} of length {} bit and starts with {}",
        #field_name_to_string,
        #field_ty_string,
        #bits,
        #str_ty
    );
    }
}



// macro rule to pass along, this is an example
/* #[proc_macro]
pub fn match_ty_implementation_example( input : proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    quote!{
        #input
    }.into()
} */


// https://blog.turbo.fish/proc-macro-simple-derive/
// TODO: generic capable
#[proc_macro_derive(Autoreflect, attributes(set_for_field_derive))]
pub fn autoreflect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_ty = input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => fields.named,
        _ => panic!("only works on structs with named fields"),
    };

    /* Flex::column()
        .with_child(
            Slider::new()
                .with_range(0.05, 0.95)
                .with_step(0.10)
                .lens(#input_ty::#field_name),
        )
        .with_spacer(4.0)
        .with_child(Label::new(|data: &#input_ty, _: &_| {
            format!("{:3.2}%", data.#field_name * 100.)
    })) */



    // capture primitive number types with regex
    let any_num = Regex::new(r"^(u|i|f)([0-9]+)$").unwrap();
    // capture bits as well
    let capture_any_number = |field_ty_string : &String|
        -> Option<(String, usize)> {
        if let Some(num) = any_num.captures(&field_ty_string){
            Some((
                num.get(1).unwrap().as_str().to_string(), // String
                num.get(2).unwrap().as_str().parse::<usize>().expect(format!("{}{}",
                    num.get(1).unwrap().as_str(),
                    num.get(2).unwrap().as_str()
                ).as_str()) // usize
            )) } else {
            None
        }
    };

    /* let field_builders = fields.into_iter().map(|f| {
        // Interpolation only works for variables, not arbitrary expressions.
        // That's why we need to move these fields into local variables first
        // (borrowing would also work though).
        let field_ident = f.ident;
        let field_ty = f.ty;
        let field_attrs = f.attrs;
        // for using strings in code:
        let field_name_to_string = ident_opt_to_to_string(&field_ident);
        let field_ty_string = field_ty.to_token_stream().to_string();

        let opt_num = capture_any_number(&field_ty_string);
        let code_body = match field_ty_string.as_str() {
            "String" => { quote!{}}
            "bool" => {quote!{}}
            _ if let Some(num) = opt_num => {
                for_any_number( field_ident,  field_ty,   num )
            }
            _ => {quote!{}}
        };
        quote! {
            println!("{} is {}", #field_name_to_string, #field_ty_string );
            #code_body
            //println!("{}", #test );
            
            /* #field_name.(&self) -> &#field_ty {
                &self.#field_name
            } */
        }
    }); */
    //

    
    // check input attributes for all "need set_for_field_derive(for_each_field, fn_name)"
    // iterates over all attributes
    let for_each_input_attr = input.attrs.into_iter().map(|attr|{
        let defaultmsg = "need set_for_field_derive(for_each_field, fn_name)";
        let test = attr.to_token_stream().to_string();
        if let Ok(attr) = attr.meta.require_list(){
            if attr.path.is_ident("set_for_field_derive") {
                // take attributes from comma seperated list
                // TODO: syn might have a better solution for this?
                let mut tokens_iter : proc_macro2::token_stream::IntoIter = attr.tokens.clone().into_iter();
                let for_each_field = tokens_iter.next().expect(defaultmsg);
                assert! (tokens_iter.next().expect(defaultmsg).to_string() == ",", "missing ','");
                let fn_name = tokens_iter.next().expect(defaultmsg);
                // debug
                let _test_1 = for_each_field.to_string();
                let _test_2 = fn_name.to_string();

                // goes here when a correct input has been found
                // now needs to iterate over each field
                let field_builders = fields.clone().into_iter().map(|f| {
                    // Interpolation only works for variables, not arbitrary expressions.
                    // That's why we need to move these fields into local variables first
                    // (borrowing would also work though).
                    let field_ident = f.ident;
                    let field_ty = f.ty;
                    let field_attrs = f.attrs;
                    // for using strings in code:
                    let field_name_to_string = ident_opt_to_to_string(&field_ident);
                    let field_ty_string = field_ty.to_token_stream().to_string();

                    let opt_num = capture_any_number(&field_ty_string);
                    let code_body = match field_ty_string.as_str() {
                        "String" => { quote!{}}
                        "bool" => {quote!{}}
                        _ if let Some(num) = opt_num => {
                            for_any_number( field_ident,  field_ty,   num )
                        }
                        _ => {quote!{}}
                    };
                    quote! {
                        println!("{} is {}", #field_name_to_string, #field_ty_string );
                        #code_body
                        //println!("{}", #test );
                        
                        /* #field_name.(&self) -> &#field_ty {
                            &self.#field_name
                        } */
                    }
                });
                
                // return fn implementation
                return quote!{
                    //println!("test idents: {}   {}", #_test_1,#_test_2);
                    fn #fn_name (){
                        #(#field_builders)*
                    }
                }.into()
            }
        }
        //if test.starts_with("#[set_for_field_derive(");
        TokenStream::new()
    });

    let output = quote!{
        #(#for_each_input_attr)*
        /* fn test_build() /* -> impl Widget<#input_ty> */ {
            //#(#field_builders)*
            println!("!!!!");
            #(#input_attrs)*
        } */
    };
    
    output.into()
}


/// when field name is an option, this unwraps this safely
/// note that f.ident is None on tuples
fn ident_opt_to_to_string(ident : &Option<proc_macro2::Ident>) -> proc_macro2::TokenStream {
    match ident {
        Some(field_name) => format!("\"{}\".to_string()", field_name.to_string()).parse().expect("ident err"),
        None => format!("\"_\".to_string()").parse().expect("ident err")
    }
}


// TODO: learn how this works
fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| &idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

