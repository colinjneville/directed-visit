use crate::{Direct, DirectMut, Director, DirectorVisitor};

/// A wrapper for a [Visit] implementation.
#[derive(Debug)]
pub struct Visitor<'dv, D: ?Sized, V: ?Sized>(DirectorVisitor<'dv, D, V>);

impl<'dv, D: ?Sized, V: ?Sized> Visitor<'dv, D, V> {
    pub(crate) fn new(data: crate::DirectorVisitor<'dv, D, V>) -> Self {
        Self(data)
    }

    /// Complete visiting this node and return control to the [Direct] implementation.
    pub fn visit<N>(this: Self, node: &N)
    where
        D: Direct<V, N>,
        N: ?Sized,
    {
        <D as Direct<V, N>>::direct(Director::new(this.0), node)
    }

    /// Complete visiting this node and return control to the [DirectMut] implementation.
    pub fn visit_mut<N>(this: Self, node: &mut N)
    where
        D: DirectMut<V, N>,
        N: ?Sized,
    {
        <D as DirectMut<V, N>>::direct_mut(Director::new(this.0), node)
    }
}

impl<D: ?Sized, V: ?Sized> std::ops::Deref for Visitor<'_, D, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.0.visit
    }
}

impl<D: ?Sized, V: ?Sized> std::ops::DerefMut for Visitor<'_, D, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.visit
    }
}

/// The action to be performed when visiting a node of type `N`. This trait must be
/// implemented for all node types in the input, even if the visitor has no special handling
/// for that type.
pub trait Visit<N: ?Sized> {
    /// Performs the visiting action. The default implementation simply calls
    /// `Visitor::visit(visitor, node)`, which returns control to the [Direct] implementation
    /// to continue to further sub-nodes. If you wish to skip sub-nodes, simply omit this
    /// call from your implementation.
    fn visit<D>(visitor: Visitor<'_, D, Self>, node: &N)
    where
        D: Direct<Self, N> + ?Sized,
    {
        Visitor::visit(visitor, node);
    }
}

/// The action to be performed when visiting a node of type `N`. This trait must be
/// implemented for all node types in the input, even if the visitor has no special handling
/// for that type.
pub trait VisitMut<N: ?Sized> {
    /// Performs the visiting action. The default implementation simply calls
    /// `Visitor::visit_mut(visitor, node)`, which returns control to the [DirectMut] implementation
    /// to continue to further sub-nodes. If you wish to skip sub-nodes, simply omit this
    /// call from your implementation.
    fn visit_mut<D>(visitor: Visitor<'_, D, Self>, node: &mut N)
    where
        D: DirectMut<Self, N> + ?Sized,
    {
        Visitor::visit_mut(visitor, node);
    }
}
