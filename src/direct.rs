use crate::{DirectorVisitor, Visit, Visitor};

pub struct Director<'dv, D: ?Sized, V: ?Sized>(DirectorVisitor<'dv, D, V>);

impl<'dv, D: ?Sized, V: ?Sized> Director<'dv, D, V> {
    pub(crate) fn new(data: DirectorVisitor<'dv, D, V>) -> Self {
        Self(data)
    }
    
    pub fn direct<NN: ?Sized>(this: &mut Self, node: &NN)
    where
        D: Direct<V, NN>,
        V: Visit<NN>,
    {
        <V as Visit<NN>>::visit(Visitor::new(this.0.reborrow(), node), node)
    }
}

impl<D: ?Sized, V: ?Sized> std::ops::Deref for Director<'_, D, V> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.0.direct
    }
}

impl<D: ?Sized, V: ?Sized> std::ops::DerefMut for Director<'_, D, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.direct
    }
}

pub trait Direct<V: ?Sized, N: ?Sized> {
    fn direct(director: Director<'_, Self, V>, node: &N);
}