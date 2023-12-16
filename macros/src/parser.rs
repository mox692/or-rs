use proc_macro::TokenStream;
use proc_macro2::Span as Span2;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Arm;
use syn::ExprMatch;
use syn::{Block, Expr, ExprIf, Ident, LocalInit, Pat, PathArguments, Stmt, Type};

use crate::error::{Error, Result};

// private macro parser
pub(crate) struct MacroParser {
    depth: usize,
    typ: Type,
}

impl MacroParser {
    pub(crate) fn new(input_token_stream: TokenStream) -> Self {
        let return_type = Self::get_return_type_from_input_token(input_token_stream.clone());

        MacroParser {
            depth: 0,
            typ: return_type,
        }
    }

    fn get_return_type_from_input_token(input_token_stream: TokenStream) -> Type {
        todo!()
    }
}

impl MacroParser {
    // parser's entry point
    pub(crate) fn parse(input: TokenStream) -> TokenStream {
        let local = match syn::parse2::<Stmt>(input.clone().into()) {
            Ok(Stmt::Local(local)) => local,
            Err(error) => panic!("{}", error),
            _ => panic!("expected Stmt::Local, but not"),
        };
        let let_tok = local.let_token;
        let (pat_tok, typ) = Self::parse_pat_and_ret_type(local.pat);
        let mut parser = MacroParser { depth: 0, typ: typ };
        let local_tok = match local.init {
            None => unreachable!(),
            Some(local_init) => parser.parse_local_init(local_init),
        };

        // TODO: check parse result err chack

        TokenStream::from(quote! { #let_tok #pat_tok #local_tok ;})
    }

    // parse `x: Or2<i32, f32>` in `let x: Or2<i32, f32> = if true { ... } else { ... }`
    fn parse_pat_and_ret_type(pat: Pat) -> (TokenStream2, Type) {
        match pat {
            Pat::Type(pat_type) => {
                let ident_tok = pat_type.pat.as_ref().clone();
                let typ_tok = pat_type.ty.as_ref().clone();
                (quote! { #ident_tok : #typ_tok }, typ_tok)
            }
            _ => panic!("expected Pat::Type, but not, pat: {:?}", &pat),
        }
    }

    // parse `= if true { ... } else { ... }` in `let x: Or2<i32, f32> = if true { ... } else { ... }`
    fn parse_local_init(&mut self, local_init: LocalInit) -> TokenStream2 {
        let expr_tok = self.parse_expr(local_init.expr.as_ref().clone());

        quote! { = #expr_tok }
    }

    // parse `if true { ... } else { ... }` in `let x: Or2<i32, f32> = if true { ... } else { ... }`
    fn parse_expr(&mut self, expr: Expr) -> TokenStream2 {
        let expr_tok = match expr {
            Expr::If(expr_if) => self.parse_expr_if(expr_if),
            Expr::Match(expr_match) => self.parse_expr_match(expr_match),
            Expr::Lit(expr_lit) => {
                let rewrited = self.rewrite_method_name(quote!(#expr_lit));
                quote!(#rewrited)
            }
            Expr::MethodCall(expr_method_call) => {
                let rewrited = self.rewrite_method_name(quote!(#expr_method_call));
                quote!(#rewrited)
            }
            _ => unreachable!(),
        };

        quote! { #expr_tok }
    }

    fn parse_expr_match(&mut self, expr_match: ExprMatch) -> TokenStream2 {
        let arms_tok: TokenStream2 = expr_match
            .arms
            .into_iter()
            .map(|arm| {
                self.depth += 1;
                self.parse_match_arm(arm)
            })
            .collect();

        quote! {
            match 33 {
                #arms_tok
            };
        }
    }

    fn parse_match_arm(&mut self, arm: Arm) -> TokenStream2 {
        let pat_tok = arm.pat;
        let expr_tok = self.parse_expr(arm.body.as_ref().clone());
        quote! {
                #pat_tok => #expr_tok,
        }
    }

    fn parse_expr_if(&mut self, expr_if: ExprIf) -> TokenStream2 {
        let then_tok = self.parse_then(expr_if.then_branch);
        let cond = expr_if.cond.as_ref().clone();
        let cond_tok = quote! { #cond };

        let cur_if = quote! {
            if #cond_tok #then_tok
        };

        match expr_if.else_branch {
            Some(else_branch) => {
                match else_branch.1.as_ref().clone() {
                    // else
                    Expr::Block(block) => {
                        let then = self.parse_then(block.block);
                        quote! { #cur_if else { #then } }
                    }
                    // else-if
                    Expr::If(_expr_if) => {
                        let _if = self.parse_expr_if(_expr_if);
                        quote! { #cur_if else #_if }
                    }
                    _ => panic!("expected else or elseif"),
                }
            }
            _ => cur_if,
        }
    }

    fn parse_then(&mut self, then_branch: Block) -> TokenStream2 {
        self.depth += 1;
        let stmts = then_branch.stmts;
        self.parse_stmts(stmts)
    }

    fn parse_stmts(&mut self, stmts: Vec<Stmt>) -> TokenStream2 {
        let (before, last) = stmts.split_at(stmts.len() - 1);

        let before_tok = quote! { #(#before)* };
        let last = quote! { #(#last)* };
        let rewrited_stmt = self.rewrite_method_name(last);
        let stmts = quote! {
            // then-block
            {
                #before_tok
                // return as `Or`
                #rewrited_stmt
            }
        };

        stmts
    }

    // get `Or3::Or3<i32, i32, f32>`
    fn rewrite_method_name(&mut self, wraped_expr: TokenStream2) -> TokenStream2 {
        let typ_tok = self.parse_enum_type();
        let method_name: Ident =
            Ident::new(format!("T{}", self.depth).as_str(), Span2::call_site());
        let or_type_name = self.get_or_type_name();
        quote! {
            #or_type_name::#typ_tok::#method_name(#wraped_expr)
        }
    }

    // get `<i32, i32, f32>`
    fn parse_enum_type(&mut self) -> TokenStream2 {
        let angle_bracket_tok = match &(self.typ) {
            Type::Path(ptype) => match ptype.path.segments.first().cloned() {
                Some(seg) => match seg.arguments {
                    PathArguments::AngleBracketed(args) => args,
                    _ => unreachable!(),
                },
                None => unreachable!(),
            },
            _ => unreachable!(),
        };
        quote!(#angle_bracket_tok)
    }

    // get `Or3` from Or3<i32,f32,String>
    fn get_or_type_name(&self) -> TokenStream2 {
        let ty = &self.typ;
        let str = quote!(#ty).to_string();
        //
        let idx = str
            .find("<")
            .unwrap_or_else(|| panic!("fail parse, expect token `,`. str: {}", str));
        let substr = &str[0..idx];
        substr
            .parse()
            .unwrap_or_else(|e| panic!("fail parse, expect token `,`. str: {}, error: {}", str, e))
    }
}
