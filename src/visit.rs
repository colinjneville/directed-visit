use crate::{Direct, Director, DirectorVisitor};

pub struct Visitor<'dv, 'n, D: ?Sized, V: ?Sized, N: ?Sized>(DirectorVisitor<'dv, D, V>, &'n N);

impl<'dv, 'n, D: ?Sized, V: ?Sized, N: ?Sized> Visitor<'dv, 'n, D, V, N> {
    pub(crate) fn new(data: crate::DirectorVisitor<'dv, D, V>, node: &'n N) -> Self {
        Self(data, node)
    }
    
    pub fn visit(this: Self) 
    where 
        D: Direct<V, N>,
    {
        <D as Direct<V, N>>::direct(Director::new(this.0), this.1)
    }
}

impl<D: ?Sized, V: ?Sized, N: ?Sized> std::ops::Deref for Visitor<'_, '_, D, V, N> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.0.visit
    }
}

impl<D: ?Sized, V: ?Sized, N: ?Sized> std::ops::DerefMut for Visitor<'_, '_, D, V, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.visit
    }
}

pub trait Visit<N: ?Sized> {
    fn visit<'n, D>(visitor: Visitor<'_, 'n, D, Self, N>, _node: &'n N) 
    where
        D: Direct<Self, N> + ?Sized,
    {
        Visitor::visit(visitor);
    }
}