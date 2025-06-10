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

impl<'v, A: ?Sized, V: ?Sized> DirectorVisitor<'v, A, V> {
    pub(crate) fn reborrow<'v2>(&'v2 mut self) -> DirectorVisitor<'v2, A, V> {
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

pub fn visit<'dv, 'n, D: ?Sized, V: ?Sized, N: ?Sized>(direct: &'dv mut D, visit: &'dv mut V, node: &'n N) 
where 
    D: Direct<'n, V, N>,
    V: Visit<'n, N>,
{
    Visitor::visit(
        Visitor::new(
            DirectorVisitor {
                direct,
                visit,
            },
            node
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
