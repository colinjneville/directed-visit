
pub mod direct;
pub mod visit;

#[cfg(test)]
mod test {
    use proc_macro2::Ident;

    struct IdentCount(usize);

    impl crate::syn::visit::Full for IdentCount {
        fn visit_ident<'dv, 'n, D: ?Sized>(mut visitor: crate::Visitor<'dv, 'n, D, Self, Ident>, _node: &'n Ident)
        where D: crate::Direct<Self, Ident> {
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

        struct IdentList(Vec<Ident>);

        impl crate::syn::visit::Full for IdentList {
            fn visit_ident<'dv, 'n, D: ?Sized>(mut visitor: crate::Visitor<'dv, 'n, D, Self, Ident>, node: &'n Ident)
            where D: crate::Direct<Self, Ident> {
                visitor.0.push(node.clone());
                crate::Visitor::visit(visitor);
            }
        }

        struct AttrIdentList;

        impl crate::syn::direct::Full for AttrIdentList {
            fn visit_meta_list<'dv, V: ?Sized>(mut director: crate::Director<'dv, Self, V>, node: &syn::MetaList) 
                where V: crate::syn::visit::Full
            {
                if node.path.is_ident("custom_attr") {
                    crate::Director::direct(&mut director, &node.path);
                    crate::Director::direct(&mut director, &node.delimiter);

                    for ident in 
                        node.parse_args_with(
                            syn::punctuated::Punctuated::<Ident, syn::Token![,]>::parse_terminated
                        ).unwrap() 
                    {
                        crate::Director::direct(&mut director, &ident);
                    }
                } else {
                    super::direct::default::visit_meta_list(director, node);
                }
            }
        }

        let mut visit = IdentList(vec![]);

        crate::visit(
            &mut AttrIdentList,
            &mut visit,
            &attr
        );

        assert_eq!(visit.0, vec!["custom_attr", "these", "are", "actually", "idents"]);
    }
}
