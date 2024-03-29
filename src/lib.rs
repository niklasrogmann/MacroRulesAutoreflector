/* #![feature(if_let_guard)]
#![feature(extend_one)] */

extern crate proc_macro;
//use proc_macro::{TokenStream};
use proc_macro2::{Ident, TokenStream};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields};
use syn::*;
use quote::{quote, ToTokens};

// TODO: use https://docs.rs/regex/latest/regex/#modules
use regex::Regex;

struct MacroArgs {
    field_ident : Ident,
    field_ty : Type,
    field_ident_string : String,
    field_ty_string : String,
    meta : Vec<Vec<TokenStream>>,
    user_args : TokenStream
}

impl MacroArgs {
    fn new(
        field_ident: Ident,
        field_ty: Type,
        field_ident_string: String,
        field_ty_string: String,
    ) -> Self {
        return MacroArgs{
            field_ident,
            field_ty,
            field_ident_string,
            field_ty_string,
            meta: Vec::<Vec<TokenStream>>::new(),
            user_args: TokenStream::new(),
        }
    }

    fn as_macro_arguments_token_stream(self) -> TokenStream {
        let field_ident = self.field_ident;
        let field_ty = self.field_ty;
        let field_ident_string = self.field_ident_string;
        let field_ty_string = self.field_ty_string;
        let meta = self.meta;
        let user_args = self.user_args;
        // meta brackets
        let expand_meta = meta.into_iter().map(|meta_in_bracket| {
            let expand_meta_more = meta_in_bracket.into_iter().map(|meta_arg| {
                return quote!(#meta_arg)
                //#(#for_each_input_attr)*
            });
            return quote!(#(#expand_meta_more,)*)
        });
        //return quote!(#macro_name!([#field_ident, #field_ty][#field_ident_string, #field_ty_string]#([#expand_meta])*; #user_args)).into()
        return quote!([#field_ident, #field_ty][#field_ident_string, #field_ty_string]#([#expand_meta])*; #user_args).into()
    }
}



/// numbers get additional data containing [i|u|f, number of bits]
/* fn for_any_number(field_ident : Option<Ident>, field_ty : Type, num : (String, usize)) -> TokenStream {
    // testing, injection
    let field_name_to_string = ident_opt_to_to_string(&field_ident);
    let field_ty_string = field_ty.to_token_stream().to_string();

    let (str_ty,bits) = num;
    let meta = 
    /* quote!{

    } */
    return write_into_macro_with_args()
    // testcode
    /* return quote!{
        println!("{} is {} of length {} bit and starts with {}",
            #field_name_to_string,
            #field_ty_string,
            #bits,
            #str_ty
        )
    }; */
} */

// useful: https://blog.turbo.fish/proc-macro-simple-derive/
// check this out: https://github.com/jakobhellermann/bevy-inspector-egui/blob/be57f0d88a18984ad450b2a984d3d1b76105a376/crates/bevy-inspector-egui-derive/src/lib.rs#L21
// TODO: generic capable
#[proc_macro_derive(Autoreflect, attributes(Autoreflect))]
pub fn autoreflect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let input = parse_macro_input!(input as DeriveInput);
    let input_ty = input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => fields.named,
        _ => panic!("only works on structs with named fields"),
    };

    // capture primitive number types with regex
    let any_num = Regex::new(r"^(u|i|f)([0-9]+)$").unwrap();
    // capture bits as well
    let capture_any_number = |field_ty_string : &String| -> Option<(String, usize)> {
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
        let defaultmsg = "need Autoreflect(macro_rules!-name to call with field data)";
        let test = attr.to_token_stream().to_string();
        if let Ok(attr) = attr.meta.require_list(){
            if attr.path.is_ident("Autoreflect") {
                // 
                // take attributes from comma seperated list
                // TODO: syn might have a better solution for this?
                let comma_seperated_list_in_list = attr.tokens.clone().into();
                let parser : Punctuated::<TokenStream, Token![,]> = Punctuated::<TokenStream, Token![,]>::parse_terminated.parse(comma_seperated_list_in_list).expect("could not parse");
                // parser = syn::parse::<>(k).expect("could not parse");
                //let test = parser[0];
                

                let tokens_iter : proc_macro2::token_stream::IntoIter = attr.tokens.clone().into_iter();
                let macro_rules_name_to_call_with_data = &parser[0];
                //let fn_name = parser[1];
                // debug
                let _test_1 = macro_rules_name_to_call_with_data.to_string();
                //let _test_2 = fn_name.to_string();

                // goes here when a correct input has been found
                // now needs to iterate over each field
                let field_builders = fields.clone().into_iter().map(|field| {
                    // Interpolation only works for variables, not arbitrary expressions.
                    // That's why we need to move these fields into local variables first
                    // (borrowing would also work though).

                    // fill arguments
                    let field_ident = field.ident.expect("has no field ident");
                    let field_ty = field.ty;
                    let field_attrs = field.attrs;
                    let field_ident_string = field_ident.to_token_stream().to_string();
                    let field_ty_string = field_ty.to_token_stream().to_string();
                    let mut macro_args = MacroArgs::new(field_ident, field_ty, field_ident_string, field_ty_string);

                    // for using strings in code:
                    //let field_name_to_string = ident_opt_to_to_string(&field_ident);
                    let opt_num = capture_any_number(&macro_args.field_ty_string);
                    if let Some(num) = opt_num {
                        let (str_ty,bits) = num;
                        macro_args.meta.push(vec![str_ty.to_token_stream(), quote!(#bits)]);
                        //write_into_macro_with_args(/* fn_name, */ macro_args);
                    } else {

                    }
                    /* let code_body = match field_ty_string.as_str() {
                        /* "String" => { quote!{}}
                        "bool" => {quote!{}} */
                        _ if let Some(num) = opt_num => {
                            let field_ty_string = field_ty.to_token_stream().to_string();

                            let (str_ty,bits) = num;
                            let meta = 
                            /* quote!{

                            } */
                            return write_into_macro_with_args(macro_args);
                            //for_any_number( field_ident,  field_ty,   num )
                        }
                        _ => {quote!{}}
                    }; */

                    
                    // TODO: user options
                    // user options need to be per-field to pass down the line within the user supplied macro

                    let per_field_data = macro_args.as_macro_arguments_token_stream();
                    quote! {
                        //println!("{} is {}", #field_name_to_string, #field_ty_string );
                        ( #per_field_data )
                        //println!("{}", #test );
                        
                        /* #field_name.(&self) -> &#field_ty {
                            &self.#field_name
                        } */
                    }
                });
                
                // return fn implementation
                return quote!{
                    #macro_rules_name_to_call_with_data!(@body {
                        #(#field_builders)*
                    });
                }.into()
            }
        }
        TokenStream::new()
    });

    // return all
    let output = quote!(#(#for_each_input_attr)*);
    output.into()
}


// when field name is an option, this unwraps this safely
// note that f.ident is None on tuples
/* fn ident_opt_to_to_string(ident : &Option<proc_macro2::Ident>) -> proc_macro2::TokenStream {
    match ident {
        Some(field_name) => format!("\"{}\".to_string()", field_name.to_string()).parse().expect("ident err"),
        None => format!("\"_\".to_string()").parse().expect("ident err")
    }
} */


// TODO: learn how this works
/*
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

 */