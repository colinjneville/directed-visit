macro_rules! node_set {
    ($(#[$attr:meta])* $vis:vis trait $trait_ident:ident $trait_ident_mut:ident { $($fn_ident:ident $fn_ident_mut:ident : $ty:ty),* $(,)? }) => {
        $(#[$attr])*
        $vis trait $trait_ident {
            $(
                #[doc = concat!("Visits [", stringify!($ty), "] nodes")]
                fn $fn_ident<D>(visitor: crate::Visitor<'_, D, Self>, node: &$ty)
                where
                    D: crate::Direct<Self, $ty> + ?Sized,
                {
                    crate::Visitor::visit(visitor, node);
                }
            )*
        }

        $(#[$attr])*
        $vis trait $trait_ident_mut {
            $(
                #[doc = concat!("Visits [", stringify!($ty), "] nodes")]
                fn $fn_ident_mut<D>(visitor: crate::Visitor<'_, D, Self>, node: &mut $ty)
                where
                    D: crate::DirectMut<Self, $ty> + ?Sized,
                {
                    crate::Visitor::visit_mut(visitor, node);
                }
            )*
        }

        $(
            impl<T: $trait_ident + ?Sized> crate::Visit<$ty> for T {
                fn visit<D>(visitor: crate::Visitor<'_, D, Self>, node: &$ty)
                where
                    D: crate::Direct<Self, $ty> + ?Sized,
                {
                    Self::$fn_ident(visitor, node);
                }
            }

            impl<T: $trait_ident_mut + ?Sized> crate::VisitMut<$ty> for T {
                fn visit_mut<D>(visitor: crate::Visitor<'_, D, Self>, node: &mut $ty)
                where
                    D: crate::DirectMut<Self, $ty> + ?Sized,
                {
                    Self::$fn_ident_mut(visitor, node);
                }
            }
        )*
    }
}

node_set! {
    /// A convenience trait for creating [syn] visitors. Implementing this trait will also implement [crate::Visit] for
    /// all 'feature = "full"' syn ast types. Like [syn::visit::Visit], each impl has a no-op default.
    pub trait Full FullMut {
        visit_abi visit_abi_mut: syn::Abi,
        visit_angle_bracketed_generic_arguments visit_angle_bracketed_generic_arguments_mut: syn::AngleBracketedGenericArguments,
        visit_arm visit_arm_mut: syn::Arm,
        visit_assoc_const visit_assoc_const_mut: syn::AssocConst,
        visit_assoc_type visit_assoc_type_mut: syn::AssocType,
        visit_attr_style visit_attr_style_mut: syn::AttrStyle,
        visit_attribute visit_attribute_mut: syn::Attribute,
        visit_bare_fn_arg visit_bare_fn_arg_mut: syn::BareFnArg,
        visit_bare_variadic visit_bare_variadic_mut: syn::BareVariadic,
        visit_bin_op visit_bin_op_mut: syn::BinOp,
        visit_block visit_block_mut: syn::Block,
        visit_bound_lifetimes visit_bound_lifetimes_mut: syn::BoundLifetimes,
        visit_captured_param visit_captured_param_mut: syn::CapturedParam,
        visit_const_param visit_const_param_mut: syn::ConstParam,
        visit_constraint visit_constraint_mut: syn::Constraint,
        visit_data visit_data_mut: syn::Data,
        visit_data_enum visit_data_enum_mut: syn::DataEnum,
        visit_data_struct visit_data_struct_mut: syn::DataStruct,
        visit_data_union visit_data_union_mut: syn::DataUnion,
        visit_derive_input visit_derive_input_mut: syn::DeriveInput,
        visit_expr visit_expr_mut: syn::Expr,
        visit_expr_array visit_expr_array_mut: syn::ExprArray,
        visit_expr_assign visit_expr_assign_mut: syn::ExprAssign,
        visit_expr_async visit_expr_async_mut: syn::ExprAsync,
        visit_expr_await visit_expr_await_mut: syn::ExprAwait,
        visit_expr_binary visit_expr_binary_mut: syn::ExprBinary,
        visit_expr_block visit_expr_block_mut: syn::ExprBlock,
        visit_expr_break visit_expr_break_mut: syn::ExprBreak,
        visit_expr_call visit_expr_call_mut: syn::ExprCall,
        visit_expr_cast visit_expr_cast_mut: syn::ExprCast,
        visit_expr_closure visit_expr_closure_mut: syn::ExprClosure,
        visit_expr_const visit_expr_const_mut: syn::ExprConst,
        visit_expr_continue visit_expr_continue_mut: syn::ExprContinue,
        visit_expr_field visit_expr_field_mut: syn::ExprField,
        visit_expr_for_loop visit_expr_for_loop_mut: syn::ExprForLoop,
        visit_expr_group visit_expr_group_mut: syn::ExprGroup,
        visit_expr_if visit_expr_if_mut: syn::ExprIf,
        visit_expr_index visit_expr_index_mut: syn::ExprIndex,
        visit_expr_infer visit_expr_infer_mut: syn::ExprInfer,
        visit_expr_let visit_expr_let_mut: syn::ExprLet,
        visit_expr_lit visit_expr_lit_mut: syn::ExprLit,
        visit_expr_loop visit_expr_loop_mut: syn::ExprLoop,
        visit_expr_macro visit_expr_macro_mut: syn::ExprMacro,
        visit_expr_match visit_expr_match_mut: syn::ExprMatch,
        visit_expr_method_call visit_expr_method_call_mut: syn::ExprMethodCall,
        visit_expr_paren visit_expr_paren_mut: syn::ExprParen,
        visit_expr_path visit_expr_path_mut: syn::ExprPath,
        visit_expr_range visit_expr_range_mut: syn::ExprRange,
        visit_expr_raw_addr visit_expr_raw_addr_mut: syn::ExprRawAddr,
        visit_expr_reference visit_expr_reference_mut: syn::ExprReference,
        visit_expr_repeat visit_expr_repeat_mut: syn::ExprRepeat,
        visit_expr_return visit_expr_return_mut: syn::ExprReturn,
        visit_expr_struct visit_expr_struct_mut: syn::ExprStruct,
        visit_expr_try visit_expr_try_mut: syn::ExprTry,
        visit_expr_try_block visit_expr_try_block_mut: syn::ExprTryBlock,
        visit_expr_tuple visit_expr_tuple_mut: syn::ExprTuple,
        visit_expr_unary visit_expr_unary_mut: syn::ExprUnary,
        visit_expr_unsafe visit_expr_unsafe_mut: syn::ExprUnsafe,
        visit_expr_while visit_expr_while_mut: syn::ExprWhile,
        visit_expr_yield visit_expr_yield_mut: syn::ExprYield,
        visit_field visit_field_mut: syn::Field,
        visit_field_mutability visit_field_mutability_mut: syn::FieldMutability,
        visit_field_pat visit_field_pat_mut: syn::FieldPat,
        visit_field_value visit_field_value_mut: syn::FieldValue,
        visit_fields visit_fields_mut: syn::Fields,
        visit_fields_named visit_fields_named_mut: syn::FieldsNamed,
        visit_fields_unnamed visit_fields_unnamed_mut: syn::FieldsUnnamed,
        visit_file visit_file_mut: syn::File,
        visit_fn_arg visit_fn_arg_mut: syn::FnArg,
        visit_foreign_item visit_foreign_item_mut: syn::ForeignItem,
        visit_foreign_item_fn visit_foreign_item_fn_mut: syn::ForeignItemFn,
        visit_foreign_item_macro visit_foreign_item_macro_mut: syn::ForeignItemMacro,
        visit_foreign_item_static visit_foreign_item_static_mut: syn::ForeignItemStatic,
        visit_foreign_item_type visit_foreign_item_type_mut: syn::ForeignItemType,
        visit_generic_argument visit_generic_argument_mut: syn::GenericArgument,
        visit_generic_param visit_generic_param_mut: syn::GenericParam,
        visit_generics visit_generics_mut: syn::Generics,
        visit_ident visit_ident_mut: proc_macro2::Ident,
        visit_impl_item visit_impl_item_mut: syn::ImplItem,
        visit_impl_item_const visit_impl_item_const_mut: syn::ImplItemConst,
        visit_impl_item_fn visit_impl_item_fn_mut: syn::ImplItemFn,
        visit_impl_item_macro visit_impl_item_macro_mut: syn::ImplItemMacro,
        visit_impl_item_type visit_impl_item_type_mut: syn::ImplItemType,
        visit_impl_restriction visit_impl_restriction_mut: syn::ImplRestriction,
        visit_index visit_index_mut: syn::Index,
        visit_item visit_item_mut: syn::Item,
        visit_item_const visit_item_const_mut: syn::ItemConst,
        visit_item_enum visit_item_enum_mut: syn::ItemEnum,
        visit_item_extern_crate visit_item_extern_crate_mut: syn::ItemExternCrate,
        visit_item_fn visit_item_fn_mut: syn::ItemFn,
        visit_item_foreign_mod visit_item_foreign_mod_mut: syn::ItemForeignMod,
        visit_item_impl visit_item_impl_mut: syn::ItemImpl,
        visit_item_macro visit_item_macro_mut: syn::ItemMacro,
        visit_item_mod visit_item_mod_mut: syn::ItemMod,
        visit_item_static visit_item_static_mut: syn::ItemStatic,
        visit_item_struct visit_item_struct_mut: syn::ItemStruct,
        visit_item_trait visit_item_trait_mut: syn::ItemTrait,
        visit_item_trait_alias visit_item_trait_alias_mut: syn::ItemTraitAlias,
        visit_item_type visit_item_type_mut: syn::ItemType,
        visit_item_union visit_item_union_mut: syn::ItemUnion,
        visit_item_use visit_item_use_mut: syn::ItemUse,
        visit_label visit_label_mut: syn::Label,
        visit_lifetime visit_lifetime_mut: syn::Lifetime,
        visit_lifetime_param visit_lifetime_param_mut: syn::LifetimeParam,
        visit_lit visit_lit_mut: syn::Lit,
        visit_lit_bool visit_lit_bool_mut: syn::LitBool,
        visit_lit_byte visit_lit_byte_mut: syn::LitByte,
        visit_lit_byte_str visit_lit_byte_str_mut: syn::LitByteStr,
        visit_lit_cstr visit_lit_cstr_mut: syn::LitCStr,
        visit_lit_char visit_lit_char_mut: syn::LitChar,
        visit_lit_float visit_lit_float_mut: syn::LitFloat,
        visit_lit_int visit_lit_int_mut: syn::LitInt,
        visit_lit_str visit_lit_str_mut: syn::LitStr,
        visit_local visit_local_mut: syn::Local,
        visit_local_init visit_local_init_mut: syn::LocalInit,
        visit_macro visit_macro_mut: syn::Macro,
        visit_macro_delimiter visit_macro_delimiter_mut: syn::MacroDelimiter,
        visit_member visit_member_mut: syn::Member,
        visit_meta visit_meta_mut: syn::Meta,
        visit_meta_list visit_meta_list_mut: syn::MetaList,
        visit_meta_name_value visit_meta_name_value_mut: syn::MetaNameValue,
        visit_parenthesized_generic_arguments visit_parenthesized_generic_arguments_mut: syn::ParenthesizedGenericArguments,
        visit_pat visit_pat_mut: syn::Pat,
        visit_pat_ident visit_pat_ident_mut: syn::PatIdent,
        visit_pat_or visit_pat_or_mut: syn::PatOr,
        visit_pat_paren visit_pat_paren_mut: syn::PatParen,
        visit_pat_reference visit_pat_reference_mut: syn::PatReference,
        visit_pat_rest visit_pat_rest_mut: syn::PatRest,
        visit_pat_slice visit_pat_slice_mut: syn::PatSlice,
        visit_pat_struct visit_pat_struct_mut: syn::PatStruct,
        visit_pat_tuple visit_pat_tuple_mut: syn::PatTuple,
        visit_pat_tuple_struct visit_pat_tuple_struct_mut: syn::PatTupleStruct,
        visit_pat_type visit_pat_type_mut: syn::PatType,
        visit_pat_wild visit_pat_wild_mut: syn::PatWild,
        visit_path visit_path_mut: syn::Path,
        visit_path_arguments visit_path_arguments_mut: syn::PathArguments,
        visit_path_segment visit_path_segment_mut: syn::PathSegment,
        visit_pointer_mutability visit_pointer_mutability_mut: syn::PointerMutability,
        visit_precise_capture visit_precise_capture_mut: syn::PreciseCapture,
        visit_predicate_lifetime visit_predicate_lifetime_mut: syn::PredicateLifetime,
        visit_predicate_type visit_predicate_type_mut: syn::PredicateType,
        visit_qself visit_qself_mut: syn::QSelf,
        visit_range_limits visit_range_limits_mut: syn::RangeLimits,
        visit_receiver visit_receiver_mut: syn::Receiver,
        visit_return_type visit_return_type_mut: syn::ReturnType,
        visit_signature visit_signature_mut: syn::Signature,
        // visit_span visit_span_mut: proc_macro2::Span,
        visit_static_mutability visit_static_mutability_mut: syn::StaticMutability,
        visit_stmt visit_stmt_mut: syn::Stmt,
        visit_stmt_macro visit_stmt_macro_mut: syn::StmtMacro,
        visit_token_stream visit_token_stream_mut: proc_macro2::TokenStream,
        visit_trait_bound visit_trait_bound_mut: syn::TraitBound,
        visit_trait_bound_modifier visit_trait_bound_modifier_mut: syn::TraitBoundModifier,
        visit_trait_item visit_trait_item_mut: syn::TraitItem,
        visit_trait_item_const visit_trait_item_const_mut: syn::TraitItemConst,
        visit_trait_item_fn visit_trait_item_fn_mut: syn::TraitItemFn,
        visit_trait_item_macro visit_trait_item_macro_mut: syn::TraitItemMacro,
        visit_trait_item_type visit_trait_item_type_mut: syn::TraitItemType,
        visit_type visit_type_mut: syn::Type,
        visit_type_array visit_type_array_mut: syn::TypeArray,
        visit_type_bare_fn visit_type_bare_fn_mut: syn::TypeBareFn,
        visit_type_group visit_type_group_mut: syn::TypeGroup,
        visit_type_impl_trait visit_type_impl_trait_mut: syn::TypeImplTrait,
        visit_type_infer visit_type_infer_mut: syn::TypeInfer,
        visit_type_macro visit_type_macro_mut: syn::TypeMacro,
        visit_type_never visit_type_never_mut: syn::TypeNever,
        visit_type_param visit_type_param_mut: syn::TypeParam,
        visit_type_param_bound visit_type_param_bound_mut: syn::TypeParamBound,
        visit_type_paren visit_type_paren_mut: syn::TypeParen,
        visit_type_path visit_type_path_mut: syn::TypePath,
        visit_type_ptr visit_type_ptr_mut: syn::TypePtr,
        visit_type_reference visit_type_reference_mut: syn::TypeReference,
        visit_type_slice visit_type_slice_mut: syn::TypeSlice,
        visit_type_trait_object visit_type_trait_object_mut: syn::TypeTraitObject,
        visit_type_tuple visit_type_tuple_mut: syn::TypeTuple,
        visit_un_op visit_un_op_mut: syn::UnOp,
        visit_use_glob visit_use_glob_mut: syn::UseGlob,
        visit_use_group visit_use_group_mut: syn::UseGroup,
        visit_use_name visit_use_name_mut: syn::UseName,
        visit_use_path visit_use_path_mut: syn::UsePath,
        visit_use_rename visit_use_rename_mut: syn::UseRename,
        visit_use_tree visit_use_tree_mut: syn::UseTree,
        visit_variadic visit_variadic_mut: syn::Variadic,
        visit_variant visit_variant_mut: syn::Variant,
        visit_vis_restricted visit_vis_restricted_mut: syn::VisRestricted,
        visit_visibility visit_visibility_mut: syn::Visibility,
        visit_where_clause visit_where_clause_mut: syn::WhereClause,
        visit_where_predicate visit_where_predicate_mut: syn::WherePredicate,
        // Added nodes
        visit_generics_enter visit_generics_enter_mut: crate::syn::GenericsEnter,
        visit_generics_exit visit_generics_exit_mut: crate::syn::GenericsExit,
    }
}
