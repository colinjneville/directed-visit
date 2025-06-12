/// A [crate::Direct] implementation equivalent to [syn]::visit's ast traversal.
#[derive(Debug)]
pub struct FullDefault;

impl<V> Full<V> for FullDefault where V: crate::syn::visit::Full + ?Sized {}

impl<V> FullMut<V> for FullDefault where V: crate::syn::visit::FullMut + ?Sized {}

macro_rules! node_set {
    ($(#[$attr:meta])* $vis:vis trait $trait_ident:ident { $($fn_ident:ident($director_ident:ident, $node_ident:ident) -> $ty:ty $fn_impl:block)* }) => {
        #[allow(unused_mut)]
        $(#[$attr])*
        $vis trait $trait_ident <V>
        where
            V: crate::syn::visit::Full + ?Sized,
        {
            $(
                #[doc = concat!("Direct to sub-nodes of [", stringify!($ty), "]")]
                fn $fn_ident(mut $director_ident: crate::Director<'_, Self, V>, $node_ident: &$ty)
                {
                    default::$fn_ident(&mut $director_ident, $node_ident);
                }
            )*
        }

        $(
            impl<V, T> crate::Direct<V, $ty> for T
            where
                V: crate::syn::visit::Full + ?Sized,
                T: Full<V> + ?Sized,
            {
                fn direct(director: crate::Director<'_, Self, V>, node: &$ty) {
                    Self::$fn_ident(director, node);
                }
            }
        )*

        /// Free-standing direct impls
        pub mod default {
            $(
                #[doc = concat!("Free-standing ", stringify!($ty), " direct impl")]
                pub fn $fn_ident<D, V>($director_ident: &mut crate::Director<'_, D, V>, $node_ident: &$ty)
                where
                    D: super::$trait_ident<V> + ?Sized,
                    V: crate::syn::visit::Full + ?Sized,
                {
                    $fn_impl
                }
            )*
        }
    }
}

macro_rules! node_set_mut {
    ($(#[$attr:meta])* $vis:vis trait $trait_ident:ident { $($fn_ident:ident($director_ident:ident, $node_ident:ident) -> $ty:ty $fn_impl:block)* }) => {
        #[allow(unused_mut)]
        $(#[$attr])*
        $vis trait $trait_ident <V>
        where
            V: crate::syn::visit::FullMut + ?Sized,
        {
            $(
                #[doc = concat!("Direct to sub-nodes of [", stringify!($ty), "]")]
                fn $fn_ident(mut $director_ident: crate::Director<'_, Self, V>, $node_ident: &mut $ty)
                {
                    default_mut::$fn_ident(&mut $director_ident, $node_ident);
                }
            )*
        }

        $(
            impl<V, T> crate::DirectMut<V, $ty> for T
            where
                V: crate::syn::visit::FullMut + ?Sized,
                T: FullMut<V> + ?Sized,
            {
                fn direct_mut(director: crate::Director<'_, Self, V>, node: &mut $ty) {
                    Self::$fn_ident(director, node);
                }
            }
        )*

        /// Free-standing direct impls
        pub mod default_mut {
            $(
                #[doc = concat!("Free-standing ", stringify!($ty), " direct impl")]
                pub fn $fn_ident<D, V>($director_ident: &mut crate::Director<'_, D, V>, $node_ident: &mut $ty)
                where
                    D: super::$trait_ident<V> + ?Sized,
                    V: crate::syn::visit::FullMut + ?Sized,
                {
                    $fn_impl
                }
            )*
        }
    }
}

macro_rules! full {
    ($e:expr) => {
        $e
    };
}

macro_rules! skip {
    ($($tt:tt)*) => {
        let _ = $($tt)*;
    };
}

node_set! {
    /// A convenience trait for creating [syn] directors. Implementing this trait will also implement [crate::Direct] for
    /// all 'feature = "full"' syn ast types. Each impl has a default corresponding to [syn]'s traversal behavior.
    pub trait Full {
        direct_abi(director, node) -> syn::Abi {
            skip!(node.extern_token);
            if let Some(it) = &node.name {
                crate::Director::direct(director, it);
            }
        }
        direct_angle_bracketed_generic_arguments(director, node) -> syn::AngleBracketedGenericArguments {
            skip!(node.colon2_token);
            skip!(node.lt_token);
            for el in syn::punctuated::Punctuated::pairs(&node.args) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            skip!(node.gt_token);
        }
        direct_arm(director, node) -> syn::Arm {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.pat);
            if let Some(it) = &node.guard {
                skip!((it).0);
                crate::Director::direct(director, &*(it).1);
            }
            skip!(node.fat_arrow_token);
            crate::Director::direct(director, &*node.body);
            skip!(node.comma);
        }
        direct_assoc_const(director, node) -> syn::AssocConst {
            crate::Director::direct(director, &node.ident);
            if let Some(it) = &node.generics {
                crate::Director::direct(director, it);
            }
            skip!(node.eq_token);
            crate::Director::direct(director, &node.value);
        }
        direct_assoc_type(director, node) -> syn::AssocType {
            crate::Director::direct(director, &node.ident);
            if let Some(it) = &node.generics {
                crate::Director::direct(director, it);
            }
            skip!(node.eq_token);
            crate::Director::direct(director, &node.ty);
        }
        direct_attr_style(_director, node) -> syn::AttrStyle {
            match node {
                syn::AttrStyle::Outer => {}
                syn::AttrStyle::Inner(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        direct_attribute(director, node) -> syn::Attribute {
            skip!(node.pound_token);
            crate::Director::direct(director, &node.style);
            skip!(node.bracket_token);
            crate::Director::direct(director, &node.meta);
        }
        direct_bare_fn_arg(director, node) -> syn::BareFnArg {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.name {
                crate::Director::direct(director, &(it).0);
                skip!((it).1);
            }
            crate::Director::direct(director, &node.ty);
        }
        direct_bare_variadic(director, node) -> syn::BareVariadic {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.name {
                crate::Director::direct(director, &(it).0);
                skip!((it).1);
            }
            skip!(node.dots);
            skip!(node.comma);
        }
        direct_bin_op(_director, node) -> syn::BinOp {
            match node {
                syn::BinOp::Add(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Sub(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Mul(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Div(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Rem(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::And(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Or(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitXor(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitAnd(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitOr(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Shl(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Shr(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Eq(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Lt(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Le(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Ne(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Ge(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Gt(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::AddAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::SubAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::MulAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::DivAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::RemAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitXorAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitAndAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitOrAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::ShlAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::ShrAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                _ => { }
            }
        }
        direct_block(director, node) -> syn::Block {
            skip!(node.brace_token);
            for it in &node.stmts {
                crate::Director::direct(director, it);
            }
        }
        direct_bound_lifetimes(director, node) -> syn::BoundLifetimes {
            skip!(node.for_token);
            skip!(node.lt_token);
            for el in syn::punctuated::Punctuated::pairs(&node.lifetimes) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            skip!(node.gt_token);
        }
        direct_captured_param(director, node) -> syn::CapturedParam {
            match node {
                syn::CapturedParam::Lifetime(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::CapturedParam::Ident(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_const_param(director, node) -> syn::ConstParam {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.const_token);
            crate::Director::direct(director, &node.ident);
            skip!(node.colon_token);
            crate::Director::direct(director, &node.ty);
            skip!(node.eq_token);
            if let Some(it) = &node.default {
                crate::Director::direct(director, it);
            }
        }
        direct_constraint(director, node) -> syn::Constraint {
            crate::Director::direct(director, &node.ident);
            if let Some(it) = &node.generics {
                crate::Director::direct(director, it);
            }
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_data(director, node) -> syn::Data {
            match node {
                syn::Data::Struct(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Data::Enum(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Data::Union(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
            }
        }
        direct_data_enum(director, node) -> syn::DataEnum {
            skip!(node.enum_token);
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.variants) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_data_struct(director, node) -> syn::DataStruct {
            skip!(node.struct_token);
            crate::Director::direct(director, &node.fields);
            skip!(node.semi_token);
        }
        direct_data_union(director, node) -> syn::DataUnion {
            skip!(node.union_token);
            crate::Director::direct(director, &node.fields);
        }
        direct_derive_input(director, node) -> syn::DeriveInput {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            crate::Director::direct(director, &node.data);
            super::exit_scope(director, &node.generics.params);
        }
        direct_expr(director, node) -> syn::Expr {
            match node {
                syn::Expr::Array(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Assign(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Async(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Await(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Binary(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Block(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Break(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Call(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Cast(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Closure(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Const(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Continue(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Field(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::ForLoop(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Group(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::If(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Index(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Infer(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Let(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Lit(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Loop(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Macro(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Match(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::MethodCall(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Paren(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Path(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Range(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::RawAddr(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Reference(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Repeat(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Return(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Struct(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Try(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::TryBlock(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Tuple(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Unary(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::Unsafe(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Verbatim(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Expr::While(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::Expr::Yield(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                _ => { }
            }
        }
        direct_expr_array(director, node) -> syn::ExprArray {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.bracket_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_expr_assign(director, node) -> syn::ExprAssign {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &*node.left);
            skip!(node.eq_token);
            crate::Director::direct(director, &*node.right);
        }
        direct_expr_async(director, node) -> syn::ExprAsync {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.async_token);
            skip!(node.capture);
            crate::Director::direct(director, &node.block);
        }
        direct_expr_await(director, node) -> syn::ExprAwait {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &*node.base);
            skip!(node.dot_token);
            skip!(node.await_token);
        }
        direct_expr_binary(director, node) -> syn::ExprBinary {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &*node.left);
            crate::Director::direct(director, &node.op);
            crate::Director::direct(director, &*node.right);
        }
        direct_expr_block(director, node) -> syn::ExprBlock {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.label {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.block);
        }
        direct_expr_break(director, node) -> syn::ExprBreak {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.break_token);
            if let Some(it) = &node.label {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.expr {
                crate::Director::direct(director, &**it);
            }
        }
        direct_expr_call(director, node) -> syn::ExprCall {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &*node.func);
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.args) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_expr_cast(director, node) -> syn::ExprCast {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &*node.expr);
            skip!(node.as_token);
            crate::Director::direct(director, &*node.ty);
        }
        direct_expr_closure(director, node) -> syn::ExprClosure {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.lifetimes {
                crate::Director::direct(director, it);
            }
            skip!(node.constness);
            skip!(node.movability);
            skip!(node.asyncness);
            skip!(node.capture);
            skip!(node.or1_token);
            for el in syn::punctuated::Punctuated::pairs(&node.inputs) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            skip!(node.or2_token);
            crate::Director::direct(director, &node.output);
            crate::Director::direct(director, &*node.body);
        }
        direct_expr_const(director, node) -> syn::ExprConst {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.const_token);
            crate::Director::direct(director, &node.block);
        }
        direct_expr_continue(director, node) -> syn::ExprContinue {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.continue_token);
            if let Some(it) = &node.label {
                crate::Director::direct(director, it);
            }
        }
        direct_expr_field(director, node) -> syn::ExprField {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &*node.base);
            skip!(node.dot_token);
            crate::Director::direct(director, &node.member);
        }
        direct_expr_for_loop(director, node) -> syn::ExprForLoop {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.label {
                crate::Director::direct(director, it);
            }
            skip!(node.for_token);
            crate::Director::direct(director, &*node.pat);
            skip!(node.in_token);
            crate::Director::direct(director, &*node.expr);
            crate::Director::direct(director, &node.body);
        }
        direct_expr_group(director, node) -> syn::ExprGroup {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.group_token);
            crate::Director::direct(director, &*node.expr);
        }
        direct_expr_if(director, node) -> syn::ExprIf {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.if_token);
            crate::Director::direct(director, &*node.cond);
            crate::Director::direct(director, &node.then_branch);
            if let Some(it) = &node.else_branch {
                skip!((it).0);
                crate::Director::direct(director, &*(it).1);
            }
        }
        direct_expr_index(director, node) -> syn::ExprIndex {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &*node.expr);
            skip!(node.bracket_token);
            crate::Director::direct(director, &*node.index);
        }
        direct_expr_infer(director, node) -> syn::ExprInfer {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.underscore_token);
        }
        direct_expr_let(director, node) -> syn::ExprLet {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.let_token);
            crate::Director::direct(director, &*node.pat);
            skip!(node.eq_token);
            crate::Director::direct(director, &*node.expr);
        }
        direct_expr_lit(director, node) -> syn::ExprLit {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.lit);
        }
        direct_expr_loop(director, node) -> syn::ExprLoop {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.label {
                crate::Director::direct(director, it);
            }
            skip!(node.loop_token);
            crate::Director::direct(director, &node.body);
        }
        direct_expr_macro(director, node) -> syn::ExprMacro {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.mac);
        }
        direct_expr_match(director, node) -> syn::ExprMatch {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.match_token);
            crate::Director::direct(director, &*node.expr);
            skip!(node.brace_token);
            for it in &node.arms {
                crate::Director::direct(director, it);
            }
        }
        direct_expr_method_call(director, node) -> syn::ExprMethodCall {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &*node.receiver);
            skip!(node.dot_token);
            crate::Director::direct(director, &node.method);
            if let Some(it) = &node.turbofish {
                crate::Director::direct(director, it);
            }
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.args) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_expr_paren(director, node) -> syn::ExprParen {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.paren_token);
            crate::Director::direct(director, &*node.expr);
        }
        direct_expr_path(director, node) -> syn::ExprPath {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.qself {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.path);
        }
        direct_expr_range(director, node) -> syn::ExprRange {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.start {
                crate::Director::direct(director, &**it);
            }
            crate::Director::direct(director, &node.limits);
            if let Some(it) = &node.end {
                crate::Director::direct(director, &**it);
            }
        }
        direct_expr_raw_addr(director, node) -> syn::ExprRawAddr {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.and_token);
            skip!(node.raw);
            crate::Director::direct(director, &node.mutability);
            crate::Director::direct(director, &*node.expr);
        }
        direct_expr_reference(director, node) -> syn::ExprReference {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.and_token);
            skip!(node.mutability);
            crate::Director::direct(director, &*node.expr);
        }
        direct_expr_repeat(director, node) -> syn::ExprRepeat {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.bracket_token);
            crate::Director::direct(director, &*node.expr);
            skip!(node.semi_token);
            crate::Director::direct(director, &*node.len);
        }
        direct_expr_return(director, node) -> syn::ExprReturn {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.return_token);
            if let Some(it) = &node.expr {
                crate::Director::direct(director, &**it);
            }
        }
        direct_expr_struct(director, node) -> syn::ExprStruct {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.qself {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.path);
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.fields) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            skip!(node.dot2_token);
            if let Some(it) = &node.rest {
                crate::Director::direct(director, &**it);
            }
        }
        direct_expr_try(director, node) -> syn::ExprTry {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &*node.expr);
            skip!(node.question_token);
        }
        direct_expr_try_block(director, node) -> syn::ExprTryBlock {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.try_token);
            crate::Director::direct(director, &node.block);
        }
        direct_expr_tuple(director, node) -> syn::ExprTuple {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_expr_unary(director, node) -> syn::ExprUnary {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.op);
            crate::Director::direct(director, &*node.expr);
        }
        direct_expr_unsafe(director, node) -> syn::ExprUnsafe {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.unsafe_token);
            crate::Director::direct(director, &node.block);
        }
        direct_expr_while(director, node) -> syn::ExprWhile {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.label {
                crate::Director::direct(director, it);
            }
            skip!(node.while_token);
            crate::Director::direct(director, &*node.cond);
            crate::Director::direct(director, &node.body);
        }
        direct_expr_yield(director, node) -> syn::ExprYield {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.yield_token);
            if let Some(it) = &node.expr {
                crate::Director::direct(director, &**it);
            }
        }
        direct_field(director, node) -> syn::Field {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            crate::Director::direct(director, &node.mutability);
            if let Some(it) = &node.ident {
                crate::Director::direct(director, it);
            }
            skip!(node.colon_token);
            crate::Director::direct(director, &node.ty);
        }
        direct_field_mutability(_director, _node) -> syn::FieldMutability {

        }
        direct_field_pat(director, node) -> syn::FieldPat {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.member);
            skip!(node.colon_token);
            crate::Director::direct(director, &*node.pat);
        }
        direct_field_value(director, node) -> syn::FieldValue {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.member);
            skip!(node.colon_token);
            crate::Director::direct(director, &node.expr);
        }
        direct_fields(director, node) -> syn::Fields {
            match node {
                syn::Fields::Named(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Fields::Unnamed(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Fields::Unit => {}
            }
        }
        direct_fields_named(director, node) -> syn::FieldsNamed {
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.named) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_fields_unnamed(director, node) -> syn::FieldsUnnamed {
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.unnamed) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_file(director, node) -> syn::File {
            skip!(node.shebang);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            for it in &node.items {
                crate::Director::direct(director, it);
            }
        }
        direct_fn_arg(director, node) -> syn::FnArg {
            match node {
                syn::FnArg::Receiver(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::FnArg::Typed(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
            }
        }
        direct_foreign_item(director, node) -> syn::ForeignItem {
            match node {
                syn::ForeignItem::Fn(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::ForeignItem::Static(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::ForeignItem::Type(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::ForeignItem::Macro(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::ForeignItem::Verbatim(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_foreign_item_fn(director, node) -> syn::ForeignItemFn {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            crate::Director::direct(director, &node.sig);
            skip!(node.semi_token);
        }
        direct_foreign_item_macro(director, node) -> syn::ForeignItemMacro {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.mac);
            skip!(node.semi_token);
        }
        direct_foreign_item_static(director, node) -> syn::ForeignItemStatic {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.static_token);
            crate::Director::direct(director, &node.mutability);
            crate::Director::direct(director, &node.ident);
            skip!(node.colon_token);
            crate::Director::direct(director, &*node.ty);
            skip!(node.semi_token);
        }
        direct_foreign_item_type(director, node) -> syn::ForeignItemType {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.type_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.semi_token);
            super::exit_scope(director, &node.generics.params);
        }
        direct_generic_argument(director, node) -> syn::GenericArgument {
            match node {
                syn::GenericArgument::Lifetime(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::GenericArgument::Type(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::GenericArgument::Const(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::GenericArgument::AssocType(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::GenericArgument::AssocConst(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::GenericArgument::Constraint(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_generic_param(director, node) -> syn::GenericParam {
            match node {
                syn::GenericParam::Lifetime(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::GenericParam::Type(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::GenericParam::Const(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
            }
        }
        direct_generics(director, node) -> syn::Generics {
            skip!(node.lt_token);
            for el in syn::punctuated::Punctuated::pairs(&node.params) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            skip!(node.gt_token);
            if let Some(it) = &node.where_clause {
                crate::Director::direct(director, it);
            }
        }
        direct_ident(_director, node) -> proc_macro2::Ident {
            skip!(node.span());
        }
        direct_impl_item(director, node) -> syn::ImplItem {
            match node {
                syn::ImplItem::Const(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::ImplItem::Fn(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::ImplItem::Type(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::ImplItem::Macro(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::ImplItem::Verbatim(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_impl_item_const(director, node) -> syn::ImplItemConst {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.defaultness);
            skip!(node.const_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.colon_token);
            crate::Director::direct(director, &node.ty);
            skip!(node.eq_token);
            crate::Director::direct(director, &node.expr);
            skip!(node.semi_token);
            super::exit_scope(director, &node.generics.params);
        }
        direct_impl_item_fn(director, node) -> syn::ImplItemFn {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.defaultness);
            crate::Director::direct(director, &node.sig);
            crate::Director::direct(director, &node.block);
        }
        direct_impl_item_macro(director, node) -> syn::ImplItemMacro {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.mac);
            skip!(node.semi_token);
        }
        direct_impl_item_type(director, node) -> syn::ImplItemType {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.defaultness);
            skip!(node.type_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.eq_token);
            crate::Director::direct(director, &node.ty);
            skip!(node.semi_token);
            super::exit_scope(director, &node.generics.params);
        }
        direct_impl_restriction(_director, _node) -> syn::ImplRestriction {

        }
        direct_index(_director, node) -> syn::Index {
            skip!(node.index);
            skip!(node.span);
        }
        direct_item(director, node) -> syn::Item {
            match node {
                syn::Item::Const(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Enum(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::ExternCrate(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Fn(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::ForeignMod(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Impl(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Macro(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Mod(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Static(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Struct(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Trait(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::TraitAlias(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Type(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Union(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Use(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Item::Verbatim(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_item_const(director, node) -> syn::ItemConst {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.const_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.colon_token);
            crate::Director::direct(director, &*node.ty);
            skip!(node.eq_token);
            crate::Director::direct(director, &*node.expr);
            skip!(node.semi_token);
            super::exit_scope(director, &node.generics.params);
        }
        direct_item_enum(director, node) -> syn::ItemEnum {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.enum_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.variants) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            super::exit_scope(director, &node.generics.params);
        }
        direct_item_extern_crate(director, node) -> syn::ItemExternCrate {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.extern_token);
            skip!(node.crate_token);
            crate::Director::direct(director, &node.ident);
            if let Some(it) = &node.rename {
                skip!((it).0);
                crate::Director::direct(director, &(it).1);
            }
            skip!(node.semi_token);
        }
        direct_item_fn(director, node) -> syn::ItemFn {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            crate::Director::direct(director, &node.sig);
            crate::Director::direct(director, &*node.block);
        }
        direct_item_foreign_mod(director, node) -> syn::ItemForeignMod {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.unsafety);
            crate::Director::direct(director, &node.abi);
            skip!(node.brace_token);
            for it in &node.items {
                crate::Director::direct(director, it);
            }
        }
        direct_item_impl(director, node) -> syn::ItemImpl {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.defaultness);
            skip!(node.unsafety);
            skip!(node.impl_token);
            crate::Director::direct(director, &node.generics);
            if let Some(it) = &node.trait_ {
                skip!((it).0);
                crate::Director::direct(director, &(it).1);
                skip!((it).2);
            }
            crate::Director::direct(director, &*node.self_ty);
            skip!(node.brace_token);
            for it in &node.items {
                crate::Director::direct(director, it);
            }
            super::exit_scope(director, &node.generics.params);
        }
        direct_item_macro(director, node) -> syn::ItemMacro {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.ident {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.mac);
            skip!(node.semi_token);
        }
        direct_item_mod(director, node) -> syn::ItemMod {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.unsafety);
            skip!(node.mod_token);
            crate::Director::direct(director, &node.ident);
            if let Some(it) = &node.content {
                skip!((it).0);
                for it in &(it).1 {
                    crate::Director::direct(director, it);
                }
            }
            skip!(node.semi);
        }
        direct_item_static(director, node) -> syn::ItemStatic {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.static_token);
            crate::Director::direct(director, &node.mutability);
            crate::Director::direct(director, &node.ident);
            skip!(node.colon_token);
            crate::Director::direct(director, &*node.ty);
            skip!(node.eq_token);
            crate::Director::direct(director, &*node.expr);
            skip!(node.semi_token);
        }
        direct_item_struct(director, node) -> syn::ItemStruct {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.struct_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            crate::Director::direct(director, &node.fields);
            skip!(node.semi_token);
            super::exit_scope(director, &node.generics.params);
        }
        direct_item_trait(director, node) -> syn::ItemTrait {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.unsafety);
            skip!(node.auto_token);
            if let Some(it) = &node.restriction {
                crate::Director::direct(director, it);
            }
            skip!(node.trait_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.supertraits) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            skip!(node.brace_token);
            for it in &node.items {
                crate::Director::direct(director, it);
            }
            super::exit_scope(director, &node.generics.params);
        }
        direct_item_trait_alias(director, node) -> syn::ItemTraitAlias {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.trait_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.eq_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            skip!(node.semi_token);
            super::exit_scope(director, &node.generics.params);
        }
        direct_item_type(director, node) -> syn::ItemType {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.type_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.eq_token);
            crate::Director::direct(director, &*node.ty);
            skip!(node.semi_token);
            super::exit_scope(director, &node.generics.params);
        }
        direct_item_union(director, node) -> syn::ItemUnion {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.union_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            crate::Director::direct(director, &node.fields);
            super::exit_scope(director, &node.generics.params);
        }
        direct_item_use(director, node) -> syn::ItemUse {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.vis);
            skip!(node.use_token);
            skip!(node.leading_colon);
            crate::Director::direct(director, &node.tree);
            skip!(node.semi_token);
        }
        direct_label(director, node) -> syn::Label {
            crate::Director::direct(director, &node.name);
            skip!(node.colon_token);
        }
        direct_lifetime(director, node) -> syn::Lifetime {
            skip!(node.apostrophe);
            crate::Director::direct(director, &node.ident);
        }
        direct_lifetime_param(director, node) -> syn::LifetimeParam {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.lifetime);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_lit(director, node) -> syn::Lit {
            match node {
                syn::Lit::Str(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Lit::ByteStr(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Lit::CStr(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Lit::Byte(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Lit::Char(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Lit::Int(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Lit::Float(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Lit::Bool(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Lit::Verbatim(_binding_0) => {
                    skip!(_binding_0);
                }
                _ => { }
            }
        }
        direct_lit_bool(_director, node) -> syn::LitBool {
            skip!(node.value);
            skip!(node.span);
        }
        direct_lit_byte(_director, _node) -> syn::LitByte {

        }
        direct_lit_byte_str(_director, _node) -> syn::LitByteStr {

        }
        direct_lit_cstr(_director, _node) -> syn::LitCStr {

        }
        direct_lit_char(_director, _node) -> syn::LitChar {

        }
        direct_lit_float(_director, _node) -> syn::LitFloat {

        }
        direct_lit_int(_director, _node) -> syn::LitInt {

        }
        direct_lit_str(_director, _node) -> syn::LitStr {

        }
        direct_local(director, node) -> syn::Local {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.let_token);
            crate::Director::direct(director, &node.pat);
            if let Some(it) = &node.init {
                crate::Director::direct(director, it);
            }
            skip!(node.semi_token);
        }
        direct_local_init(director, node) -> syn::LocalInit {
            skip!(node.eq_token);
            crate::Director::direct(director, &*node.expr);
            if let Some(it) = &node.diverge {
                skip!((it).0);
                crate::Director::direct(director, &*(it).1);
            }
        }
        direct_macro(director, node) -> syn::Macro {
            crate::Director::direct(director, &node.path);
            skip!(node.bang_token);
            crate::Director::direct(director, &node.delimiter);
            crate::Director::direct(director, &node.tokens);
        }
        direct_macro_delimiter(_director, node) -> syn::MacroDelimiter {
            match node {
                syn::MacroDelimiter::Paren(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::MacroDelimiter::Brace(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::MacroDelimiter::Bracket(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        direct_member(director, node) -> syn::Member {
            match node {
                syn::Member::Named(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Member::Unnamed(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
            }
        }
        direct_meta(director, node) -> syn::Meta {
            match node {
                syn::Meta::Path(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Meta::List(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Meta::NameValue(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
            }
        }
        direct_meta_list(director, node) -> syn::MetaList {
            crate::Director::direct(director, &node.path);
            crate::Director::direct(director, &node.delimiter);
            crate::Director::direct(director, &node.tokens);
        }
        direct_meta_name_value(director, node) -> syn::MetaNameValue {
            crate::Director::direct(director, &node.path);
            skip!(node.eq_token);
            crate::Director::direct(director, &node.value);
        }
        direct_parenthesized_generic_arguments(director, node) -> syn::ParenthesizedGenericArguments {
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.inputs) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            crate::Director::direct(director, &node.output);
        }
        direct_pat(director, node) -> syn::Pat {
            match node {
                syn::Pat::Const(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Ident(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Lit(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Macro(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Or(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Paren(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Path(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Range(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Reference(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Rest(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Slice(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Struct(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Tuple(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::TupleStruct(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Type(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Verbatim(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Pat::Wild(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_pat_ident(director, node) -> syn::PatIdent {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.by_ref);
            skip!(node.mutability);
            crate::Director::direct(director, &node.ident);
            if let Some(it) = &node.subpat {
                skip!((it).0);
                crate::Director::direct(director, &*(it).1);
            }
        }
        direct_pat_or(director, node) -> syn::PatOr {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.leading_vert);
            for el in syn::punctuated::Punctuated::pairs(&node.cases) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_pat_paren(director, node) -> syn::PatParen {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.paren_token);
            crate::Director::direct(director, &*node.pat);
        }
        direct_pat_reference(director, node) -> syn::PatReference {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.and_token);
            skip!(node.mutability);
            crate::Director::direct(director, &*node.pat);
        }
        direct_pat_rest(director, node) -> syn::PatRest {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.dot2_token);
        }
        direct_pat_slice(director, node) -> syn::PatSlice {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.bracket_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_pat_struct(director, node) -> syn::PatStruct {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.qself {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.path);
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.fields) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            if let Some(it) = &node.rest {
                crate::Director::direct(director, it);
            }
        }
        direct_pat_tuple(director, node) -> syn::PatTuple {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_pat_tuple_struct(director, node) -> syn::PatTupleStruct {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.qself {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.path);
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_pat_type(director, node) -> syn::PatType {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &*node.pat);
            skip!(node.colon_token);
            crate::Director::direct(director, &*node.ty);
        }
        direct_pat_wild(director, node) -> syn::PatWild {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.underscore_token);
        }
        direct_path(director, node) -> syn::Path {
            skip!(node.leading_colon);
            for el in syn::punctuated::Punctuated::pairs(&node.segments) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_path_arguments(director, node) -> syn::PathArguments {
            match node {
                syn::PathArguments::None => {}
                syn::PathArguments::AngleBracketed(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::PathArguments::Parenthesized(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
            }
        }
        direct_path_segment(director, node) -> syn::PathSegment {
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.arguments);
        }
        direct_pointer_mutability(_director, node) -> syn::PointerMutability {
            match node {
                syn::PointerMutability::Const(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::PointerMutability::Mut(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        direct_precise_capture(director, node) -> syn::PreciseCapture {
            skip!(node.use_token);
            skip!(node.lt_token);
            for el in syn::punctuated::Punctuated::pairs(&node.params) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            skip!(node.gt_token);
        }
        direct_predicate_lifetime(director, node) -> syn::PredicateLifetime {
            crate::Director::direct(director, &node.lifetime);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_predicate_type(director, node) -> syn::PredicateType {
            if let Some(it) = &node.lifetimes {
                super::enter_scope(director, &it.lifetimes);
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.bounded_ty);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            if let Some(it) = &node.lifetimes {
                super::exit_scope(director, &it.lifetimes);
            }
        }
        direct_qself(director, node) -> syn::QSelf {
            skip!(node.lt_token);
            crate::Director::direct(director, &*node.ty);
            skip!(node.position);
            skip!(node.as_token);
            skip!(node.gt_token);
        }
        direct_range_limits(_director, node) -> syn::RangeLimits {
            match node {
                syn::RangeLimits::HalfOpen(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::RangeLimits::Closed(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        direct_receiver(director, node) -> syn::Receiver {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.reference {
                skip!((it).0);
                if let Some(it) = &(it).1 {
                    crate::Director::direct(director, it);
                }
            }
            skip!(node.mutability);
            skip!(node.self_token);
            skip!(node.colon_token);
            crate::Director::direct(director, &*node.ty);
        }
        direct_return_type(director, node) -> syn::ReturnType {
            match node {
                syn::ReturnType::Default => {}
                syn::ReturnType::Type(_binding_0, _binding_1) => {
                    skip!(_binding_0);
                    crate::Director::direct(director, &**_binding_1);
                }
            }
        }
        direct_signature(director, node) -> syn::Signature {
            super::enter_scope(director, &node.generics.params);
            skip!(node.constness);
            skip!(node.asyncness);
            skip!(node.unsafety);
            if let Some(it) = &node.abi {
                crate::Director::direct(director, it);
            }
            skip!(node.fn_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.inputs) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            if let Some(it) = &node.variadic {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.output);
            super::exit_scope(director, &node.generics.params);
        }
        direct_static_mutability(_director, node) -> syn::StaticMutability {
            match node {
                syn::StaticMutability::Mut(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::StaticMutability::None => {}
                _ => { }
            }
        }
        direct_stmt(director, node) -> syn::Stmt {
            match node {
                syn::Stmt::Local(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Stmt::Item(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Stmt::Expr(_binding_0, _binding_1) => {
                    crate::Director::direct(director, _binding_0);
                    skip!(_binding_1);
                }
                syn::Stmt::Macro(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
            }
        }
        direct_stmt_macro(director, node) -> syn::StmtMacro {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.mac);
            skip!(node.semi_token);
        }
        direct_token_stream(_director, _node) -> proc_macro2::TokenStream {

        }
        direct_trait_bound(director, node) -> syn::TraitBound {
            skip!(node.paren_token);
            crate::Director::direct(director, &node.modifier);
            if let Some(it) = &node.lifetimes {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.path);
        }
        direct_trait_bound_modifier(_director, node) -> syn::TraitBoundModifier {
            match node {
                syn::TraitBoundModifier::None => {}
                syn::TraitBoundModifier::Maybe(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        direct_trait_item(director, node) -> syn::TraitItem {
            match node {
                syn::TraitItem::Const(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::TraitItem::Fn(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::TraitItem::Type(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::TraitItem::Macro(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::TraitItem::Verbatim(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_trait_item_const(director, node) -> syn::TraitItemConst {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.const_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.colon_token);
            crate::Director::direct(director, &node.ty);
            if let Some(it) = &node.default {
                skip!((it).0);
                crate::Director::direct(director, &(it).1);
            }
            skip!(node.semi_token);
            super::exit_scope(director, &node.generics.params);
        }
        direct_trait_item_fn(director, node) -> syn::TraitItemFn {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.sig);
            if let Some(it) = &node.default {
                crate::Director::direct(director, it);
            }
            skip!(node.semi_token);
        }
        direct_trait_item_macro(director, node) -> syn::TraitItemMacro {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.mac);
            skip!(node.semi_token);
        }
        direct_trait_item_type(director, node) -> syn::TraitItemType {
            super::enter_scope(director, &node.generics.params);
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            skip!(node.type_token);
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.generics);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            if let Some(it) = &node.default {
                skip!((it).0);
                crate::Director::direct(director, &(it).1);
            }
            skip!(node.semi_token);
            super::exit_scope(director, &node.generics.params);
        }
        direct_type(director, node) -> syn::Type {
            match node {
                syn::Type::Array(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::BareFn(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Group(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::ImplTrait(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Infer(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Macro(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Never(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Paren(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Path(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Ptr(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Reference(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Slice(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::TraitObject(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Tuple(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Type::Verbatim(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_type_array(director, node) -> syn::TypeArray {
            skip!(node.bracket_token);
            crate::Director::direct(director, &*node.elem);
            skip!(node.semi_token);
            crate::Director::direct(director, &node.len);
        }
        direct_type_bare_fn(director, node) -> syn::TypeBareFn {
            if let Some(it) = &node.lifetimes {
                crate::Director::direct(director, it);
            }
            skip!(node.unsafety);
            if let Some(it) = &node.abi {
                crate::Director::direct(director, it);
            }
            skip!(node.fn_token);
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.inputs) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            if let Some(it) = &node.variadic {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.output);
        }
        direct_type_group(director, node) -> syn::TypeGroup {
            skip!(node.group_token);
            crate::Director::direct(director, &*node.elem);
        }
        direct_type_impl_trait(director, node) -> syn::TypeImplTrait {
            skip!(node.impl_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_type_infer(_director, node) -> syn::TypeInfer {
            skip!(node.underscore_token);
        }
        direct_type_macro(director, node) -> syn::TypeMacro {
            crate::Director::direct(director, &node.mac);
        }
        direct_type_never(_director, node) -> syn::TypeNever {
            skip!(node.bang_token);
        }
        direct_type_param(director, node) -> syn::TypeParam {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.ident);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
            skip!(node.eq_token);
            if let Some(it) = &node.default {
                crate::Director::direct(director, it);
            }
        }
        direct_type_param_bound(director, node) -> syn::TypeParamBound {
            match node {
                syn::TypeParamBound::Trait(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::TypeParamBound::Lifetime(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::TypeParamBound::PreciseCapture(_binding_0) => {
                    full!(crate::Director::direct(director, _binding_0));
                }
                syn::TypeParamBound::Verbatim(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_type_paren(director, node) -> syn::TypeParen {
            skip!(node.paren_token);
            crate::Director::direct(director, &*node.elem);
        }
        direct_type_path(director, node) -> syn::TypePath {
            if let Some(it) = &node.qself {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.path);
        }
        direct_type_ptr(director, node) -> syn::TypePtr {
            skip!(node.star_token);
            skip!(node.const_token);
            skip!(node.mutability);
            crate::Director::direct(director, &*node.elem);
        }
        direct_type_reference(director, node) -> syn::TypeReference {
            skip!(node.and_token);
            if let Some(it) = &node.lifetime {
                crate::Director::direct(director, it);
            }
            skip!(node.mutability);
            crate::Director::direct(director, &*node.elem);
        }
        direct_type_slice(director, node) -> syn::TypeSlice {
            skip!(node.bracket_token);
            crate::Director::direct(director, &*node.elem);
        }
        direct_type_trait_object(director, node) -> syn::TypeTraitObject {
            skip!(node.dyn_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_type_tuple(director, node) -> syn::TypeTuple {
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_un_op(_director, node) -> syn::UnOp {
            match node {
                syn::UnOp::Deref(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::UnOp::Not(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::UnOp::Neg(_binding_0) => {
                    skip!(_binding_0);
                }
                _ => { }
            }
        }
        direct_use_glob(_director, node) -> syn::UseGlob {
            skip!(node.star_token);
        }
        direct_use_group(director, node) -> syn::UseGroup {
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.items) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_use_name(director, node) -> syn::UseName {
            crate::Director::direct(director, &node.ident);
        }
        direct_use_path(director, node) -> syn::UsePath {
            crate::Director::direct(director, &node.ident);
            skip!(node.colon2_token);
            crate::Director::direct(director, &*node.tree);
        }
        direct_use_rename(director, node) -> syn::UseRename {
            crate::Director::direct(director, &node.ident);
            skip!(node.as_token);
            crate::Director::direct(director, &node.rename);
        }
        direct_use_tree(director, node) -> syn::UseTree {
            match node {
                syn::UseTree::Path(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::UseTree::Name(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::UseTree::Rename(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::UseTree::Glob(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::UseTree::Group(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
            }
        }
        direct_variadic(director, node) -> syn::Variadic {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            if let Some(it) = &node.pat {
                crate::Director::direct(director, &*(it).0);
                skip!((it).1);
            }
            skip!(node.dots);
            skip!(node.comma);
        }
        direct_variant(director, node) -> syn::Variant {
            for it in &node.attrs {
                crate::Director::direct(director, it);
            }
            crate::Director::direct(director, &node.ident);
            crate::Director::direct(director, &node.fields);
            if let Some(it) = &node.discriminant {
                skip!((it).0);
                crate::Director::direct(director, &(it).1);
            }
        }
        direct_vis_restricted(director, node) -> syn::VisRestricted {
            skip!(node.pub_token);
            skip!(node.paren_token);
            skip!(node.in_token);
            crate::Director::direct(director, &*node.path);
        }
        direct_visibility(director, node) -> syn::Visibility {
            match node {
                syn::Visibility::Public(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::Visibility::Restricted(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::Visibility::Inherited => {}
            }
        }
        direct_where_clause(director, node) -> syn::WhereClause {
            skip!(node.where_token);
            for el in syn::punctuated::Punctuated::pairs(&node.predicates) {
                let it = el.value();
                crate::Director::direct(director, *it);
            }
        }
        direct_where_predicate(director, node) -> syn::WherePredicate {
            match node {
                syn::WherePredicate::Lifetime(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                syn::WherePredicate::Type(_binding_0) => {
                    crate::Director::direct(director, _binding_0);
                }
                _ => { }
            }
        }
        // Added nodes not originally present in syn
        direct_generics_enter(_director, _node) -> crate::syn::GenericsEnter {

        }
        direct_generics_exit(_director, _node) -> crate::syn::GenericsExit {

        }
    }
}

node_set_mut! {
    /// A convenience trait for creating [syn] directors. Implementing this trait will also implement [crate::DirectMut] for
    /// all 'feature = "full"' syn ast types. Each impl has a default corresponding to [syn]'s traversal behavior.
    pub trait FullMut {
        direct_abi_mut(director, node) -> syn::Abi {
            skip!(node.extern_token);
            if let Some(it) = &mut node.name {
                crate::Director::direct_mut(director, it);
            }
        }
        direct_angle_bracketed_generic_arguments_mut(director, node) -> syn::AngleBracketedGenericArguments {
            skip!(node.colon2_token);
            skip!(node.lt_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.args) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            skip!(node.gt_token);
        }
        direct_arm_mut(director, node) -> syn::Arm {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.pat);
            if let Some(it) = &mut node.guard {
                skip!((it).0);
                crate::Director::direct_mut(director, &mut *(it).1);
            }
            skip!(node.fat_arrow_token);
            crate::Director::direct_mut(director, &mut *node.body);
            skip!(node.comma);
        }
        direct_assoc_const_mut(director, node) -> syn::AssocConst {
            crate::Director::direct_mut(director, &mut node.ident);
            if let Some(it) = &mut node.generics {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut node.value);
        }
        direct_assoc_type_mut(director, node) -> syn::AssocType {
            crate::Director::direct_mut(director, &mut node.ident);
            if let Some(it) = &mut node.generics {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut node.ty);
        }
        direct_attr_style_mut(_director, node) -> syn::AttrStyle {
            match node {
                syn::AttrStyle::Outer => {}
                syn::AttrStyle::Inner(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        direct_attribute_mut(director, node) -> syn::Attribute {
            skip!(node.pound_token);
            crate::Director::direct_mut(director, &mut node.style);
            skip!(node.bracket_token);
            crate::Director::direct_mut(director, &mut node.meta);
        }
        direct_bare_fn_arg_mut(director, node) -> syn::BareFnArg {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.name {
                crate::Director::direct_mut(director, &mut (it).0);
                skip!((it).1);
            }
            crate::Director::direct_mut(director, &mut node.ty);
        }
        direct_bare_variadic_mut(director, node) -> syn::BareVariadic {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.name {
                crate::Director::direct_mut(director, &mut (it).0);
                skip!((it).1);
            }
            skip!(node.dots);
            skip!(node.comma);
        }
        direct_bin_op_mut(_director, node) -> syn::BinOp {
            match node {
                syn::BinOp::Add(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Sub(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Mul(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Div(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Rem(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::And(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Or(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitXor(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitAnd(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitOr(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Shl(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Shr(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Eq(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Lt(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Le(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Ne(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Ge(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::Gt(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::AddAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::SubAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::MulAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::DivAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::RemAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitXorAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitAndAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::BitOrAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::ShlAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::BinOp::ShrAssign(_binding_0) => {
                    skip!(_binding_0);
                }
                _ => { }
            }
        }
        direct_block_mut(director, node) -> syn::Block {
            skip!(node.brace_token);
            for it in &mut node.stmts {
                crate::Director::direct_mut(director, it);
            }
        }
        direct_bound_lifetimes_mut(director, node) -> syn::BoundLifetimes {
            skip!(node.for_token);
            skip!(node.lt_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.lifetimes) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            skip!(node.gt_token);
        }
        direct_captured_param_mut(director, node) -> syn::CapturedParam {
            match node {
                syn::CapturedParam::Lifetime(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::CapturedParam::Ident(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_const_param_mut(director, node) -> syn::ConstParam {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.const_token);
            crate::Director::direct_mut(director, &mut node.ident);
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut node.ty);
            skip!(node.eq_token);
            if let Some(it) = &mut node.default {
                crate::Director::direct_mut(director, it);
            }
        }
        direct_constraint_mut(director, node) -> syn::Constraint {
            crate::Director::direct_mut(director, &mut node.ident);
            if let Some(it) = &mut node.generics {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.colon_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.bounds) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_data_mut(director, node) -> syn::Data {
            match node {
                syn::Data::Struct(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Data::Enum(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Data::Union(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
            }
        }
        direct_data_enum_mut(director, node) -> syn::DataEnum {
            skip!(node.enum_token);
            skip!(node.brace_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.variants) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_data_struct_mut(director, node) -> syn::DataStruct {
            skip!(node.struct_token);
            crate::Director::direct_mut(director, &mut node.fields);
            skip!(node.semi_token);
        }
        direct_data_union_mut(director, node) -> syn::DataUnion {
            skip!(node.union_token);
            crate::Director::direct_mut(director, &mut node.fields);
        }
        direct_derive_input_mut(director, node) -> syn::DeriveInput {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            crate::Director::direct_mut(director, &mut node.data);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_expr_mut(director, node) -> syn::Expr {
            match node {
                syn::Expr::Array(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Assign(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Async(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Await(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Binary(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Block(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Break(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Call(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Cast(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Closure(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Const(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Continue(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Field(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::ForLoop(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Group(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::If(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Index(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Infer(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Let(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Lit(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Loop(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Macro(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Match(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::MethodCall(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Paren(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Path(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Range(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::RawAddr(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Reference(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Repeat(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Return(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Struct(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Try(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::TryBlock(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Tuple(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Unary(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::Unsafe(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Verbatim(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Expr::While(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::Expr::Yield(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                _ => { }
            }
        }
        direct_expr_array_mut(director, node) -> syn::ExprArray {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.bracket_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.elems) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_expr_assign_mut(director, node) -> syn::ExprAssign {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut *node.left);
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut *node.right);
        }
        direct_expr_async_mut(director, node) -> syn::ExprAsync {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.async_token);
            skip!(node.capture);
            crate::Director::direct_mut(director, &mut node.block);
        }
        direct_expr_await_mut(director, node) -> syn::ExprAwait {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut *node.base);
            skip!(node.dot_token);
            skip!(node.await_token);
        }
        direct_expr_binary_mut(director, node) -> syn::ExprBinary {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut *node.left);
            crate::Director::direct_mut(director, &mut node.op);
            crate::Director::direct_mut(director, &mut *node.right);
        }
        direct_expr_block_mut(director, node) -> syn::ExprBlock {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.label {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.block);
        }
        direct_expr_break_mut(director, node) -> syn::ExprBreak {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.break_token);
            if let Some(it) = &mut node.label {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.expr {
                crate::Director::direct_mut(director, &mut **it);
            }
        }
        direct_expr_call_mut(director, node) -> syn::ExprCall {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut *node.func);
            skip!(node.paren_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.args) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_expr_cast_mut(director, node) -> syn::ExprCast {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut *node.expr);
            skip!(node.as_token);
            crate::Director::direct_mut(director, &mut *node.ty);
        }
        direct_expr_closure_mut(director, node) -> syn::ExprClosure {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.lifetimes {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.constness);
            skip!(node.movability);
            skip!(node.asyncness);
            skip!(node.capture);
            skip!(node.or1_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.inputs) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            skip!(node.or2_token);
            crate::Director::direct_mut(director, &mut node.output);
            crate::Director::direct_mut(director, &mut *node.body);
        }
        direct_expr_const_mut(director, node) -> syn::ExprConst {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.const_token);
            crate::Director::direct_mut(director, &mut node.block);
        }
        direct_expr_continue_mut(director, node) -> syn::ExprContinue {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.continue_token);
            if let Some(it) = &mut node.label {
                crate::Director::direct_mut(director, it);
            }
        }
        direct_expr_field_mut(director, node) -> syn::ExprField {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut *node.base);
            skip!(node.dot_token);
            crate::Director::direct_mut(director, &mut node.member);
        }
        direct_expr_for_loop_mut(director, node) -> syn::ExprForLoop {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.label {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.for_token);
            crate::Director::direct_mut(director, &mut *node.pat);
            skip!(node.in_token);
            crate::Director::direct_mut(director, &mut *node.expr);
            crate::Director::direct_mut(director, &mut node.body);
        }
        direct_expr_group_mut(director, node) -> syn::ExprGroup {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.group_token);
            crate::Director::direct_mut(director, &mut *node.expr);
        }
        direct_expr_if_mut(director, node) -> syn::ExprIf {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.if_token);
            crate::Director::direct_mut(director, &mut *node.cond);
            crate::Director::direct_mut(director, &mut node.then_branch);
            if let Some(it) = &mut node.else_branch {
                skip!((it).0);
                crate::Director::direct_mut(director, &mut *(it).1);
            }
        }
        direct_expr_index_mut(director, node) -> syn::ExprIndex {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut *node.expr);
            skip!(node.bracket_token);
            crate::Director::direct_mut(director, &mut *node.index);
        }
        direct_expr_infer_mut(director, node) -> syn::ExprInfer {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.underscore_token);
        }
        direct_expr_let_mut(director, node) -> syn::ExprLet {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.let_token);
            crate::Director::direct_mut(director, &mut *node.pat);
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut *node.expr);
        }
        direct_expr_lit_mut(director, node) -> syn::ExprLit {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.lit);
        }
        direct_expr_loop_mut(director, node) -> syn::ExprLoop {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.label {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.loop_token);
            crate::Director::direct_mut(director, &mut node.body);
        }
        direct_expr_macro_mut(director, node) -> syn::ExprMacro {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.mac);
        }
        direct_expr_match_mut(director, node) -> syn::ExprMatch {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.match_token);
            crate::Director::direct_mut(director, &mut *node.expr);
            skip!(node.brace_token);
            for it in &mut node.arms {
                crate::Director::direct_mut(director, it);
            }
        }
        direct_expr_method_call_mut(director, node) -> syn::ExprMethodCall {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut *node.receiver);
            skip!(node.dot_token);
            crate::Director::direct_mut(director, &mut node.method);
            if let Some(it) = &mut node.turbofish {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.paren_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.args) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_expr_paren_mut(director, node) -> syn::ExprParen {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.paren_token);
            crate::Director::direct_mut(director, &mut *node.expr);
        }
        direct_expr_path_mut(director, node) -> syn::ExprPath {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.qself {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.path);
        }
        direct_expr_range_mut(director, node) -> syn::ExprRange {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.start {
                crate::Director::direct_mut(director, &mut **it);
            }
            crate::Director::direct_mut(director, &mut node.limits);
            if let Some(it) = &mut node.end {
                crate::Director::direct_mut(director, &mut **it);
            }
        }
        direct_expr_raw_addr_mut(director, node) -> syn::ExprRawAddr {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.and_token);
            skip!(node.raw);
            crate::Director::direct_mut(director, &mut node.mutability);
            crate::Director::direct_mut(director, &mut *node.expr);
        }
        direct_expr_reference_mut(director, node) -> syn::ExprReference {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.and_token);
            skip!(node.mutability);
            crate::Director::direct_mut(director, &mut *node.expr);
        }
        direct_expr_repeat_mut(director, node) -> syn::ExprRepeat {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.bracket_token);
            crate::Director::direct_mut(director, &mut *node.expr);
            skip!(node.semi_token);
            crate::Director::direct_mut(director, &mut *node.len);
        }
        direct_expr_return_mut(director, node) -> syn::ExprReturn {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.return_token);
            if let Some(it) = &mut node.expr {
                crate::Director::direct_mut(director, &mut **it);
            }
        }
        direct_expr_struct_mut(director, node) -> syn::ExprStruct {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.qself {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.path);
            skip!(node.brace_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.fields) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            skip!(node.dot2_token);
            if let Some(it) = &mut node.rest {
                crate::Director::direct_mut(director, &mut **it);
            }
        }
        direct_expr_try_mut(director, node) -> syn::ExprTry {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut *node.expr);
            skip!(node.question_token);
        }
        direct_expr_try_block_mut(director, node) -> syn::ExprTryBlock {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.try_token);
            crate::Director::direct_mut(director, &mut node.block);
        }
        direct_expr_tuple_mut(director, node) -> syn::ExprTuple {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.paren_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.elems) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_expr_unary_mut(director, node) -> syn::ExprUnary {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.op);
            crate::Director::direct_mut(director, &mut *node.expr);
        }
        direct_expr_unsafe_mut(director, node) -> syn::ExprUnsafe {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.unsafe_token);
            crate::Director::direct_mut(director, &mut node.block);
        }
        direct_expr_while_mut(director, node) -> syn::ExprWhile {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.label {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.while_token);
            crate::Director::direct_mut(director, &mut *node.cond);
            crate::Director::direct_mut(director, &mut node.body);
        }
        direct_expr_yield_mut(director, node) -> syn::ExprYield {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.yield_token);
            if let Some(it) = &mut node.expr {
                crate::Director::direct_mut(director, &mut **it);
            }
        }
        direct_field_mut(director, node) -> syn::Field {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            crate::Director::direct_mut(director, &mut node.mutability);
            if let Some(it) = &mut node.ident {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut node.ty);
        }
        direct_field_mutability_mut(_director, _node) -> syn::FieldMutability {

        }
        direct_field_pat_mut(director, node) -> syn::FieldPat {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.member);
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut *node.pat);
        }
        direct_field_value_mut(director, node) -> syn::FieldValue {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.member);
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut node.expr);
        }
        direct_fields_mut(director, node) -> syn::Fields {
            match node {
                syn::Fields::Named(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Fields::Unnamed(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Fields::Unit => {}
            }
        }
        direct_fields_named_mut(director, node) -> syn::FieldsNamed {
            skip!(node.brace_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.named) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_fields_unnamed_mut(director, node) -> syn::FieldsUnnamed {
            skip!(node.paren_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.unnamed) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_file_mut(director, node) -> syn::File {
            skip!(node.shebang);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            for it in &mut node.items {
                crate::Director::direct_mut(director, it);
            }
        }
        direct_fn_arg_mut(director, node) -> syn::FnArg {
            match node {
                syn::FnArg::Receiver(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::FnArg::Typed(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
            }
        }
        direct_foreign_item_mut(director, node) -> syn::ForeignItem {
            match node {
                syn::ForeignItem::Fn(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::ForeignItem::Static(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::ForeignItem::Type(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::ForeignItem::Macro(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::ForeignItem::Verbatim(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_foreign_item_fn_mut(director, node) -> syn::ForeignItemFn {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            crate::Director::direct_mut(director, &mut node.sig);
            skip!(node.semi_token);
        }
        direct_foreign_item_macro_mut(director, node) -> syn::ForeignItemMacro {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.mac);
            skip!(node.semi_token);
        }
        direct_foreign_item_static_mut(director, node) -> syn::ForeignItemStatic {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.static_token);
            crate::Director::direct_mut(director, &mut node.mutability);
            crate::Director::direct_mut(director, &mut node.ident);
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut *node.ty);
            skip!(node.semi_token);
        }
        direct_foreign_item_type_mut(director, node) -> syn::ForeignItemType {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.type_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.semi_token);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_generic_argument_mut(director, node) -> syn::GenericArgument {
            match node {
                syn::GenericArgument::Lifetime(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::GenericArgument::Type(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::GenericArgument::Const(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::GenericArgument::AssocType(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::GenericArgument::AssocConst(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::GenericArgument::Constraint(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_generic_param_mut(director, node) -> syn::GenericParam {
            match node {
                syn::GenericParam::Lifetime(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::GenericParam::Type(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::GenericParam::Const(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
            }
        }
        direct_generics_mut(director, node) -> syn::Generics {
            skip!(node.lt_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.params) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            skip!(node.gt_token);
            if let Some(it) = &mut node.where_clause {
                crate::Director::direct_mut(director, it);
            }
        }
        direct_ident_mut(_director, node) -> proc_macro2::Ident {
            skip!(node.span());
        }
        direct_impl_item_mut(director, node) -> syn::ImplItem {
            match node {
                syn::ImplItem::Const(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::ImplItem::Fn(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::ImplItem::Type(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::ImplItem::Macro(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::ImplItem::Verbatim(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_impl_item_const_mut(director, node) -> syn::ImplItemConst {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.defaultness);
            skip!(node.const_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut node.ty);
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut node.expr);
            skip!(node.semi_token);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_impl_item_fn_mut(director, node) -> syn::ImplItemFn {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.defaultness);
            crate::Director::direct_mut(director, &mut node.sig);
            crate::Director::direct_mut(director, &mut node.block);
        }
        direct_impl_item_macro_mut(director, node) -> syn::ImplItemMacro {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.mac);
            skip!(node.semi_token);
        }
        direct_impl_item_type_mut(director, node) -> syn::ImplItemType {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.defaultness);
            skip!(node.type_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut node.ty);
            skip!(node.semi_token);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_impl_restriction_mut(_director, _node) -> syn::ImplRestriction {

        }
        direct_index_mut(_director, node) -> syn::Index {
            skip!(node.index);
            skip!(node.span);
        }
        direct_item_mut(director, node) -> syn::Item {
            match node {
                syn::Item::Const(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Enum(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::ExternCrate(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Fn(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::ForeignMod(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Impl(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Macro(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Mod(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Static(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Struct(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Trait(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::TraitAlias(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Type(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Union(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Use(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Item::Verbatim(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_item_const_mut(director, node) -> syn::ItemConst {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.const_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut *node.ty);
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut *node.expr);
            skip!(node.semi_token);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_item_enum_mut(director, node) -> syn::ItemEnum {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.enum_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.brace_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.variants) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_item_extern_crate_mut(director, node) -> syn::ItemExternCrate {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.extern_token);
            skip!(node.crate_token);
            crate::Director::direct_mut(director, &mut node.ident);
            if let Some(it) = &mut node.rename {
                skip!((it).0);
                crate::Director::direct_mut(director, &mut (it).1);
            }
            skip!(node.semi_token);
        }
        direct_item_fn_mut(director, node) -> syn::ItemFn {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            crate::Director::direct_mut(director, &mut node.sig);
            crate::Director::direct_mut(director, &mut *node.block);
        }
        direct_item_foreign_mod_mut(director, node) -> syn::ItemForeignMod {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.unsafety);
            crate::Director::direct_mut(director, &mut node.abi);
            skip!(node.brace_token);
            for it in &mut node.items {
                crate::Director::direct_mut(director, it);
            }
        }
        direct_item_impl_mut(director, node) -> syn::ItemImpl {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.defaultness);
            skip!(node.unsafety);
            skip!(node.impl_token);
            crate::Director::direct_mut(director, &mut node.generics);
            if let Some(it) = &mut node.trait_ {
                skip!((it).0);
                crate::Director::direct_mut(director, &mut (it).1);
                skip!((it).2);
            }
            crate::Director::direct_mut(director, &mut *node.self_ty);
            skip!(node.brace_token);
            for it in &mut node.items {
                crate::Director::direct_mut(director, it);
            }
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_item_macro_mut(director, node) -> syn::ItemMacro {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.ident {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.mac);
            skip!(node.semi_token);
        }
        direct_item_mod_mut(director, node) -> syn::ItemMod {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.unsafety);
            skip!(node.mod_token);
            crate::Director::direct_mut(director, &mut node.ident);
            if let Some(it) = &mut node.content {
                skip!((it).0);
                for it in &mut (it).1 {
                    crate::Director::direct_mut(director, it);
                }
            }
            skip!(node.semi);
        }
        direct_item_static_mut(director, node) -> syn::ItemStatic {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.static_token);
            crate::Director::direct_mut(director, &mut node.mutability);
            crate::Director::direct_mut(director, &mut node.ident);
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut *node.ty);
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut *node.expr);
            skip!(node.semi_token);
        }
        direct_item_struct_mut(director, node) -> syn::ItemStruct {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.struct_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            crate::Director::direct_mut(director, &mut node.fields);
            skip!(node.semi_token);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_item_trait_mut(director, node) -> syn::ItemTrait {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.unsafety);
            skip!(node.auto_token);
            if let Some(it) = &mut node.restriction {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.trait_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.colon_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.supertraits) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            skip!(node.brace_token);
            for it in &mut node.items {
                crate::Director::direct_mut(director, it);
            }
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_item_trait_alias_mut(director, node) -> syn::ItemTraitAlias {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.trait_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.eq_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.bounds) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            skip!(node.semi_token);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_item_type_mut(director, node) -> syn::ItemType {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.type_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut *node.ty);
            skip!(node.semi_token);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_item_union_mut(director, node) -> syn::ItemUnion {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.union_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            crate::Director::direct_mut(director, &mut node.fields);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_item_use_mut(director, node) -> syn::ItemUse {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.vis);
            skip!(node.use_token);
            skip!(node.leading_colon);
            crate::Director::direct_mut(director, &mut node.tree);
            skip!(node.semi_token);
        }
        direct_label_mut(director, node) -> syn::Label {
            crate::Director::direct_mut(director, &mut node.name);
            skip!(node.colon_token);
        }
        direct_lifetime_mut(director, node) -> syn::Lifetime {
            skip!(node.apostrophe);
            crate::Director::direct_mut(director, &mut node.ident);
        }
        direct_lifetime_param_mut(director, node) -> syn::LifetimeParam {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.lifetime);
            skip!(node.colon_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.bounds) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_lit_mut(director, node) -> syn::Lit {
            match node {
                syn::Lit::Str(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Lit::ByteStr(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Lit::CStr(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Lit::Byte(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Lit::Char(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Lit::Int(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Lit::Float(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Lit::Bool(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Lit::Verbatim(_binding_0) => {
                    skip!(_binding_0);
                }
                _ => { }
            }
        }
        direct_lit_bool_mut(_director, node) -> syn::LitBool {
            skip!(node.value);
            skip!(node.span);
        }
        direct_lit_byte_mut(_director, _node) -> syn::LitByte {

        }
        direct_lit_byte_str_mut(_director, _node) -> syn::LitByteStr {

        }
        direct_lit_cstr_mut(_director, _node) -> syn::LitCStr {

        }
        direct_lit_char_mut(_director, _node) -> syn::LitChar {

        }
        direct_lit_float_mut(_director, _node) -> syn::LitFloat {

        }
        direct_lit_int_mut(_director, _node) -> syn::LitInt {

        }
        direct_lit_str_mut(_director, _node) -> syn::LitStr {

        }
        direct_local_mut(director, node) -> syn::Local {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.let_token);
            crate::Director::direct_mut(director, &mut node.pat);
            if let Some(it) = &mut node.init {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.semi_token);
        }
        direct_local_init_mut(director, node) -> syn::LocalInit {
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut *node.expr);
            if let Some(it) = &mut node.diverge {
                skip!((it).0);
                crate::Director::direct_mut(director, &mut *(it).1);
            }
        }
        direct_macro_mut(director, node) -> syn::Macro {
            crate::Director::direct_mut(director, &mut node.path);
            skip!(node.bang_token);
            crate::Director::direct_mut(director, &mut node.delimiter);
            crate::Director::direct_mut(director, &mut node.tokens);
        }
        direct_macro_delimiter_mut(_director, node) -> syn::MacroDelimiter {
            match node {
                syn::MacroDelimiter::Paren(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::MacroDelimiter::Brace(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::MacroDelimiter::Bracket(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        direct_member_mut(director, node) -> syn::Member {
            match node {
                syn::Member::Named(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Member::Unnamed(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
            }
        }
        direct_meta_mut(director, node) -> syn::Meta {
            match node {
                syn::Meta::Path(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Meta::List(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Meta::NameValue(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
            }
        }
        direct_meta_list_mut(director, node) -> syn::MetaList {
            crate::Director::direct_mut(director, &mut node.path);
            crate::Director::direct_mut(director, &mut node.delimiter);
            crate::Director::direct_mut(director, &mut node.tokens);
        }
        direct_meta_name_value_mut(director, node) -> syn::MetaNameValue {
            crate::Director::direct_mut(director, &mut node.path);
            skip!(node.eq_token);
            crate::Director::direct_mut(director, &mut node.value);
        }
        direct_parenthesized_generic_arguments_mut(director, node) -> syn::ParenthesizedGenericArguments {
            skip!(node.paren_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.inputs) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            crate::Director::direct_mut(director, &mut node.output);
        }
        direct_pat_mut(director, node) -> syn::Pat {
            match node {
                syn::Pat::Const(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Ident(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Lit(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Macro(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Or(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Paren(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Path(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Range(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Reference(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Rest(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Slice(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Struct(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Tuple(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::TupleStruct(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Type(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Verbatim(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Pat::Wild(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_pat_ident_mut(director, node) -> syn::PatIdent {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.by_ref);
            skip!(node.mutability);
            crate::Director::direct_mut(director, &mut node.ident);
            if let Some(it) = &mut node.subpat {
                skip!((it).0);
                crate::Director::direct_mut(director, &mut *(it).1);
            }
        }
        direct_pat_or_mut(director, node) -> syn::PatOr {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.leading_vert);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.cases) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_pat_paren_mut(director, node) -> syn::PatParen {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.paren_token);
            crate::Director::direct_mut(director, &mut *node.pat);
        }
        direct_pat_reference_mut(director, node) -> syn::PatReference {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.and_token);
            skip!(node.mutability);
            crate::Director::direct_mut(director, &mut *node.pat);
        }
        direct_pat_rest_mut(director, node) -> syn::PatRest {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.dot2_token);
        }
        direct_pat_slice_mut(director, node) -> syn::PatSlice {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.bracket_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.elems) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_pat_struct_mut(director, node) -> syn::PatStruct {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.qself {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.path);
            skip!(node.brace_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.fields) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            if let Some(it) = &mut node.rest {
                crate::Director::direct_mut(director, it);
            }
        }
        direct_pat_tuple_mut(director, node) -> syn::PatTuple {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.paren_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.elems) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_pat_tuple_struct_mut(director, node) -> syn::PatTupleStruct {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.qself {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.path);
            skip!(node.paren_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.elems) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_pat_type_mut(director, node) -> syn::PatType {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut *node.pat);
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut *node.ty);
        }
        direct_pat_wild_mut(director, node) -> syn::PatWild {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.underscore_token);
        }
        direct_path_mut(director, node) -> syn::Path {
            skip!(node.leading_colon);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.segments) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_path_arguments_mut(director, node) -> syn::PathArguments {
            match node {
                syn::PathArguments::None => {}
                syn::PathArguments::AngleBracketed(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::PathArguments::Parenthesized(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
            }
        }
        direct_path_segment_mut(director, node) -> syn::PathSegment {
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.arguments);
        }
        direct_pointer_mutability_mut(_director, node) -> syn::PointerMutability {
            match node {
                syn::PointerMutability::Const(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::PointerMutability::Mut(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        direct_precise_capture_mut(director, node) -> syn::PreciseCapture {
            skip!(node.use_token);
            skip!(node.lt_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.params) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            skip!(node.gt_token);
        }
        direct_predicate_lifetime_mut(director, node) -> syn::PredicateLifetime {
            crate::Director::direct_mut(director, &mut node.lifetime);
            skip!(node.colon_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.bounds) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_predicate_type_mut(director, node) -> syn::PredicateType {
            if let Some(it) = &mut node.lifetimes {
                super::enter_scope_mut(director, &mut it.lifetimes);
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.bounded_ty);
            skip!(node.colon_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.bounds) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            if let Some(it) = &mut node.lifetimes {
                super::exit_scope_mut(director, &mut it.lifetimes);
            }
        }
        direct_qself_mut(director, node) -> syn::QSelf {
            skip!(node.lt_token);
            crate::Director::direct_mut(director, &mut *node.ty);
            skip!(node.position);
            skip!(node.as_token);
            skip!(node.gt_token);
        }
        direct_range_limits_mut(_director, node) -> syn::RangeLimits {
            match node {
                syn::RangeLimits::HalfOpen(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::RangeLimits::Closed(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        direct_receiver_mut(director, node) -> syn::Receiver {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.reference {
                skip!((it).0);
                if let Some(it) = &mut (it).1 {
                    crate::Director::direct_mut(director, it);
                }
            }
            skip!(node.mutability);
            skip!(node.self_token);
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut *node.ty);
        }
        direct_return_type_mut(director, node) -> syn::ReturnType {
            match node {
                syn::ReturnType::Default => {}
                syn::ReturnType::Type(_binding_0, _binding_1) => {
                    skip!(_binding_0);
                    crate::Director::direct_mut(director, &mut **_binding_1);
                }
            }
        }
        direct_signature_mut(director, node) -> syn::Signature {
            super::enter_scope_mut(director, &mut node.generics.params);
            skip!(node.constness);
            skip!(node.asyncness);
            skip!(node.unsafety);
            if let Some(it) = &mut node.abi {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.fn_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.paren_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.inputs) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            if let Some(it) = &mut node.variadic {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.output);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_static_mutability_mut(_director, node) -> syn::StaticMutability {
            match node {
                syn::StaticMutability::Mut(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::StaticMutability::None => {}
                _ => { }
            }
        }
        direct_stmt_mut(director, node) -> syn::Stmt {
            match node {
                syn::Stmt::Local(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Stmt::Item(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Stmt::Expr(_binding_0, _binding_1) => {
                    crate::Director::direct_mut(director, _binding_0);
                    skip!(_binding_1);
                }
                syn::Stmt::Macro(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
            }
        }
        direct_stmt_macro_mut(director, node) -> syn::StmtMacro {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.mac);
            skip!(node.semi_token);
        }
        direct_token_stream_mut(_director, _node) -> proc_macro2::TokenStream {

        }
        direct_trait_bound_mut(director, node) -> syn::TraitBound {
            skip!(node.paren_token);
            crate::Director::direct_mut(director, &mut node.modifier);
            if let Some(it) = &mut node.lifetimes {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.path);
        }
        direct_trait_bound_modifier_mut(_director, node) -> syn::TraitBoundModifier {
            match node {
                syn::TraitBoundModifier::None => {}
                syn::TraitBoundModifier::Maybe(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        direct_trait_item_mut(director, node) -> syn::TraitItem {
            match node {
                syn::TraitItem::Const(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::TraitItem::Fn(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::TraitItem::Type(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::TraitItem::Macro(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::TraitItem::Verbatim(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_trait_item_const_mut(director, node) -> syn::TraitItemConst {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.const_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.colon_token);
            crate::Director::direct_mut(director, &mut node.ty);
            if let Some(it) = &mut node.default {
                skip!((it).0);
                crate::Director::direct_mut(director, &mut (it).1);
            }
            skip!(node.semi_token);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_trait_item_fn_mut(director, node) -> syn::TraitItemFn {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.sig);
            if let Some(it) = &mut node.default {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.semi_token);
        }
        direct_trait_item_macro_mut(director, node) -> syn::TraitItemMacro {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.mac);
            skip!(node.semi_token);
        }
        direct_trait_item_type_mut(director, node) -> syn::TraitItemType {
            super::enter_scope_mut(director, &mut node.generics.params);
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.type_token);
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.generics);
            skip!(node.colon_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.bounds) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            if let Some(it) = &mut node.default {
                skip!((it).0);
                crate::Director::direct_mut(director, &mut (it).1);
            }
            skip!(node.semi_token);
            super::exit_scope_mut(director, &mut node.generics.params);
        }
        direct_type_mut(director, node) -> syn::Type {
            match node {
                syn::Type::Array(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::BareFn(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Group(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::ImplTrait(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Infer(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Macro(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Never(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Paren(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Path(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Ptr(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Reference(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Slice(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::TraitObject(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Tuple(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Type::Verbatim(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_type_array_mut(director, node) -> syn::TypeArray {
            skip!(node.bracket_token);
            crate::Director::direct_mut(director, &mut *node.elem);
            skip!(node.semi_token);
            crate::Director::direct_mut(director, &mut node.len);
        }
        direct_type_bare_fn_mut(director, node) -> syn::TypeBareFn {
            if let Some(it) = &mut node.lifetimes {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.unsafety);
            if let Some(it) = &mut node.abi {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.fn_token);
            skip!(node.paren_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.inputs) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            if let Some(it) = &mut node.variadic {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.output);
        }
        direct_type_group_mut(director, node) -> syn::TypeGroup {
            skip!(node.group_token);
            crate::Director::direct_mut(director, &mut *node.elem);
        }
        direct_type_impl_trait_mut(director, node) -> syn::TypeImplTrait {
            skip!(node.impl_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.bounds) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_type_infer_mut(_director, node) -> syn::TypeInfer {
            skip!(node.underscore_token);
        }
        direct_type_macro_mut(director, node) -> syn::TypeMacro {
            crate::Director::direct_mut(director, &mut node.mac);
        }
        direct_type_never_mut(_director, node) -> syn::TypeNever {
            skip!(node.bang_token);
        }
        direct_type_param_mut(director, node) -> syn::TypeParam {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.ident);
            skip!(node.colon_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.bounds) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
            skip!(node.eq_token);
            if let Some(it) = &mut node.default {
                crate::Director::direct_mut(director, it);
            }
        }
        direct_type_param_bound_mut(director, node) -> syn::TypeParamBound {
            match node {
                syn::TypeParamBound::Trait(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::TypeParamBound::Lifetime(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::TypeParamBound::PreciseCapture(_binding_0) => {
                    full!(crate::Director::direct_mut(director, _binding_0));
                }
                syn::TypeParamBound::Verbatim(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                _ => { }
            }
        }
        direct_type_paren_mut(director, node) -> syn::TypeParen {
            skip!(node.paren_token);
            crate::Director::direct_mut(director, &mut *node.elem);
        }
        direct_type_path_mut(director, node) -> syn::TypePath {
            if let Some(it) = &mut node.qself {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.path);
        }
        direct_type_ptr_mut(director, node) -> syn::TypePtr {
            skip!(node.star_token);
            skip!(node.const_token);
            skip!(node.mutability);
            crate::Director::direct_mut(director, &mut *node.elem);
        }
        direct_type_reference_mut(director, node) -> syn::TypeReference {
            skip!(node.and_token);
            if let Some(it) = &mut node.lifetime {
                crate::Director::direct_mut(director, it);
            }
            skip!(node.mutability);
            crate::Director::direct_mut(director, &mut *node.elem);
        }
        direct_type_slice_mut(director, node) -> syn::TypeSlice {
            skip!(node.bracket_token);
            crate::Director::direct_mut(director, &mut *node.elem);
        }
        direct_type_trait_object_mut(director, node) -> syn::TypeTraitObject {
            skip!(node.dyn_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.bounds) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_type_tuple_mut(director, node) -> syn::TypeTuple {
            skip!(node.paren_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.elems) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_un_op_mut(_director, node) -> syn::UnOp {
            match node {
                syn::UnOp::Deref(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::UnOp::Not(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::UnOp::Neg(_binding_0) => {
                    skip!(_binding_0);
                }
                _ => { }
            }
        }
        direct_use_glob_mut(_director, node) -> syn::UseGlob {
            skip!(node.star_token);
        }
        direct_use_group_mut(director, node) -> syn::UseGroup {
            skip!(node.brace_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.items) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_use_name_mut(director, node) -> syn::UseName {
            crate::Director::direct_mut(director, &mut node.ident);
        }
        direct_use_path_mut(director, node) -> syn::UsePath {
            crate::Director::direct_mut(director, &mut node.ident);
            skip!(node.colon2_token);
            crate::Director::direct_mut(director, &mut *node.tree);
        }
        direct_use_rename_mut(director, node) -> syn::UseRename {
            crate::Director::direct_mut(director, &mut node.ident);
            skip!(node.as_token);
            crate::Director::direct_mut(director, &mut node.rename);
        }
        direct_use_tree_mut(director, node) -> syn::UseTree {
            match node {
                syn::UseTree::Path(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::UseTree::Name(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::UseTree::Rename(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::UseTree::Glob(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::UseTree::Group(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
            }
        }
        direct_variadic_mut(director, node) -> syn::Variadic {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            if let Some(it) = &mut node.pat {
                crate::Director::direct_mut(director, &mut *(it).0);
                skip!((it).1);
            }
            skip!(node.dots);
            skip!(node.comma);
        }
        direct_variant_mut(director, node) -> syn::Variant {
            for it in &mut node.attrs {
                crate::Director::direct_mut(director, it);
            }
            crate::Director::direct_mut(director, &mut node.ident);
            crate::Director::direct_mut(director, &mut node.fields);
            if let Some(it) = &mut node.discriminant {
                skip!((it).0);
                crate::Director::direct_mut(director, &mut (it).1);
            }
        }
        direct_vis_restricted_mut(director, node) -> syn::VisRestricted {
            skip!(node.pub_token);
            skip!(node.paren_token);
            skip!(node.in_token);
            crate::Director::direct_mut(director, &mut *node.path);
        }
        direct_visibility_mut(director, node) -> syn::Visibility {
            match node {
                syn::Visibility::Public(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::Visibility::Restricted(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::Visibility::Inherited => {}
            }
        }
        direct_where_clause_mut(director, node) -> syn::WhereClause {
            skip!(node.where_token);
            for mut el in syn::punctuated::Punctuated::pairs_mut(&mut node.predicates) {
                let it = el.value_mut();
                crate::Director::direct_mut(director, *it);
            }
        }
        direct_where_predicate_mut(director, node) -> syn::WherePredicate {
            match node {
                syn::WherePredicate::Lifetime(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                syn::WherePredicate::Type(_binding_0) => {
                    crate::Director::direct_mut(director, _binding_0);
                }
                _ => { }
            }
        }
        // Added nodes not originally present in syn
        direct_generics_enter_mut(_director, _node) -> crate::syn::GenericsEnter {

        }
        direct_generics_exit_mut(_director, _node) -> crate::syn::GenericsExit {

        }
    }
}

fn enter_scope<D, V>(
    director: &mut crate::Director<'_, D, V>,
    generics: &syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>,
) where
    D: for<'g> crate::Direct<V, super::GenericsEnter> + ?Sized,
    V: for<'g> crate::Visit<super::GenericsEnter> + ?Sized,
{
    crate::Director::direct(director, super::GenericsEnter::new(generics));
}

fn exit_scope<D, V>(
    director: &mut crate::Director<'_, D, V>,
    generics: &syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>,
) where
    D: for<'g> crate::Direct<V, super::GenericsExit> + ?Sized,
    V: for<'g> crate::Visit<super::GenericsExit> + ?Sized,
{
    crate::Director::direct(director, super::GenericsExit::new(generics));
}

fn enter_scope_mut<D, V>(
    director: &mut crate::Director<'_, D, V>,
    generics: &mut syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>,
) where
    D: for<'g> crate::DirectMut<V, super::GenericsEnter> + ?Sized,
    V: for<'g> crate::VisitMut<super::GenericsEnter> + ?Sized,
{
    crate::Director::direct_mut(director, super::GenericsEnter::new_mut(generics));
}

fn exit_scope_mut<D, V>(
    director: &mut crate::Director<'_, D, V>,
    generics: &mut syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>,
) where
    D: for<'g> crate::DirectMut<V, super::GenericsExit> + ?Sized,
    V: for<'g> crate::VisitMut<super::GenericsExit> + ?Sized,
{
    crate::Director::direct_mut(director, super::GenericsExit::new_mut(generics));
}
