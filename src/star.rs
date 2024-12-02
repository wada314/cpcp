// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An extended version of the default `Pair`.
//! This data structure supports star-like shaped 1:N relation:
//! Supports values `t`, `u1`, `u2`, ... `un` where `t` is bidirectionary
//! convertible to each `u_i`s, but each `u_i`s are not convertible to each other.
//!
//! In this data structure, we call `t` the "hub" and `u_i`s the "spokes".

use crate::pair::Pair;
use ::once_list2::OnceList;
use ::std::convert::Infallible;
use ::std::iter;
use ::std::ops::{Deref, DerefMut};

pub struct Star<H, S>(Pair<Hub<H>, Spokes<S>>);
struct Hub<T>(T);
struct Spokes<T>(T, OnceList<T>);

impl<H, S> Star<H, S> {
    pub fn from_hub(hub: H) -> Self {
        Self(Pair::from_left(Hub(hub)))
    }
    pub fn from_spoke(spoke: S) -> Self {
        Self(Pair::from_right(Spokes::new(spoke)))
    }
    /// Constructs from multiple spoke items.
    /// Note this method does not allow an empty spoke list,
    /// so you need to provide at least one spoke.
    pub fn from_spokes(first: S, rest: impl Iterator<Item = S>) -> Self {
        Self(Pair::from_right(Spokes::from_spokes(first, rest)))
    }
}
impl<T> Hub<T> {
    fn new(hub: T) -> Self {
        Self(hub)
    }
}
impl<T> Spokes<T> {
    fn new(spoke: T) -> Self {
        Self(spoke, OnceList::new())
    }
    fn from_spokes(first: T, rest: impl Iterator<Item = T>) -> Self {
        Self(first, rest.collect())
    }
    fn iter(&self) -> impl Iterator<Item = &T> {
        iter::once(&self.0).chain(self.1.iter())
    }
    pub fn select_spoke<S: SpokeSelector<T>>(&self, mut sel: S) -> &T {
        sel.select(&self.0, self.1.iter())
    }
}

pub trait SpokeSelector<T> {
    fn select<'a>(&mut self, first: &'a T, rest: impl Iterator<Item = &'a T>) -> &'a T;
}
pub mod spoke_selector {
    pub struct First;
    impl<T> super::SpokeSelector<T> for First {
        fn select<'a>(&mut self, first: &'a T, _: impl Iterator<Item = &'a T>) -> &'a T {
            first
        }
    }
    pub fn from_reduce_fn<T, F>(f: F) -> impl super::SpokeSelector<T>
    where
        F: for<'a> FnMut(&'a T, &'a T) -> &'a T,
    {
        struct ReduceFn<F>(F);
        impl<T, F> super::SpokeSelector<T> for ReduceFn<F>
        where
            F: for<'a> FnMut(&'a T, &'a T) -> &'a T,
        {
            fn select<'a>(&mut self, first: &'a T, rest: impl Iterator<Item = &'a T>) -> &'a T {
                rest.fold(first, &mut self.0)
            }
        }
        ReduceFn(f)
    }
}

impl<H, S> Star<H, S> {
    pub fn hub<F>(&self, f: F) -> &H
    where
        F: for<'a> FnMut(&'a S, &'a S) -> &'a S,
        S: Into<H>,
    {
        todo!()
    }
}

impl<T> Deref for Hub<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Hub<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
