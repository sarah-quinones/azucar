#![no_std]

pub use azucar_macro::{index, infer, ref_ops};

use core::slice::SliceIndex;

pub trait Infer {
	fn infer() -> Self;
}

impl<T> Infer for Option<T> {
	#[inline]
	fn infer() -> Self {
		None
	}
}
impl Infer for () {
	#[inline]
	fn infer() -> Self {
		()
	}
}

pub trait Index<'a, I, Outlives = &'a Self> {
	type Output;

	fn index(&'a self, idx: I) -> Self::Output;
}

pub trait IndexMut<'a, I, Outlives = &'a Self> {
	type Output;

	fn index_mut(&'a mut self, idx: I) -> Self::Output;
}

pub trait IndexMove<I> {
	type Output;
	fn index_move(self, idx: I) -> Self::Output;
}

impl<'a, T, I: SliceIndex<[T]> + 'a> Index<'a, I> for [T] {
	type Output = &'a I::Output;

	#[inline]
	fn index(&'a self, idx: I) -> Self::Output {
		&self[idx]
	}
}
impl<'a, T, I: SliceIndex<[T]> + 'a> IndexMut<'a, I> for [T] {
	type Output = &'a mut I::Output;

	#[inline]
	fn index_mut(&'a mut self, idx: I) -> Self::Output {
		&mut self[idx]
	}
}
