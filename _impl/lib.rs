use proc_macro::TokenStream;
use proc_macro2::{Group, TokenStream as TokenStream2, TokenTree as TT};
use quote::quote;
use syn::{buffer::TokenBuffer, parse::Parse, parse_macro_input, Ident, Token, TypePath};

#[proc_macro_attribute]
pub fn genericity_select(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Substitute);
    let buf = TokenBuffer::new(input);

    let mut ts = Vec::new();
    for pairs in args.iter() {
        let mut cursor = buf.begin();
        while let Some((tt, c)) = cursor.token_tree() {
            cursor = c;
            ts.push(token_tree(tt, &pairs));
        }
    }

    ts.into_iter().map(TokenStream::from).collect()
}

fn token_tree(tt: TT, pairs: &Pairs) -> TokenStream2 {
    match tt {
        TT::Ident(i) => pairs.iter()
                             .find_map(|(id, ty)| if **id == i { Some(quote!(#ty)) } else { None })
                             .unwrap_or_else(|| TT::Ident(i).into()),
        TT::Group(g) => group(g, pairs),
        t => t.into(),
    }
}

fn group(g: Group, id_ty: &Pairs) -> TokenStream2 {
    let (del, span) = (g.delimiter(), g.span());
    let ts = g.stream().into_iter().map(|tt| token_tree(tt, id_ty)).collect();
    let mut new = Group::new(del, ts);
    new.set_span(span);
    TT::Group(new).into()
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
            let mut ty = Vec::new();
            while !(input.is_empty() || input.peek(Token![,])) {
                ty.push(input.parse()?);
                if input.peek(Token![|]) {
                    let _: Token![|] = input.parse()?;
                }
            }
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
            sub.push(Sub { id, ty });
        }
        Ok(Substitute(sub))
    }
}

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
