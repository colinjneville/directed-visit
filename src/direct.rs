use crate::{DirectorVisitor, Visit, VisitMut, Visitor};

/// A wrapper for a [Direct] implementation.
#[derive(Debug)]
pub struct Director<'dv, D: ?Sized, V: ?Sized>(DirectorVisitor<'dv, D, V>);

impl<'dv, D: ?Sized, V: ?Sized> Director<'dv, D, V> {
    pub(crate) fn new(data: DirectorVisitor<'dv, D, V>) -> Self {
        Self(data)
    }

    /// Direct from this node to a sub-node with the [Visit] implementation.
    pub fn direct<NN: ?Sized>(this: &mut Self, node: &NN)
    where
        D: Direct<V, NN>,
        V: Visit<NN>,
    {
        <V as Visit<NN>>::visit(Visitor::new(this.0.reborrow()), node)
    }

    /// Direct from this node to a sub-node with the [VisitMut] implementation.
    pub fn direct_mut<NN: ?Sized>(this: &mut Self, node: &mut NN)
    where
        D: DirectMut<V, NN>,
        V: VisitMut<NN>,
    {
        <V as VisitMut<NN>>::visit_mut(Visitor::new(this.0.reborrow()), node)
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

/// Determines how to traverse the nodes within the input. This must be implemented for
/// all node types in the input.
pub trait Direct<V: ?Sized, N: ?Sized> {
    /// Determines all the sub-nodes of the given node. For each sub-node, call
    /// `Director::direct(&mut director, &node.my_subnode)`.
    fn direct(director: Director<'_, Self, V>, node: &N);
}

/// Determines how to traverse the nodes within the input. This must be implemented for
/// all node types in the input.
pub trait DirectMut<V: ?Sized, N: ?Sized> {
    /// Determines all the sub-nodes of the given node. For each sub-node, call
    /// `Director::direct_mut(&mut director, &mut node.my_subnode)`.
    fn direct_mut(director: Director<'_, Self, V>, node: &mut N);
}
