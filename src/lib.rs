//! Implement the visitor pattern with interchangable implementations for both the visit algorithm and node object traversal. It is also possible to dynamically create temporary 'virtual' nodes during traversal.  
//!
//! An object traversal is comprised of 3 parts:  
//! The input to traverse, which can start at any node type. This can be an external type too, there are no required derives or trait impls.  
//! The director navigates between node objects. It must implement `Direct<N>` for each node type `N` in the object graph, determining the sub-nodes.  
//! The visitor performs the desired algorithm, implementing `Visit<N>` for each node type `N` in the object graph.  
//!
//! ```rust,ignore
//! fn my_visit(input: &MyTree) -> usize {
//!     let mut my_director = MyDirector::new();
//!     let mut my_visitor = MyVisitor::new();
//!     directed_visit::visit(
//!         &mut my_director,
//!         &mut my_visitor,
//!         input,
//!     );
//!
//!     my_visitor.result_value()
//! }
//! ```
//!
//! ## syn
//! The crate includes a replacement for `syn::visit::Visit` if the `syn` feature is enabled. Implement `directed_visit::syn::visit::Full` as you would `syn::visit::Visit`.  
//! For your director, `directed_visit::syn::direct::FullDefault` traverses as `syn::visit` does, or you can customize the behavior by implementing `directed_visit::syn::direct::Full`.  
//! In addition to the existing syn AST, two nodes have been added to the tree to represent when generic parameters become in and out of scope.  
//! The `derive` feature subset of `full` is not yet supported.
//!
//! ## Limitations
//! Because the director can dynamically create new nodes to visit, the visitor cannot hold references to the node graph (i.e. there is no single `'ast` lifetime for all nodes). For this reason there is also currently no `VisitMut` equivalent, because the ideal interaction between handling temporary dynamic nodes and mutating them is unclear.  
#![warn(missing_docs)]

mod direct;
pub use direct::{Direct, Director};
/// Direct and Visit implementations for the syn AST
#[cfg(feature = "syn")]
pub mod syn;
mod visit;
pub use visit::{Visit, Visitor};

#[derive(Debug)]
pub(crate) struct DirectorVisitor<'d, A: ?Sized, V: ?Sized> {
    pub(crate) direct: &'d mut A,
    pub(crate) visit: &'d mut V,
}

impl<A: ?Sized, V: ?Sized> DirectorVisitor<'_, A, V> {
    pub(crate) fn reborrow(&mut self) -> DirectorVisitor<'_, A, V> {
        let Self { direct, visit } = self;

        DirectorVisitor { direct, visit }
    }
}

/// Perform a visit using a Direct-Visit pair, and a given input.
pub fn visit<'dv, D, V, N>(direct: &'dv mut D, visit: &'dv mut V, node: &N)
where
    D: Direct<V, N> + ?Sized,
    V: Visit<N> + ?Sized,
    N: ?Sized,
{
    V::visit(Visitor::new(DirectorVisitor { direct, visit }, node), node);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct A(Option<B>);
    struct B(Vec<C>);
    struct C(A);

    struct MyDirect;
    impl crate::Direct<MyVisit, A> for MyDirect {
        fn direct<'dv>(mut director: Director<'dv, Self, MyVisit>, node: &A) {
            if let Some(b) = &node.0 {
                Director::direct(&mut director, b);
            }
        }
    }

    impl crate::Direct<MyVisit, B> for MyDirect {
        fn direct<'dv>(mut director: Director<'dv, Self, MyVisit>, node: &B) {
            for c in &node.0 {
                Director::direct(&mut director, c);
            }
        }
    }

    impl crate::Direct<MyVisit, C> for MyDirect {
        fn direct<'dv>(mut director: Director<'dv, Self, MyVisit>, node: &C) {
            Director::direct(&mut director, &node.0)
        }
    }

    struct MyVisit(usize);
    impl crate::Visit<A> for MyVisit {
        fn visit<'dv, 'n, D: ?Sized>(mut visitor: Visitor<'dv, 'n, D, Self, A>, _node: &'n A)
        where
            D: Direct<Self, A>,
        {
            visitor.0 += 1;
            Visitor::visit(visitor);
        }
    }
    impl crate::Visit<B> for MyVisit {
        fn visit<'dv, 'n, D: ?Sized>(mut visitor: Visitor<'dv, 'n, D, Self, B>, _node: &'n B)
        where
            D: Direct<Self, B>,
        {
            visitor.0 += 2;
            Visitor::visit(visitor);
        }
    }
    impl crate::Visit<C> for MyVisit {
        fn visit<'dv, 'n, D: ?Sized>(mut visitor: Visitor<'dv, 'n, D, Self, C>, _node: &'n C)
        where
            D: Direct<Self, C>,
        {
            visitor.0 += 3;
            Visitor::visit(visitor);
        }
    }

    #[test]
    fn custom_node_set() {
        let input = A(Some(
            // 1
            B(vec![
                // 3
                C(
                    // 6
                    A(None),
                ), // 7
                C(
                    // 10
                    A(Some(
                        // 11
                        B(vec![]), // 13
                    )),
                ),
            ]),
        ));

        let mut direct = MyDirect;
        let mut visit = MyVisit(0);
        crate::visit(&mut direct, &mut visit, &input);

        assert_eq!(visit.0, 13);
    }
}
