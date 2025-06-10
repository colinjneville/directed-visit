use crate::{Direct, Director, DirectorVisitor};

pub struct Visitor<'dv, 'n, D: ?Sized, V: ?Sized, N: ?Sized>(DirectorVisitor<'dv, D, V>, &'n N);

impl<'dv, 'n, D: ?Sized, V: ?Sized, N: ?Sized> Visitor<'dv, 'n, D, V, N> {
    pub(crate) fn new(data: crate::DirectorVisitor<'dv, D, V>, node: &'n N) -> Self {
        Self(data, node)
    }
    
    pub fn visit(this: Self) 
    where 
        D: Direct<'n, V, N>,
    {
        <D as Direct<'n, V, N>>::direct(Director::new(this.0), this.1)
    }
}

impl<'dv, 'n, D: ?Sized, V: ?Sized, N: ?Sized> std::ops::Deref for Visitor<'dv, 'n, D, V, N> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.0.visit
    }
}

impl<'dv, 'n, D: ?Sized, V: ?Sized, N: ?Sized> std::ops::DerefMut for Visitor<'dv, 'n, D, V, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.visit
    }
}

pub trait Visit<'n, N: ?Sized> {
    fn visit<'dv, D: ?Sized>(visitor: Visitor<'dv, 'n, D, Self, N>, _node: &'n N) 
    where
        D: Direct<'n, Self, N>,
    {
        Visitor::visit(visitor);
    }
}