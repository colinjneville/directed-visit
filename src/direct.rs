use crate::{DirectorVisitor, Visit, Visitor};

pub struct Director<'dv, D: ?Sized, V: ?Sized>(DirectorVisitor<'dv, D, V>);

impl<'dv, D: ?Sized, V: ?Sized> Director<'dv, D, V> {
    pub(crate) fn new(data: DirectorVisitor<'dv, D, V>) -> Self {
        Self(data)
    }
    
    pub fn direct<'s, NN: ?Sized>(this: &mut Self, node: &'s NN)
    where
        D: Direct<'s, V, NN>,
        V: Visit<'s, NN>,
    {
        <V as Visit<'s, NN>>::visit(Visitor::new(this.0.reborrow(), node), node)
    }
}

impl<'dv, D: ?Sized, V: ?Sized> std::ops::Deref for Director<'dv, D, V> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.0.direct
    }
}

impl<'dv, D: ?Sized, V: ?Sized> std::ops::DerefMut for Director<'dv, D, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.direct
    }
}

pub trait Direct<'n, V: ?Sized, N: ?Sized> {
    fn direct<'dv>(wrapper: Director<'dv, Self, V>, node: &'n N);
}