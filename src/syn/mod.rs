/// syn directors
pub mod direct;
/// syn visitors
pub mod visit;

/// The [syn::GenericParam]s of the generic scope being entered
#[repr(transparent)]
pub struct GenericsEnter(syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>);

impl GenericsEnter {
    pub(crate) fn new(
        params: &syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>,
    ) -> &Self {
        // Transmuting #[repr(transparent)] refs is safe
        unsafe { std::mem::transmute(params) }
    }

    pub(crate) fn new_mut(
        params: &mut syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>,
    ) -> &mut Self {
        // Transmuting #[repr(transparent)] refs is safe
        unsafe { std::mem::transmute(params) }
    }
}

impl<'g> IntoIterator for &'g GenericsEnter {
    type Item = &'g syn::GenericParam;
    type IntoIter = GenericsEnterIter<'g>;

    fn into_iter(self) -> Self::IntoIter {
        GenericsEnterIter(self.0.iter())
    }
}

/// Iterates the [syn::GenericParam]s of the generic scope being entered
pub struct GenericsEnterIter<'g>(syn::punctuated::Iter<'g, syn::GenericParam>);
impl<'g> Iterator for GenericsEnterIter<'g> {
    type Item = &'g syn::GenericParam;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// The [syn::GenericParam]s of the generic scope being exited
#[repr(transparent)]
pub struct GenericsExit(syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>);

impl GenericsExit {
    pub(crate) fn new(
        params: &syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>,
    ) -> &Self {
        // Transmuting #[repr(transparent)] refs is safe
        unsafe { std::mem::transmute(params) }
    }

    pub(crate) fn new_mut(
        params: &mut syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>,
    ) -> &mut Self {
        // Transmuting #[repr(transparent)] refs is safe
        unsafe { std::mem::transmute(params) }
    }
}

impl<'g> IntoIterator for &'g GenericsExit {
    type Item = &'g syn::GenericParam;
    type IntoIter = GenericsExitIter<'g>;

    fn into_iter(self) -> Self::IntoIter {
        GenericsExitIter(self.0.iter())
    }
}

/// Iterates the [syn::GenericParam]s of the generic scope being exited
pub struct GenericsExitIter<'g>(syn::punctuated::Iter<'g, syn::GenericParam>);

impl<'g> Iterator for GenericsExitIter<'g> {
    type Item = &'g syn::GenericParam;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::Ident;

    struct IdentCount(usize);

    impl crate::syn::visit::Full for IdentCount {
        fn visit_ident<'dv, D: ?Sized>(mut visitor: crate::Visitor<'dv, D, Self>, node: &Ident)
        where
            D: crate::Direct<Self, Ident>,
        {
            visitor.0 += 1;
            crate::Visitor::visit(visitor, node);
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

        crate::visit(&mut crate::syn::direct::FullDefault, &mut visit, &item);

        assert_eq!(visit.0, 9);
    }

    #[test]
    fn custom_direct() {
        let attr: syn::Attribute = syn::parse_quote! {
            #[custom_attr(these, are, actually, idents)]
        };

        struct IdentList(Vec<Ident>);

        impl crate::syn::visit::Full for IdentList {
            fn visit_ident<'dv, D: ?Sized>(mut visitor: crate::Visitor<'dv, D, Self>, node: &Ident)
            where
                D: crate::Direct<Self, Ident>,
            {
                visitor.0.push(node.clone());
                crate::Visitor::visit(visitor, node);
            }
        }

        struct AttrIdentList;

