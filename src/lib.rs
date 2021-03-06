
#![cfg_attr(not(feature = "std"), feature(collections))]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate collections;
#[cfg(not(feature = "std"))]
use core::mem;
#[cfg(not(feature = "std"))]
use collections::{String, Vec};
#[cfg(not(feature = "std"))]
use collections::boxed::Box;


#[cfg(feature = "std")]
use std::mem;

/**
 * Leak a piece of data by never calling its destructor
 *
 * Useful for things that are going to be used for the life of the program, but aren't technically
 * static (because they are created in response to arguments, environment, or other
 * configuration/data read at program start).
 *
 * This is a modified version of the proposed rfc: https://github.com/rust-lang/rfcs/pull/1233
 *
 * Notable changes:
 *
 *  - for convenience, leak() is a non-static method
 */
pub trait Leak<T : ?Sized> {
    fn leak<'a>(self) -> &'a mut T where T: 'a;
}

impl<T : ?Sized> Leak<T> for Box<T> {
    fn leak<'a>(self) -> &'a mut T where T: 'a {
        let r = Self::into_raw(self);
        unsafe { &mut *r }
    }
}

/*
 * while String and Vec<T> could have impls in terms of Box, we specialize them because their
 * conversions to Box (into_boxed_slice and into_boxed_str) result in resizing underlying storage
 */

impl Leak<str> for String {
    fn leak<'a>(mut self) -> &'a mut str where Self: 'a {
        let r: *mut str = &mut self[..];
        mem::forget(self);
        unsafe { &mut *r }
    }
}

impl<T> Leak<[T]> for Vec<T> {
    fn leak<'a>(mut self) -> &'a mut [T] where [T]: 'a {
        let r: *mut [T] = &mut self[..];
        mem::forget(self);
        unsafe { &mut *r }
    }
}
