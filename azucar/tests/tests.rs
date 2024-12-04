#![allow(dead_code)]

use azucar::{Index, index, infer, ref_ops};

extern crate std;

use core::ops::Range;
use std::prelude::rust_2021::*;
use std::{dbg, vec};

struct UnCopy {}

impl Default for UnCopy {
	fn default() -> Self {
		Self {}
	}
}

impl azucar::Infer for UnCopy {
	fn infer() -> Self {
		Self {}
	}
}

#[infer]
impl core::ops::Add for &UnCopy {
	type Output = UnCopy;

	#[inline]
	fn add(self, _: Self) -> Self::Output {
		_
	}
}

fn takes_option_vec(a: i32, _: Vec<Option<()>>) -> i32 {
	a + 1
}
fn takes_optional_arg(a: i32, _: Option<()>) -> i32 {
	a + 1
}

#[infer]
fn infer_option() {
	takes_optional_arg(1, _);

	let _x = takes_option_vec(4, dbg!(vec![_; 3]));
	let _y = takes_option_vec(4, dbg!(vec![_, _, _]));
}

fn infer_option_desugar() {
	let _x = takes_option_vec(3, dbg!(vec![azucar::Infer::infer(); 3]));
	let _y = takes_option_vec(
		3,
		dbg!(vec![
			azucar::Infer::infer(),
			azucar::Infer::infer(),
			azucar::Infer::infer()
		]),
	);
}

#[ref_ops]
fn uncopy() {
	let x = UnCopy {};
	let y = x + x;
	let z = x + y + x;
	let _ = z;
}

fn uncopy_desugar() {
	let x = UnCopy {};
	let y = &x + &x;
	let z = &(&x + &y) + &x;
	let _ = z;
}

struct MatRef<'a>(&'a ());
struct MatMut<'a> {
	inner: &'a mut (),
}

impl<'a> core::ops::Index<(usize, usize)> for MatRef<'a> {
	type Output = f32;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		todo!()
	}
}
impl<'a> core::ops::Index<(usize, usize)> for MatMut<'a> {
	type Output = f32;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		todo!()
	}
}
impl<'short, 'a> azucar::Index<'short, (Range<usize>, Range<usize>)> for MatRef<'a> {
	type Output = MatRef<'short>;

	fn index(&'short self, idx: (Range<usize>, Range<usize>)) -> Self::Output {
		todo!()
	}
}
impl<'short, 'a> azucar::IndexMut<'short, (Range<usize>, Range<usize>)> for MatRef<'a> {
	type Output = MatRef<'short>;

	fn index_mut(&'short mut self, idx: (Range<usize>, Range<usize>)) -> Self::Output {
		todo!()
	}
}
impl<'a> azucar::IndexMove<(Range<usize>, Range<usize>)> for MatRef<'a> {
	type Output = MatRef<'a>;

	fn index_move(self, idx: (Range<usize>, Range<usize>)) -> Self::Output {
		todo!()
	}
}

impl<'short, 'a> azucar::Index<'short, (Range<usize>, Range<usize>)> for MatMut<'a> {
	type Output = MatRef<'short>;

	fn index(&'short self, idx: (Range<usize>, Range<usize>)) -> Self::Output {
		todo!()
	}
}
impl<'short, 'a> azucar::IndexMut<'short, (Range<usize>, Range<usize>)> for MatMut<'a> {
	type Output = MatMut<'short>;

	fn index_mut(&'short mut self, idx: (Range<usize>, Range<usize>)) -> Self::Output {
		todo!()
	}
}
impl<'a> azucar::IndexMove<(Range<usize>, Range<usize>)> for MatMut<'a> {
	type Output = MatMut<'a>;

	fn index_move(self, idx: (Range<usize>, Range<usize>)) -> Self::Output {
		todo!()
	}
}

#[index]
fn foo() {
	let mut x = vec![0, 1, 2];

	let y = (*x)[&mut ..]; // &mut [i32]

	let mut m = MatMut { inner: &mut () };

	let regular_index = m[(1, 2)]; // f32

	// doesn't compile because std::ops::Index doesn't support user Ref/Mut types
	// m[(1..3, 0..6)];

	let ref_index = m[&(1..3, 0..6)]; // MatRef<'short>
	let mut_index = m[&mut (1..3, 0..6)]; // MatRef<'short>
	let move_index = m[*(1..3, 0..6)]; // MatMut<'long>
}
