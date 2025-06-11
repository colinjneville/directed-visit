Implement the visitor pattern with interchangable implementations for both the visit algorithm and node object traversal. It is also possible to dynamically create temporary 'virtual' nodes during traversal.  

An object traversal is comprised of 3 parts:  
The input to traverse, which can start at any node type. This can be an external type too, there are no required derives or trait impls.  
The director navigates between node objects. It must implement `Direct<N>` for each node type `N` in the object graph, determining the sub-nodes.  
The visitor performs the desired algorithm, implementing `Visit<N>` for each node type `N` in the object graph.  

```rust
fn my_visit(input: &MyTree) -> usize {
    let mut my_director = MyDirector::new();
    let mut my_visitor = MyVisitor::new();
    directed_visit::visit(
        &mut my_director,
        &mut my_visitor,
        input,
    );

    my_visitor.result_value()
}
```

## syn
The crate includes a replacement for `syn::visit::Visit` if the `syn` feature is enabled. Implement `directed_visit::syn::visit::Full` as you would `syn::visit::Visit`.  
For your director, `directed_visit::syn::direct::FullDefault` traverses as `syn::visit` does, or you can customize the behavior by implementing `directed_visit::syn::direct::Full`.  
In addition to the existing syn AST, two nodes have been added to the tree to represent when generic parameters become in and out of scope.  
The `derive` feature subset of `full` is not yet supported.

## Limitations
Because the director can dynamically create new nodes to visit, the visitor cannot hold references to the node graph (i.e. there is no single `'ast` lifetime for all nodes). For this reason there is also currently no `VisitMut` equivalent, because the ideal interaction between handling temporary dynamic nodes and mutating them is unclear.  
