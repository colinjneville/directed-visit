mod direct;
pub use direct::{Direct, Director};
#[cfg(feature = "syn")]
pub mod syn;
mod visit;
pub use visit::{Visit, Visitor};

pub(crate) struct DirectorVisitor<'d, A: ?Sized, V: ?Sized> {
    pub(crate) direct: &'d mut A,
    pub(crate) visit: &'d mut V,
}

impl<A: ?Sized, V: ?Sized> DirectorVisitor<'_, A, V> {
    pub(crate) fn reborrow(&mut self) -> DirectorVisitor<'_, A, V> {
        let Self {
            direct, 
            visit,
        } = self;
        
        DirectorVisitor {
            direct,
            visit,
        }
    }
}

pub fn visit<'dv, D, V, N>(direct: &'dv mut D, visit: &'dv mut V, node: &N) 
where 
    D: Direct<V, N> + ?Sized,
    V: Visit<N> + ?Sized,
    N: ?Sized,
{
    V::visit(
        Visitor::new(
            DirectorVisitor {
                direct,
                visit,
            },
            node
        ),
        node
    );
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
        let input = 
            A(Some( // 1
                B(vec![ // 3
                    C( // 6
                        A(None)) // 7
                    , 
                    C( // 10
                        A(Some( // 11
                            B(vec![]) // 13
                        ))
                    )
                ])
            ));

        let mut direct = MyDirect;
        let mut visit = MyVisit(0);
        crate::visit(&mut direct, &mut visit, &input);

        assert_eq!(visit.0, 13);
    }
}
