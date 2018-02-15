#![no_std]

#![feature(i128_type)]

extern crate idem;

#[cfg(feature = "libc")]
extern crate libc;

use core::ops::*;
use idem::Zero;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span(i128);

impl Span {
    #[inline]
    pub fn from_ns(ns: i128) -> Self { Span(ns) }
    #[inline]
    pub fn to_ns(self) -> i128 { self.0 }

    #[cfg(feature = "libc")]
    #[inline]
    pub fn to_c_timespec(self) -> Option<::libc::timespec> {
        let b = 1_000_000_000;
        let s = (self.0 / b) as ::libc::time_t;
        if self.0 / b == s as i128 {
            Some(::libc::timespec { tv_sec: s, tv_nsec: (self.0 % b) as _ })
        } else { None }
    }
}

impl Zero for Span {
    const zero: Self = Span(0);
}

impl Add for Span {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self { Span(self.0 + other.0) }
}

impl Sub for Span {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self { Span(self.0 - other.0) }
}

impl AddAssign for Span {
    #[inline]
    fn add_assign(&mut self, other: Self) { self.0 += other.0 }
}

impl SubAssign for Span {
    #[inline]
    fn sub_assign(&mut self, other: Self) { self.0 -= other.0 }
}

#[cfg(feature = "libc")]
impl From<::libc::timespec> for Span {
    #[inline]
    fn from(ts: ::libc::timespec) -> Self {
        Span(1_000_000_000*(ts.tv_sec as i128) + (ts.tv_nsec as i128))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(i128);

impl Point {
    #[cfg(feature = "libc")]
    #[inline]
    pub fn now() -> Self { unsafe {
        let mut ts: ::libc::timespec = ::core::mem::uninitialized();
        ::libc::clock_gettime(::libc::CLOCK_MONOTONIC, &mut ts);
        Point(Span::from(ts).0)
    } }
}

impl Add<Span> for Point {
    type Output = Self;
    #[inline]
    fn add(self, other: Span) -> Self { Point(self.0 + other.0) }
}

impl Sub<Span> for Point {
    type Output = Self;
    #[inline]
    fn sub(self, other: Span) -> Self { Point(self.0 - other.0) }
}

impl Sub for Point {
    type Output = Span;
    #[inline]
    fn sub(self, other: Self) -> Span { Span(self.0 - other.0) }
}

impl AddAssign<Span> for Point {
    #[inline]
    fn add_assign(&mut self, other: Span) { self.0 += other.0 }
}

impl SubAssign<Span> for Point {
    #[inline]
    fn sub_assign(&mut self, other: Span) { self.0 -= other.0 }
}
