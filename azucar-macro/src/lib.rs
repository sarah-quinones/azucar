use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{Expr, Ident, Token};

struct InferVisit;
struct RefOpsVisit;
struct IndexVisit;

fn visit_expr_macro<V: VisitMut>(v: &mut V, i: &mut syn::ExprMacro) {
	if let Ok(mut mac) = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated
		.parse2(i.mac.tokens.clone())
	{
		for e in &mut mac {
			v.visit_expr_mut(e);
		}
		i.mac.tokens = quote::quote!(#mac);
		return;
	}

	if let Ok(mut mac) = syn::punctuated::Punctuated::<Expr, Token![;]>::parse_terminated
		.parse2(i.mac.tokens.clone())
	{
		for e in &mut mac {
			v.visit_expr_mut(e);
		}
		i.mac.tokens = quote::quote!(#mac);
		return;
	}
}

impl VisitMut for InferVisit {
	fn visit_expr_macro_mut(&mut self, i: &mut syn::ExprMacro) {
		visit_expr_macro(self, i)
	}

	fn visit_angle_bracketed_generic_arguments_mut(
		&mut self,
		_: &mut syn::AngleBracketedGenericArguments,
	) {
	}

	fn visit_generic_argument_mut(&mut self, _: &mut syn::GenericArgument) {}

	fn visit_generic_param_mut(&mut self, _: &mut syn::GenericParam) {}

	fn visit_generics_mut(&mut self, _: &mut syn::Generics) {}

	fn visit_parenthesized_generic_arguments_mut(
		&mut self,
		_: &mut syn::ParenthesizedGenericArguments,
	) {
	}

	fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
		match i {
			Expr::Infer(syn::ExprInfer {
				attrs,
				underscore_token,
			}) => {
				let krate = syn::Stmt::Item(syn::Item::ExternCrate(syn::ItemExternCrate {
					attrs: vec![],
					vis: syn::Visibility::Inherited,
					extern_token: Default::default(),
					crate_token: Default::default(),
					ident: Ident::new("azucar", proc_macro::Span::call_site().into()),
					rename: None,
					semi_token: Default::default(),
				}));

				let call = Expr::Call(syn::ExprCall {
					attrs: core::mem::take(attrs),
					func: Box::new(Expr::Path(syn::ExprPath {
						attrs: vec![],
						qself: None,
						path: syn::Path {
							leading_colon: Some(Default::default()),
							segments: [
								syn::PathSegment {
									ident: Ident::new("azucar", underscore_token.span),
									arguments: syn::PathArguments::None,
								},
								syn::PathSegment {
									ident: Ident::new("Infer", underscore_token.span),
									arguments: syn::PathArguments::None,
								},
								syn::PathSegment {
									ident: Ident::new("infer", underscore_token.span),
									arguments: syn::PathArguments::None,
								},
							]
							.into_iter()
							.collect(),
						},
					})),
					paren_token: Default::default(),
					args: syn::punctuated::Punctuated::new(),
				});

				*i = Expr::Block(syn::ExprBlock {
					attrs: vec![],
					label: None,
					block: syn::Block {
						brace_token: Default::default(),
						stmts: vec![krate, syn::Stmt::Expr(call, None)],
					},
				})
			},
			_ => {
				syn::visit_mut::visit_expr_mut(self, i);
			},
		}
	}
}
impl VisitMut for RefOpsVisit {
	fn visit_expr_macro_mut(&mut self, i: &mut syn::ExprMacro) {
		visit_expr_macro(self, i)
	}

	fn visit_expr_unary_mut(&mut self, i: &mut syn::ExprUnary) {
		syn::visit_mut::visit_expr_unary_mut(self, i);

		match i.op {
			syn::UnOp::Not(_) | syn::UnOp::Neg(_) => {
				*i.expr = Expr::Reference(syn::ExprReference {
					attrs: vec![],
					and_token: Default::default(),
					mutability: None,
					expr: Box::new(core::mem::replace(&mut i.expr, Expr::PLACEHOLDER)),
				});
			},
			_ => {},
		};
	}

	fn visit_expr_binary_mut(&mut self, i: &mut syn::ExprBinary) {
		syn::visit_mut::visit_expr_binary_mut(self, i);

		match i.op {
			syn::BinOp::Add(_)
			| syn::BinOp::Sub(_)
			| syn::BinOp::Mul(_)
			| syn::BinOp::Div(_)
			| syn::BinOp::Rem(_)
			| syn::BinOp::And(_)
			| syn::BinOp::Or(_)
			| syn::BinOp::BitXor(_)
			| syn::BinOp::BitAnd(_)
			| syn::BinOp::BitOr(_)
			| syn::BinOp::Shl(_)
			| syn::BinOp::Shr(_) => {
				for e in [&mut i.left, &mut i.right] {
					**e = Expr::Reference(syn::ExprReference {
						attrs: vec![],
						and_token: Default::default(),
						mutability: None,
						expr: Box::new(core::mem::replace(e, Expr::PLACEHOLDER)),
					});
				}
			},
			_ => {},
		}
	}
}

impl VisitMut for IndexVisit {
	fn visit_expr_macro_mut(&mut self, i: &mut syn::ExprMacro) {
		visit_expr_macro(self, i);
	}

	fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
		syn::visit_mut::visit_expr_mut(self, i);

		match i {
			Expr::Index(e) => match &mut *e.index {
				Expr::Unary(syn::ExprUnary { attrs, op, expr }) => match op {
					syn::UnOp::Deref(_) => {
						let index = core::mem::replace(&mut **expr, Expr::PLACEHOLDER);
						let expr = core::mem::replace(&mut *e.expr, Expr::PLACEHOLDER);

						let krate = syn::Stmt::Item(syn::Item::ExternCrate(syn::ItemExternCrate {
							attrs: vec![],
							vis: syn::Visibility::Inherited,
							extern_token: Default::default(),
							crate_token: Default::default(),
							ident: Ident::new("azucar", proc_macro::Span::call_site().into()),
							rename: None,
							semi_token: Default::default(),
						}));

						let call = Expr::Call(syn::ExprCall {
							attrs: core::mem::take(attrs),
							func: Box::new(Expr::Path(syn::ExprPath {
								attrs: vec![],
								qself: None,
								path: syn::Path {
									leading_colon: Some(Default::default()),
									segments: [
										syn::PathSegment {
											ident: Ident::new(
												"azucar",
												e.bracket_token.span.span(),
											),
											arguments: syn::PathArguments::None,
										},
										syn::PathSegment {
											ident: Ident::new(
												"IndexMove",
												e.bracket_token.span.span(),
											),
											arguments: syn::PathArguments::None,
										},
										syn::PathSegment {
											ident: Ident::new(
												"index_move",
												e.bracket_token.span.span(),
											),
											arguments: syn::PathArguments::None,
										},
									]
									.into_iter()
									.collect(),
								},
							})),
							paren_token: Default::default(),
							args: [expr, index].into_iter().collect(),
						});

						*i = Expr::Block(syn::ExprBlock {
							attrs: vec![],
							label: None,
							block: syn::Block {
								brace_token: Default::default(),
								stmts: vec![krate, syn::Stmt::Expr(call, None)],
							},
						})
					},
					_ => {},
				},
				Expr::Reference(syn::ExprReference {
					attrs,
					and_token,
					mutability,
					expr,
				}) => {
					let index = core::mem::replace(&mut **expr, Expr::PLACEHOLDER);

					let expr = Box::new(core::mem::replace(&mut *e.expr, Expr::PLACEHOLDER));
					let is_mut = mutability.is_some();

					let expr = Expr::Reference(syn::ExprReference {
						attrs: core::mem::take(attrs),
						and_token: *and_token,
						mutability: *mutability,
						expr,
					});

					let krate = syn::Stmt::Item(syn::Item::ExternCrate(syn::ItemExternCrate {
						attrs: vec![],
						vis: syn::Visibility::Inherited,
						extern_token: Default::default(),
						crate_token: Default::default(),
						ident: Ident::new("azucar", proc_macro::Span::call_site().into()),
						rename: None,
						semi_token: Default::default(),
					}));

					let call = Expr::Call(syn::ExprCall {
						attrs: core::mem::take(attrs),
						func: Box::new(Expr::Path(syn::ExprPath {
							attrs: vec![],
							qself: None,
							path: syn::Path {
								leading_colon: Some(Default::default()),
								segments: [
									syn::PathSegment {
										ident: Ident::new("azucar", e.bracket_token.span.span()),
										arguments: syn::PathArguments::None,
									},
									syn::PathSegment {
										ident: Ident::new(
											if is_mut { "IndexMut" } else { "Index" },
											e.bracket_token.span.span(),
										),
										arguments: syn::PathArguments::None,
									},
									syn::PathSegment {
										ident: Ident::new(
											if is_mut { "index_mut" } else { "index" },
											e.bracket_token.span.span(),
										),
										arguments: syn::PathArguments::None,
									},
								]
								.into_iter()
								.collect(),
							},
						})),
						paren_token: Default::default(),
						args: [expr, index].into_iter().collect(),
					});

					*i = Expr::Block(syn::ExprBlock {
						attrs: vec![],
						label: None,
						block: syn::Block {
							brace_token: Default::default(),
							stmts: vec![krate, syn::Stmt::Expr(call, None)],
						},
					})
				},
				_ => {},
			},
			_ => {},
		}
	}
}

#[proc_macro_attribute]
pub fn ref_ops(_: TokenStream, item: TokenStream) -> TokenStream {
	let mut item = syn::parse_macro_input!(item as syn::Item);

	RefOpsVisit.visit_item_mut(&mut item);

	quote::quote!(#item).into()
}

#[proc_macro_attribute]
pub fn infer(_: TokenStream, item: TokenStream) -> TokenStream {
	let mut item = syn::parse_macro_input!(item as syn::Item);

	InferVisit.visit_item_mut(&mut item);

	quote::quote!(#item).into()
}

#[proc_macro_attribute]
pub fn index(_: TokenStream, item: TokenStream) -> TokenStream {
	let mut item = syn::parse_macro_input!(item as syn::Item);

	IndexVisit.visit_item_mut(&mut item);

	quote::quote!(#item).into()
}