        impl crate::syn::direct::Full<IdentList> for AttrIdentList {
            fn direct_meta_list<'dv>(
                mut director: crate::Director<'dv, Self, IdentList>,
                node: &syn::MetaList,
            ) {
                if node.path.is_ident("custom_attr") {
                    crate::Director::direct(&mut director, &node.path);
                    crate::Director::direct(&mut director, &node.delimiter);

                    for ident in node
                        .parse_args_with(
                            syn::punctuated::Punctuated::<Ident, syn::Token![,]>::parse_terminated,
                        )
                        .unwrap()
                    {
                        crate::Director::direct(&mut director, &ident);
                    }
                } else {
                    super::direct::default::direct_meta_list(&mut director, node);
                }
            }
        }

        let mut visit = IdentList(vec![]);

        crate::visit(&mut AttrIdentList, &mut visit, &attr);

        assert_eq!(
            visit.0,
            vec!["custom_attr", "these", "are", "actually", "idents"]
        );
    }

    #[test]
    fn custom_direct_mut() {
        let mut attr: syn::Attribute = syn::parse_quote! {
            #[custom_attr(these, are, actually, idents)]
        };

        struct ReverseIdentVisitor;

        impl crate::syn::visit::FullMut for ReverseIdentVisitor {
            fn visit_ident_mut<'dv, D: ?Sized>(
                visitor: crate::Visitor<'dv, D, Self>,
                node: &mut Ident,
            ) where
                D: crate::DirectMut<Self, Ident>,
            {
                let rev_str = node.to_string().chars().rev().collect::<String>();
                *node = proc_macro2::Ident::new(rev_str.as_str(), node.span());
                crate::Visitor::visit_mut(visitor, node);
            }
        }

        struct AttrIdentList;

        impl crate::syn::direct::FullMut<ReverseIdentVisitor> for AttrIdentList {
            fn direct_meta_list_mut<'dv>(
                mut director: crate::Director<'dv, Self, ReverseIdentVisitor>,
                node: &mut syn::MetaList,
            ) {
                if node.path.is_ident("custom_attr") {
                    crate::Director::direct_mut(&mut director, &mut node.path);
                    crate::Director::direct_mut(&mut director, &mut node.delimiter);

                    let mut ident_list = node
                        .parse_args_with(
                            syn::punctuated::Punctuated::<Ident, syn::Token![,]>::parse_terminated,
                        )
                        .unwrap();
                    for ident in &mut ident_list {
                        crate::Director::direct_mut(&mut director, ident);
                    }
                    // Dynamically created nodes must be written back to their source
                    node.tokens = ident_list.to_token_stream();
                } else {
                    super::direct::default_mut::direct_meta_list_mut(&mut director, node);
                }
            }
        }

        crate::visit_mut(&mut AttrIdentList, &mut ReverseIdentVisitor, &mut attr);

        use quote::ToTokens as _;
        assert_eq!(
            attr.to_token_stream().to_string(),
            quote::quote!( #[rtta_motsuc(eseht, era, yllautca, stnedi)] ).to_string()
        );
    }

    #[test]
    fn generic_scopes() {
        let item: syn::Item = syn::parse_quote! {
            struct MyStruct<T> {
                my_field: Box<T>,
            }
        };

        struct PrintVisit(Vec<String>);

        impl visit::Full for PrintVisit {
            fn visit_ident<D>(mut visitor: crate::Visitor<'_, D, Self>, node: &proc_macro2::Ident)
            where
                D: crate::Direct<Self, proc_macro2::Ident> + ?Sized,
            {
                visitor.0.push(node.to_string());
            }

            fn visit_generics_enter<D>(
                mut visitor: crate::Visitor<'_, D, Self>,
                _node: &crate::syn::GenericsEnter,
            ) where
                D: crate::Direct<Self, crate::syn::GenericsEnter> + ?Sized,
            {
                visitor.0.push("enter".to_string());
            }

            fn visit_generics_exit<D>(
                mut visitor: crate::Visitor<'_, D, Self>,
                _node: &crate::syn::GenericsExit,
            ) where
                D: crate::Direct<Self, crate::syn::GenericsExit> + ?Sized,
            {
                visitor.0.push("exit".to_string());
            }
        }

        let mut visit = PrintVisit(vec![]);

        crate::visit(&mut crate::syn::direct::FullDefault, &mut visit, &item);

        assert_eq!(
            visit.0,
            ["enter", "MyStruct", "T", "my_field", "Box", "T", "exit",]
        );
    }
}
