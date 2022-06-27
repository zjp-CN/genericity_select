use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Ident, Token, TypePath};

#[proc_macro_attribute]
pub fn genericity_select(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Substitute);
    let input = TokenStream2::from(input);

    let mut ts: Vec<TokenStream> = Vec::new();
    for pairs in args.iter() {
        let alias = pairs.into_iter().map(|(id, ty)| quote!( type #id = #ty; ));
        ts.push(quote!(const _: () = { #(#alias)* #input };).into());
    }

    ts.into_iter().collect()
}

#[derive(Debug)]
struct Substitute(Vec<Sub>);

impl Substitute {
    fn iter(&self) -> Iter {
        let sub = &self.0;
        let len = sub.iter().map(|v| v.ty.len());
        let total = len.clone().product();
        let mut v = Vec::with_capacity(len.len());
        len.fold(1, |acc, l| {
               let f = |n| n / acc % l;
               v.push((0..total).map(f).collect::<Vec<_>>());
               acc * l
           });
        // transpose
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        let mut pos: Vec<Vec<usize>> =
            (0..total).map(|_| iters.iter_mut().map_while(|n| n.next()).collect()).collect();
        pos.reverse();
        Iter { sub, pos }
    }
}

#[derive(Debug)]
struct Sub {
    id: Ident,
    ty: Vec<TypePath>,
}

struct Iter<'sub> {
    sub: &'sub [Sub],
    pos: Vec<Vec<usize>>,
}

type Pairs<'sub> = Vec<(&'sub Ident, &'sub TypePath)>;

impl<'sub> Iterator for Iter<'sub> {
    type Item = Pairs<'sub>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos.pop()?;
        let mut pair = Vec::with_capacity(self.sub.len());
        for (sub, p) in self.sub.iter().zip(pos.into_iter()) {
            pair.push((&sub.id, &sub.ty[p]))
        }
        Some(pair)
    }
}

impl Parse for Substitute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut sub = Vec::with_capacity(8);
        while !input.is_empty() {
            let id = input.parse()?;
            let _: Token![=] = input.parse()?;
            let mut ty = Vec::with_capacity(8);
            while !(input.is_empty() || input.peek(Token![,])) {
                ty.push(input.parse()?);
                if input.peek(Token![|]) {
                    let _: Token![|] = input.parse()?;
                }
            }
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
            assert!(!ty.is_empty(), "{:?} must at least select one type!", id);
            sub.push(Sub { id, ty });
        }
        Ok(Substitute(sub))
    }
}

#[test]
#[should_panic]
fn test_t0_empty() { helper("T = ", 0); }
#[test]
#[should_panic]
fn test_t0_empty_with_comma() { helper("T = ,", 0); }
#[test]
fn test_t1() { helper("T = u8", 1); }
#[test]
fn test_t1_u1() { helper("T = u8, U = usize", 1); }
#[test]
fn test_t2_u1() { helper("T = u8 | u32, U = usize", 2); }
#[test]
fn test_t1_u2() { helper("T = u8, U = Vec<&str> | std::collections::HashSet<&str>", 2); }
#[cfg(test)]
fn helper(s: &str, count: usize) {
    let parsed: Substitute = syn::parse_str(s).unwrap();
    assert_eq!(parsed.iter().count(), count);
}
