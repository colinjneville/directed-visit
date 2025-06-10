
pub mod direct;
pub mod visit;

#[cfg(test)]
mod test {
    use proc_macro2::Ident;

    struct IdentCount(usize);

    impl<'n> crate::syn::visit::Full<'n> for IdentCount {
        fn visit_ident<'dv, D: ?Sized>(mut visitor: crate::Visitor<'dv, 'n, D, Self, Ident>, _node: &'n Ident)
        where D: crate::Direct<'n,Self,Ident> {
            visitor.0 += 1;
            crate::Visitor::visit(visitor);
        }
    }

    #[test]
    fn custom_visit() {
        let item: syn::Item = syn::parse_quote! {
            struct MyStruct<T> {
                a: i32,
                b: Box<u32>,
                c: [T],
            }
        };

        let mut visit = IdentCount(0);

        crate::visit(
            &mut crate::syn::direct::FullDefault,
            &mut visit,
            &item
        );

        assert_eq!(visit.0, 9);
    }

    #[test]
    fn custom_direct() {
        let attr: syn::Attribute = syn::parse_quote! {
            #[custom_attr(these, are, actually, idents)]
        };

        struct AttrIdentList;

        impl<'n> crate::syn::direct::Full<'n> for AttrIdentList {
            fn visit_meta<'dv, V: ?Sized>(mut director: crate::Director<'dv, Self, V>, node: &'n syn::Meta) 
                where V: crate::syn::visit::Full<'n>
            {
                if node.path().is_ident("custom_attr") {
                    let meta_list = node.require_list().unwrap();
                    for ident in 
                        meta_list.parse_args_with(
                            syn::punctuated::Punctuated::<Ident, syn::Token![,]>::parse_terminated
                        ).unwrap() 
                    {
                        crate::Director::direct(&mut director, &ident);
                    }
                } else {
                    super::direct::default::visit_meta(director, node);
                }
            }
        }

        let mut visit = IdentCount(0);

        crate::visit(
            &mut AttrIdentList,
            &mut visit,
            &attr
        );

        assert_eq!(visit.0, 5);
    }
}
