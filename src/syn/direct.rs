pub struct FullDefault;

impl Full for FullDefault { }

macro_rules! node_set {
    ($vis:vis trait $trait_ident:ident { $($fn_ident:ident($director_ident:ident, $node_ident:ident) -> $ty:ty $fn_impl:block)* }) => {
        #[allow(unused_mut)]
        $vis trait $trait_ident {
            $(
                fn $fn_ident<V>(mut $director_ident: crate::Director<'_, Self, V>, $node_ident: &$ty) 
                where
                    V: crate::syn::visit::Full + ?Sized,
                {
                    default::$fn_ident($director_ident, $node_ident);
                }
            )*
        }

        $(
            impl<'n, V, T> crate::Direct<V, $ty> for T 
            where
                V: crate::syn::visit::Full + ?Sized,
                T: Full + ?Sized,
            {
                fn direct(director: crate::Director<'_, Self, V>, node: &$ty) {
                    Self::$fn_ident(director, node);
                }
            }
        )*

        pub mod default {
            $(
                pub fn $fn_ident<D, V>(mut $director_ident: crate::Director<'_, D, V>, $node_ident: &$ty) 
                where
                    D: super::$trait_ident + ?Sized,
                    V: crate::syn::visit::Full + ?Sized,
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
    pub trait Full {
        visit_abi(director, node) -> syn::Abi {
            skip!(node.extern_token);
            if let Some(it) = &node.name {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_angle_bracketed_generic_arguments(director, node) -> syn::AngleBracketedGenericArguments {
            skip!(node.colon2_token);
            skip!(node.lt_token);
            for el in syn::punctuated::Punctuated::pairs(&node.args) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            skip!(node.gt_token);
        }
        visit_arm(director, node) -> syn::Arm {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.pat);
            if let Some(it) = &node.guard {
                skip!((it).0);
                crate::Director::direct(&mut director, &*(it).1);
            }
            skip!(node.fat_arrow_token);
            crate::Director::direct(&mut director, &*node.body);
            skip!(node.comma);
        }
        visit_assoc_const(director, node) -> syn::AssocConst {
            crate::Director::direct(&mut director, &node.ident);
            if let Some(it) = &node.generics {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &node.value);
        }
        visit_assoc_type(director, node) -> syn::AssocType {
            crate::Director::direct(&mut director, &node.ident);
            if let Some(it) = &node.generics {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &node.ty);
        }
        visit_attr_style(_director, node) -> syn::AttrStyle {
            match node {
                syn::AttrStyle::Outer => {}
                syn::AttrStyle::Inner(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        visit_attribute(director, node) -> syn::Attribute {
            skip!(node.pound_token);
            crate::Director::direct(&mut director, &node.style);
            skip!(node.bracket_token);
            crate::Director::direct(&mut director, &node.meta);
        }
        visit_bare_fn_arg(director, node) -> syn::BareFnArg {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.name {
                crate::Director::direct(&mut director, &(it).0);
                skip!((it).1);
            }
            crate::Director::direct(&mut director, &node.ty);
        }
        visit_bare_variadic(director, node) -> syn::BareVariadic {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.name {
                crate::Director::direct(&mut director, &(it).0);
                skip!((it).1);
            }
            skip!(node.dots);
            skip!(node.comma);
        }
        visit_bin_op(_director, node) -> syn::BinOp {
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
        visit_block(director, node) -> syn::Block {
            skip!(node.brace_token);
            for it in &node.stmts {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_bound_lifetimes(director, node) -> syn::BoundLifetimes {
            skip!(node.for_token);
            skip!(node.lt_token);
            for el in syn::punctuated::Punctuated::pairs(&node.lifetimes) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            skip!(node.gt_token);
        }
        visit_captured_param(director, node) -> syn::CapturedParam {
            match node {
                syn::CapturedParam::Lifetime(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::CapturedParam::Ident(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                _ => { }
            }
        }
        visit_const_param(director, node) -> syn::ConstParam {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.const_token);
            crate::Director::direct(&mut director, &node.ident);
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &node.ty);
            skip!(node.eq_token);
            if let Some(it) = &node.default {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_constraint(director, node) -> syn::Constraint {
            crate::Director::direct(&mut director, &node.ident);
            if let Some(it) = &node.generics {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_data(director, node) -> syn::Data {
            match node {
                syn::Data::Struct(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Data::Enum(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Data::Union(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
            }
        }
        visit_data_enum(director, node) -> syn::DataEnum {
            skip!(node.enum_token);
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.variants) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_data_struct(director, node) -> syn::DataStruct {
            skip!(node.struct_token);
            crate::Director::direct(&mut director, &node.fields);
            skip!(node.semi_token);
        }
        visit_data_union(director, node) -> syn::DataUnion {
            skip!(node.union_token);
            crate::Director::direct(&mut director, &node.fields);
        }
        visit_derive_input(director, node) -> syn::DeriveInput {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            crate::Director::direct(&mut director, &node.data);
        }
        visit_expr(director, node) -> syn::Expr {
            match node {
                syn::Expr::Array(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Assign(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Async(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Await(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Binary(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Block(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Break(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Call(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Cast(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Closure(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Const(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Continue(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Field(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::ForLoop(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Group(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::If(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Index(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Infer(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Let(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Lit(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Loop(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Macro(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Match(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::MethodCall(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Paren(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Path(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Range(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::RawAddr(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Reference(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Repeat(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Return(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Struct(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Try(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::TryBlock(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Tuple(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Unary(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::Unsafe(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Verbatim(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Expr::While(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::Expr::Yield(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                _ => { }
            }
        }
        visit_expr_array(director, node) -> syn::ExprArray {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.bracket_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_expr_assign(director, node) -> syn::ExprAssign {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &*node.left);
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &*node.right);
        }
        visit_expr_async(director, node) -> syn::ExprAsync {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.async_token);
            skip!(node.capture);
            crate::Director::direct(&mut director, &node.block);
        }
        visit_expr_await(director, node) -> syn::ExprAwait {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &*node.base);
            skip!(node.dot_token);
            skip!(node.await_token);
        }
        visit_expr_binary(director, node) -> syn::ExprBinary {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &*node.left);
            crate::Director::direct(&mut director, &node.op);
            crate::Director::direct(&mut director, &*node.right);
        }
        visit_expr_block(director, node) -> syn::ExprBlock {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.label {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.block);
        }
        visit_expr_break(director, node) -> syn::ExprBreak {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.break_token);
            if let Some(it) = &node.label {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.expr {
                crate::Director::direct(&mut director, &**it);
            }
        }
        visit_expr_call(director, node) -> syn::ExprCall {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &*node.func);
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.args) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_expr_cast(director, node) -> syn::ExprCast {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &*node.expr);
            skip!(node.as_token);
            crate::Director::direct(&mut director, &*node.ty);
        }
        visit_expr_closure(director, node) -> syn::ExprClosure {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.lifetimes {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.constness);
            skip!(node.movability);
            skip!(node.asyncness);
            skip!(node.capture);
            skip!(node.or1_token);
            for el in syn::punctuated::Punctuated::pairs(&node.inputs) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            skip!(node.or2_token);
            crate::Director::direct(&mut director, &node.output);
            crate::Director::direct(&mut director, &*node.body);
        }
        visit_expr_const(director, node) -> syn::ExprConst {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.const_token);
            crate::Director::direct(&mut director, &node.block);
        }
        visit_expr_continue(director, node) -> syn::ExprContinue {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.continue_token);
            if let Some(it) = &node.label {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_expr_field(director, node) -> syn::ExprField {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &*node.base);
            skip!(node.dot_token);
            crate::Director::direct(&mut director, &node.member);
        }
        visit_expr_for_loop(director, node) -> syn::ExprForLoop {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.label {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.for_token);
            crate::Director::direct(&mut director, &*node.pat);
            skip!(node.in_token);
            crate::Director::direct(&mut director, &*node.expr);
            crate::Director::direct(&mut director, &node.body);
        }
        visit_expr_group(director, node) -> syn::ExprGroup {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.group_token);
            crate::Director::direct(&mut director, &*node.expr);
        }
        visit_expr_if(director, node) -> syn::ExprIf {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.if_token);
            crate::Director::direct(&mut director, &*node.cond);
            crate::Director::direct(&mut director, &node.then_branch);
            if let Some(it) = &node.else_branch {
                skip!((it).0);
                crate::Director::direct(&mut director, &*(it).1);
            }
        }
        visit_expr_index(director, node) -> syn::ExprIndex {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &*node.expr);
            skip!(node.bracket_token);
            crate::Director::direct(&mut director, &*node.index);
        }
        visit_expr_infer(director, node) -> syn::ExprInfer {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.underscore_token);
        }
        visit_expr_let(director, node) -> syn::ExprLet {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.let_token);
            crate::Director::direct(&mut director, &*node.pat);
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &*node.expr);
        }
        visit_expr_lit(director, node) -> syn::ExprLit {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.lit);
        }
        visit_expr_loop(director, node) -> syn::ExprLoop {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.label {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.loop_token);
            crate::Director::direct(&mut director, &node.body);
        }
        visit_expr_macro(director, node) -> syn::ExprMacro {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.mac);
        }
        visit_expr_match(director, node) -> syn::ExprMatch {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.match_token);
            crate::Director::direct(&mut director, &*node.expr);
            skip!(node.brace_token);
            for it in &node.arms {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_expr_method_call(director, node) -> syn::ExprMethodCall {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &*node.receiver);
            skip!(node.dot_token);
            crate::Director::direct(&mut director, &node.method);
            if let Some(it) = &node.turbofish {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.args) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_expr_paren(director, node) -> syn::ExprParen {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.paren_token);
            crate::Director::direct(&mut director, &*node.expr);
        }
        visit_expr_path(director, node) -> syn::ExprPath {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.qself {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.path);
        }
        visit_expr_range(director, node) -> syn::ExprRange {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.start {
                crate::Director::direct(&mut director, &**it);
            }
            crate::Director::direct(&mut director, &node.limits);
            if let Some(it) = &node.end {
                crate::Director::direct(&mut director, &**it);
            }
        }
        visit_expr_raw_addr(director, node) -> syn::ExprRawAddr {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.and_token);
            skip!(node.raw);
            crate::Director::direct(&mut director, &node.mutability);
            crate::Director::direct(&mut director, &*node.expr);
        }
        visit_expr_reference(director, node) -> syn::ExprReference {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.and_token);
            skip!(node.mutability);
            crate::Director::direct(&mut director, &*node.expr);
        }
        visit_expr_repeat(director, node) -> syn::ExprRepeat {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.bracket_token);
            crate::Director::direct(&mut director, &*node.expr);
            skip!(node.semi_token);
            crate::Director::direct(&mut director, &*node.len);
        }
        visit_expr_return(director, node) -> syn::ExprReturn {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.return_token);
            if let Some(it) = &node.expr {
                crate::Director::direct(&mut director, &**it);
            }
        }
        visit_expr_struct(director, node) -> syn::ExprStruct {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.qself {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.path);
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.fields) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            skip!(node.dot2_token);
            if let Some(it) = &node.rest {
                crate::Director::direct(&mut director, &**it);
            }
        }
        visit_expr_try(director, node) -> syn::ExprTry {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &*node.expr);
            skip!(node.question_token);
        }
        visit_expr_try_block(director, node) -> syn::ExprTryBlock {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.try_token);
            crate::Director::direct(&mut director, &node.block);
        }
        visit_expr_tuple(director, node) -> syn::ExprTuple {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_expr_unary(director, node) -> syn::ExprUnary {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.op);
            crate::Director::direct(&mut director, &*node.expr);
        }
        visit_expr_unsafe(director, node) -> syn::ExprUnsafe {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.unsafe_token);
            crate::Director::direct(&mut director, &node.block);
        }
        visit_expr_while(director, node) -> syn::ExprWhile {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.label {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.while_token);
            crate::Director::direct(&mut director, &*node.cond);
            crate::Director::direct(&mut director, &node.body);
        }
        visit_expr_yield(director, node) -> syn::ExprYield {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.yield_token);
            if let Some(it) = &node.expr {
                crate::Director::direct(&mut director, &**it);
            }
        }
        visit_field(director, node) -> syn::Field {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            crate::Director::direct(&mut director, &node.mutability);
            if let Some(it) = &node.ident {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &node.ty);
        }
        visit_field_mutability(_director, _node) -> syn::FieldMutability {
            
        }
        visit_field_pat(director, node) -> syn::FieldPat {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.member);
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &*node.pat);
        }
        visit_field_value(director, node) -> syn::FieldValue {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.member);
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &node.expr);
        }
        visit_fields(director, node) -> syn::Fields {
            match node {
                syn::Fields::Named(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Fields::Unnamed(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Fields::Unit => {}
            }
        }
        visit_fields_named(director, node) -> syn::FieldsNamed {
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.named) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_fields_unnamed(director, node) -> syn::FieldsUnnamed {
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.unnamed) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_file(director, node) -> syn::File {
            skip!(node.shebang);
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            for it in &node.items {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_fn_arg(director, node) -> syn::FnArg {
            match node {
                syn::FnArg::Receiver(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::FnArg::Typed(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
            }
        }
        visit_foreign_item(director, node) -> syn::ForeignItem {
            match node {
                syn::ForeignItem::Fn(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::ForeignItem::Static(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::ForeignItem::Type(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::ForeignItem::Macro(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::ForeignItem::Verbatim(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                _ => { }
            }
        }
        visit_foreign_item_fn(director, node) -> syn::ForeignItemFn {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            crate::Director::direct(&mut director, &node.sig);
            skip!(node.semi_token);
        }
        visit_foreign_item_macro(director, node) -> syn::ForeignItemMacro {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.mac);
            skip!(node.semi_token);
        }
        visit_foreign_item_static(director, node) -> syn::ForeignItemStatic {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.static_token);
            crate::Director::direct(&mut director, &node.mutability);
            crate::Director::direct(&mut director, &node.ident);
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &*node.ty);
            skip!(node.semi_token);
        }
        visit_foreign_item_type(director, node) -> syn::ForeignItemType {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.type_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.semi_token);
        }
        visit_generic_argument(director, node) -> syn::GenericArgument {
            match node {
                syn::GenericArgument::Lifetime(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::GenericArgument::Type(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::GenericArgument::Const(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::GenericArgument::AssocType(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::GenericArgument::AssocConst(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::GenericArgument::Constraint(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                _ => { }
            }
        }
        visit_generic_param(director, node) -> syn::GenericParam {
            match node {
                syn::GenericParam::Lifetime(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::GenericParam::Type(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::GenericParam::Const(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
            }
        }
        visit_generics(director, node) -> syn::Generics {
            skip!(node.lt_token);
            for el in syn::punctuated::Punctuated::pairs(&node.params) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            skip!(node.gt_token);
            if let Some(it) = &node.where_clause {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_ident(_director, node) -> proc_macro2::Ident {
            skip!(node.span());
        }
        visit_impl_item(director, node) -> syn::ImplItem {
            match node {
                syn::ImplItem::Const(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::ImplItem::Fn(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::ImplItem::Type(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::ImplItem::Macro(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::ImplItem::Verbatim(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                _ => { }
            }
        }
        visit_impl_item_const(director, node) -> syn::ImplItemConst {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.defaultness);
            skip!(node.const_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &node.ty);
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &node.expr);
            skip!(node.semi_token);
        }
        visit_impl_item_fn(director, node) -> syn::ImplItemFn {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.defaultness);
            crate::Director::direct(&mut director, &node.sig);
            crate::Director::direct(&mut director, &node.block);
        }
        visit_impl_item_macro(director, node) -> syn::ImplItemMacro {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.mac);
            skip!(node.semi_token);
        }
        visit_impl_item_type(director, node) -> syn::ImplItemType {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.defaultness);
            skip!(node.type_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &node.ty);
            skip!(node.semi_token);
        }
        visit_impl_restriction(_director, _node) -> syn::ImplRestriction {
            
        }
        visit_index(_director, node) -> syn::Index {
            skip!(node.index);
            skip!(node.span);
        }
        visit_item(director, node) -> syn::Item {
            match node {
                syn::Item::Const(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Enum(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::ExternCrate(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Fn(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::ForeignMod(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Impl(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Macro(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Mod(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Static(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Struct(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Trait(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::TraitAlias(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Type(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Union(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Use(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Item::Verbatim(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                _ => { }
            }
        }
        visit_item_const(director, node) -> syn::ItemConst {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.const_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &*node.ty);
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &*node.expr);
            skip!(node.semi_token);
        }
        visit_item_enum(director, node) -> syn::ItemEnum {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.enum_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.variants) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_item_extern_crate(director, node) -> syn::ItemExternCrate {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.extern_token);
            skip!(node.crate_token);
            crate::Director::direct(&mut director, &node.ident);
            if let Some(it) = &node.rename {
                skip!((it).0);
                crate::Director::direct(&mut director, &(it).1);
            }
            skip!(node.semi_token);
        }
        visit_item_fn(director, node) -> syn::ItemFn {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            crate::Director::direct(&mut director, &node.sig);
            crate::Director::direct(&mut director, &*node.block);
        }
        visit_item_foreign_mod(director, node) -> syn::ItemForeignMod {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.unsafety);
            crate::Director::direct(&mut director, &node.abi);
            skip!(node.brace_token);
            for it in &node.items {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_item_impl(director, node) -> syn::ItemImpl {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.defaultness);
            skip!(node.unsafety);
            skip!(node.impl_token);
            crate::Director::direct(&mut director, &node.generics);
            if let Some(it) = &node.trait_ {
                skip!((it).0);
                crate::Director::direct(&mut director, &(it).1);
                skip!((it).2);
            }
            crate::Director::direct(&mut director, &*node.self_ty);
            skip!(node.brace_token);
            for it in &node.items {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_item_macro(director, node) -> syn::ItemMacro {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.ident {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.mac);
            skip!(node.semi_token);
        }
        visit_item_mod(director, node) -> syn::ItemMod {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.unsafety);
            skip!(node.mod_token);
            crate::Director::direct(&mut director, &node.ident);
            if let Some(it) = &node.content {
                skip!((it).0);
                for it in &(it).1 {
                    crate::Director::direct(&mut director, it);
                }
            }
            skip!(node.semi);
        }
        visit_item_static(director, node) -> syn::ItemStatic {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.static_token);
            crate::Director::direct(&mut director, &node.mutability);
            crate::Director::direct(&mut director, &node.ident);
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &*node.ty);
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &*node.expr);
            skip!(node.semi_token);
        }
        visit_item_struct(director, node) -> syn::ItemStruct {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.struct_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            crate::Director::direct(&mut director, &node.fields);
            skip!(node.semi_token);
        }
        visit_item_trait(director, node) -> syn::ItemTrait {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.unsafety);
            skip!(node.auto_token);
            if let Some(it) = &node.restriction {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.trait_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.supertraits) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            skip!(node.brace_token);
            for it in &node.items {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_item_trait_alias(director, node) -> syn::ItemTraitAlias {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.trait_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.eq_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            skip!(node.semi_token);
        }
        visit_item_type(director, node) -> syn::ItemType {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.type_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &*node.ty);
            skip!(node.semi_token);
        }
        visit_item_union(director, node) -> syn::ItemUnion {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.union_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            crate::Director::direct(&mut director, &node.fields);
        }
        visit_item_use(director, node) -> syn::ItemUse {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.vis);
            skip!(node.use_token);
            skip!(node.leading_colon);
            crate::Director::direct(&mut director, &node.tree);
            skip!(node.semi_token);
        }
        visit_label(director, node) -> syn::Label {
            crate::Director::direct(&mut director, &node.name);
            skip!(node.colon_token);
        }
        visit_lifetime(director, node) -> syn::Lifetime {
            skip!(node.apostrophe);
            crate::Director::direct(&mut director, &node.ident);
        }
        visit_lifetime_param(director, node) -> syn::LifetimeParam {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.lifetime);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_lit(director, node) -> syn::Lit {
            match node {
                syn::Lit::Str(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Lit::ByteStr(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Lit::CStr(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Lit::Byte(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Lit::Char(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Lit::Int(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Lit::Float(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Lit::Bool(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Lit::Verbatim(_binding_0) => {
                    skip!(_binding_0);
                }
                _ => { }
            }
        }
        visit_lit_bool(_director, node) -> syn::LitBool {
            skip!(node.value);
            skip!(node.span);
        }
        visit_lit_byte(_director, _node) -> syn::LitByte {
        
        }
        visit_lit_byte_str(_director, _node) -> syn::LitByteStr {
        
        }
        visit_lit_cstr(_director, _node) -> syn::LitCStr {
        
        }
        visit_lit_char(_director, _node) -> syn::LitChar {
        
        }
        visit_lit_float(_director, _node) -> syn::LitFloat {
        
        }
        visit_lit_int(_director, _node) -> syn::LitInt {
        
        }
        visit_lit_str(_director, _node) -> syn::LitStr {
        
        }
        visit_local(director, node) -> syn::Local {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.let_token);
            crate::Director::direct(&mut director, &node.pat);
            if let Some(it) = &node.init {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.semi_token);
        }
        visit_local_init(director, node) -> syn::LocalInit {
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &*node.expr);
            if let Some(it) = &node.diverge {
                skip!((it).0);
                crate::Director::direct(&mut director, &*(it).1);
            }
        }
        visit_macro(director, node) -> syn::Macro {
            crate::Director::direct(&mut director, &node.path);
            skip!(node.bang_token);
            crate::Director::direct(&mut director, &node.delimiter);
            crate::Director::direct(&mut director, &node.tokens);
        }
        visit_macro_delimiter(_director, node) -> syn::MacroDelimiter {
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
        visit_member(director, node) -> syn::Member {
            match node {
                syn::Member::Named(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Member::Unnamed(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
            }
        }
        visit_meta(director, node) -> syn::Meta {
            match node {
                syn::Meta::Path(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Meta::List(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Meta::NameValue(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
            }
        }
        visit_meta_list(director, node) -> syn::MetaList {
            crate::Director::direct(&mut director, &node.path);
            crate::Director::direct(&mut director, &node.delimiter);
            crate::Director::direct(&mut director, &node.tokens);
        }
        visit_meta_name_value(director, node) -> syn::MetaNameValue {
            crate::Director::direct(&mut director, &node.path);
            skip!(node.eq_token);
            crate::Director::direct(&mut director, &node.value);
        }
        visit_parenthesized_generic_arguments(director, node) -> syn::ParenthesizedGenericArguments {
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.inputs) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            crate::Director::direct(&mut director, &node.output);
        }
        visit_pat(director, node) -> syn::Pat {
            match node {
                syn::Pat::Const(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Ident(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Lit(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Macro(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Or(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Paren(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Path(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Range(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Reference(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Rest(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Slice(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Struct(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Tuple(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::TupleStruct(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Type(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Verbatim(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Pat::Wild(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                _ => { }
            }
        }
        visit_pat_ident(director, node) -> syn::PatIdent {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.by_ref);
            skip!(node.mutability);
            crate::Director::direct(&mut director, &node.ident);
            if let Some(it) = &node.subpat {
                skip!((it).0);
                crate::Director::direct(&mut director, &*(it).1);
            }
        }
        visit_pat_or(director, node) -> syn::PatOr {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.leading_vert);
            for el in syn::punctuated::Punctuated::pairs(&node.cases) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_pat_paren(director, node) -> syn::PatParen {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.paren_token);
            crate::Director::direct(&mut director, &*node.pat);
        }
        visit_pat_reference(director, node) -> syn::PatReference {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.and_token);
            skip!(node.mutability);
            crate::Director::direct(&mut director, &*node.pat);
        }
        visit_pat_rest(director, node) -> syn::PatRest {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.dot2_token);
        }
        visit_pat_slice(director, node) -> syn::PatSlice {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.bracket_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_pat_struct(director, node) -> syn::PatStruct {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.qself {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.path);
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.fields) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            if let Some(it) = &node.rest {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_pat_tuple(director, node) -> syn::PatTuple {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_pat_tuple_struct(director, node) -> syn::PatTupleStruct {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.qself {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.path);
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_pat_type(director, node) -> syn::PatType {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &*node.pat);
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &*node.ty);
        }
        visit_pat_wild(director, node) -> syn::PatWild {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.underscore_token);
        }
        visit_path(director, node) -> syn::Path {
            skip!(node.leading_colon);
            for el in syn::punctuated::Punctuated::pairs(&node.segments) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_path_arguments(director, node) -> syn::PathArguments {
            match node {
                syn::PathArguments::None => {}
                syn::PathArguments::AngleBracketed(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::PathArguments::Parenthesized(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
            }
        }
        visit_path_segment(director, node) -> syn::PathSegment {
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.arguments);
        }
        visit_pointer_mutability(_director, node) -> syn::PointerMutability {
            match node {
                syn::PointerMutability::Const(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::PointerMutability::Mut(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        visit_precise_capture(director, node) -> syn::PreciseCapture {
            skip!(node.use_token);
            skip!(node.lt_token);
            for el in syn::punctuated::Punctuated::pairs(&node.params) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            skip!(node.gt_token);
        }
        visit_predicate_lifetime(director, node) -> syn::PredicateLifetime {
            crate::Director::direct(&mut director, &node.lifetime);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_predicate_type(director, node) -> syn::PredicateType {
            if let Some(it) = &node.lifetimes {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.bounded_ty);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_qself(director, node) -> syn::QSelf {
            skip!(node.lt_token);
            crate::Director::direct(&mut director, &*node.ty);
            skip!(node.position);
            skip!(node.as_token);
            skip!(node.gt_token);
        }
        visit_range_limits(_director, node) -> syn::RangeLimits {
            match node {
                syn::RangeLimits::HalfOpen(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::RangeLimits::Closed(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        visit_receiver(director, node) -> syn::Receiver {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.reference {
                skip!((it).0);
                if let Some(it) = &(it).1 {
                    crate::Director::direct(&mut director, it);
                }
            }
            skip!(node.mutability);
            skip!(node.self_token);
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &*node.ty);
        }
        visit_return_type(director, node) -> syn::ReturnType {
            match node {
                syn::ReturnType::Default => {}
                syn::ReturnType::Type(_binding_0, _binding_1) => {
                    skip!(_binding_0);
                    crate::Director::direct(&mut director, &**_binding_1);
                }
            }
        }
        visit_signature(director, node) -> syn::Signature {
            skip!(node.constness);
            skip!(node.asyncness);
            skip!(node.unsafety);
            if let Some(it) = &node.abi {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.fn_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.inputs) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            if let Some(it) = &node.variadic {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.output);
        }
        visit_static_mutability(_director, node) -> syn::StaticMutability {
            match node {
                syn::StaticMutability::Mut(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::StaticMutability::None => {}
                _ => { }
            }
        }
        visit_stmt(director, node) -> syn::Stmt {
            match node {
                syn::Stmt::Local(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Stmt::Item(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Stmt::Expr(_binding_0, _binding_1) => {
                    crate::Director::direct(&mut director, _binding_0);
                    skip!(_binding_1);
                }
                syn::Stmt::Macro(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
            }
        }
        visit_stmt_macro(director, node) -> syn::StmtMacro {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.mac);
            skip!(node.semi_token);
        }
        visit_token_stream(_director, _node) -> proc_macro2::TokenStream {
            
        }
        visit_trait_bound(director, node) -> syn::TraitBound {
            skip!(node.paren_token);
            crate::Director::direct(&mut director, &node.modifier);
            if let Some(it) = &node.lifetimes {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.path);
        }
        visit_trait_bound_modifier(_director, node) -> syn::TraitBoundModifier {
            match node {
                syn::TraitBoundModifier::None => {}
                syn::TraitBoundModifier::Maybe(_binding_0) => {
                    skip!(_binding_0);
                }
            }
        }
        visit_trait_item(director, node) -> syn::TraitItem {
            match node {
                syn::TraitItem::Const(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::TraitItem::Fn(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::TraitItem::Type(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::TraitItem::Macro(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::TraitItem::Verbatim(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                _ => { }
            }
        }
        visit_trait_item_const(director, node) -> syn::TraitItemConst {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.const_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.colon_token);
            crate::Director::direct(&mut director, &node.ty);
            if let Some(it) = &node.default {
                skip!((it).0);
                crate::Director::direct(&mut director, &(it).1);
            }
            skip!(node.semi_token);
        }
        visit_trait_item_fn(director, node) -> syn::TraitItemFn {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.sig);
            if let Some(it) = &node.default {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.semi_token);
        }
        visit_trait_item_macro(director, node) -> syn::TraitItemMacro {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.mac);
            skip!(node.semi_token);
        }
        visit_trait_item_type(director, node) -> syn::TraitItemType {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.type_token);
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.generics);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            if let Some(it) = &node.default {
                skip!((it).0);
                crate::Director::direct(&mut director, &(it).1);
            }
            skip!(node.semi_token);
        }
        visit_type(director, node) -> syn::Type {
            match node {
                syn::Type::Array(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::BareFn(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Group(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::ImplTrait(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Infer(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Macro(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Never(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Paren(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Path(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Ptr(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Reference(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Slice(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::TraitObject(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Tuple(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Type::Verbatim(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                _ => { }
            }
        }
        visit_type_array(director, node) -> syn::TypeArray {
            skip!(node.bracket_token);
            crate::Director::direct(&mut director, &*node.elem);
            skip!(node.semi_token);
            crate::Director::direct(&mut director, &node.len);
        }
        visit_type_bare_fn(director, node) -> syn::TypeBareFn {
            if let Some(it) = &node.lifetimes {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.unsafety);
            if let Some(it) = &node.abi {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.fn_token);
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.inputs) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            if let Some(it) = &node.variadic {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.output);
        }
        visit_type_group(director, node) -> syn::TypeGroup {
            skip!(node.group_token);
            crate::Director::direct(&mut director, &*node.elem);
        }
        visit_type_impl_trait(director, node) -> syn::TypeImplTrait {
            skip!(node.impl_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_type_infer(_director, node) -> syn::TypeInfer {
            skip!(node.underscore_token);
        }
        visit_type_macro(director, node) -> syn::TypeMacro {
            crate::Director::direct(&mut director, &node.mac);
        }
        visit_type_never(_director, node) -> syn::TypeNever {
            skip!(node.bang_token);
        }
        visit_type_param(director, node) -> syn::TypeParam {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.ident);
            skip!(node.colon_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
            skip!(node.eq_token);
            if let Some(it) = &node.default {
                crate::Director::direct(&mut director, it);
            }
        }
        visit_type_param_bound(director, node) -> syn::TypeParamBound {
            match node {
                syn::TypeParamBound::Trait(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::TypeParamBound::Lifetime(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::TypeParamBound::PreciseCapture(_binding_0) => {
                    full!(crate::Director::direct(&mut director, _binding_0));
                }
                syn::TypeParamBound::Verbatim(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                _ => { }
            }
        }
        visit_type_paren(director, node) -> syn::TypeParen {
            skip!(node.paren_token);
            crate::Director::direct(&mut director, &*node.elem);
        }
        visit_type_path(director, node) -> syn::TypePath {
            if let Some(it) = &node.qself {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.path);
        }
        visit_type_ptr(director, node) -> syn::TypePtr {
            skip!(node.star_token);
            skip!(node.const_token);
            skip!(node.mutability);
            crate::Director::direct(&mut director, &*node.elem);
        }
        visit_type_reference(director, node) -> syn::TypeReference {
            skip!(node.and_token);
            if let Some(it) = &node.lifetime {
                crate::Director::direct(&mut director, it);
            }
            skip!(node.mutability);
            crate::Director::direct(&mut director, &*node.elem);
        }
        visit_type_slice(director, node) -> syn::TypeSlice {
            skip!(node.bracket_token);
            crate::Director::direct(&mut director, &*node.elem);
        }
        visit_type_trait_object(director, node) -> syn::TypeTraitObject {
            skip!(node.dyn_token);
            for el in syn::punctuated::Punctuated::pairs(&node.bounds) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_type_tuple(director, node) -> syn::TypeTuple {
            skip!(node.paren_token);
            for el in syn::punctuated::Punctuated::pairs(&node.elems) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_un_op(_director, node) -> syn::UnOp {
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
        visit_use_glob(_director, node) -> syn::UseGlob {
            skip!(node.star_token);
        }
        visit_use_group(director, node) -> syn::UseGroup {
            skip!(node.brace_token);
            for el in syn::punctuated::Punctuated::pairs(&node.items) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_use_name(director, node) -> syn::UseName {
            crate::Director::direct(&mut director, &node.ident);
        }
        visit_use_path(director, node) -> syn::UsePath {
            crate::Director::direct(&mut director, &node.ident);
            skip!(node.colon2_token);
            crate::Director::direct(&mut director, &*node.tree);
        }
        visit_use_rename(director, node) -> syn::UseRename {
            crate::Director::direct(&mut director, &node.ident);
            skip!(node.as_token);
            crate::Director::direct(&mut director, &node.rename);
        }
        visit_use_tree(director, node) -> syn::UseTree {
            match node {
                syn::UseTree::Path(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::UseTree::Name(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::UseTree::Rename(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::UseTree::Glob(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::UseTree::Group(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
            }
        }
        visit_variadic(director, node) -> syn::Variadic {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            if let Some(it) = &node.pat {
                crate::Director::direct(&mut director, &*(it).0);
                skip!((it).1);
            }
            skip!(node.dots);
            skip!(node.comma);
        }
        visit_variant(director, node) -> syn::Variant {
            for it in &node.attrs {
                crate::Director::direct(&mut director, it);
            }
            crate::Director::direct(&mut director, &node.ident);
            crate::Director::direct(&mut director, &node.fields);
            if let Some(it) = &node.discriminant {
                skip!((it).0);
                crate::Director::direct(&mut director, &(it).1);
            }
        }
        visit_vis_restricted(director, node) -> syn::VisRestricted {
            skip!(node.pub_token);
            skip!(node.paren_token);
            skip!(node.in_token);
            crate::Director::direct(&mut director, &*node.path);
        }
        visit_visibility(director, node) -> syn::Visibility {
            match node {
                syn::Visibility::Public(_binding_0) => {
                    skip!(_binding_0);
                }
                syn::Visibility::Restricted(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::Visibility::Inherited => {}
            }
        }
        visit_where_clause(director, node) -> syn::WhereClause {
            skip!(node.where_token);
            for el in syn::punctuated::Punctuated::pairs(&node.predicates) {
                let it = el.value();
                crate::Director::direct(&mut director, *it);
            }
        }
        visit_where_predicate(director, node) -> syn::WherePredicate {
            match node {
                syn::WherePredicate::Lifetime(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                syn::WherePredicate::Type(_binding_0) => {
                    crate::Director::direct(&mut director, _binding_0);
                }
                _ => { }
            }
        }
    }
}
