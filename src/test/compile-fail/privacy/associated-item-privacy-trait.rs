// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-tidy-linelength

#![feature(decl_macro, associated_type_defaults)]
#![allow(unused, private_in_public)]

mod priv_trait {
    trait PrivTr {
        fn method(&self) {}
        const CONST: u8 = 0;
        type AssocTy = u8;
    }
    pub struct Pub;
    impl PrivTr for Pub {}
    pub trait PubTr: PrivTr {}

    pub macro mac() {
        let value = <Pub as PrivTr>::method;
        //~^ ERROR type `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as priv_trait::PrivTr>::method}` is private
        value;
        //~^ ERROR type `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as priv_trait::PrivTr>::method}` is private
        Pub.method();
        //~^ ERROR type `for<'r> fn(&'r Self) {<Self as priv_trait::PrivTr>::method}` is private
        <Pub as PrivTr>::CONST;
        //~^ ERROR associated constant `PrivTr::CONST` is private
        let _: <Pub as PrivTr>::AssocTy;
        //~^ ERROR trait `priv_trait::PrivTr` is private
        //~| ERROR trait `priv_trait::PrivTr` is private
        pub type InSignatureTy = <Pub as PrivTr>::AssocTy;
        //~^ ERROR trait `priv_trait::PrivTr` is private
        pub trait InSignatureTr: PrivTr {}
        //~^ ERROR trait `priv_trait::PrivTr` is private
        impl PrivTr for u8 {}
        //~^ ERROR trait `priv_trait::PrivTr` is private
    }
}
fn priv_trait() {
    priv_trait::mac!();
}

mod priv_signature {
    pub trait PubTr {
        fn method(&self, arg: Priv) {}
    }
    struct Priv;
    pub struct Pub;
    impl PubTr for Pub {}

    pub macro mac() {
        let value = <Pub as PubTr>::method;
        //~^ ERROR type `priv_signature::Priv` is private
        value;
        //~^ ERROR type `priv_signature::Priv` is private
        Pub.method(loop {});
        //~^ ERROR type `priv_signature::Priv` is private
    }
}
fn priv_signature() {
    priv_signature::mac!();
}

mod priv_substs {
    pub trait PubTr {
        fn method<T>(&self) {}
    }
    struct Priv;
    pub struct Pub;
    impl PubTr for Pub {}

    pub macro mac() {
        let value = <Pub as PubTr>::method::<Priv>;
        //~^ ERROR type `priv_substs::Priv` is private
        value;
        //~^ ERROR type `priv_substs::Priv` is private
        Pub.method::<Priv>();
        //~^ ERROR type `priv_substs::Priv` is private
    }
}
fn priv_substs() {
    priv_substs::mac!();
}

mod priv_parent_substs {
    pub trait PubTr<T = Priv> {
        fn method(&self) {}
        const CONST: u8 = 0;
        type AssocTy = u8;
    }
    struct Priv;
    pub struct Pub;
    impl PubTr<Priv> for Pub {}
    impl PubTr<Pub> for Priv {}

    pub macro mac() {
        let value = <Pub as PubTr>::method;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        value;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        let value = <Pub as PubTr<_>>::method;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        value;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        Pub.method();
        //~^ ERROR type `priv_parent_substs::Priv` is private

        let value = <Priv as PubTr<_>>::method;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        value;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        Priv.method();
        //~^ ERROR type `priv_parent_substs::Priv` is private

        <Pub as PubTr>::CONST;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        <Pub as PubTr<_>>::CONST;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        <Priv as PubTr<_>>::CONST;
        //~^ ERROR type `priv_parent_substs::Priv` is private

        let _: <Pub as PubTr>::AssocTy;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        //~| ERROR type `priv_parent_substs::Priv` is private
        let _: <Pub as PubTr<_>>::AssocTy;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        //~| ERROR type `priv_parent_substs::Priv` is private
        let _: <Priv as PubTr<_>>::AssocTy;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        //~| ERROR type `priv_parent_substs::Priv` is private

        pub type InSignatureTy1 = <Pub as PubTr>::AssocTy;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        pub type InSignatureTy2 = <Priv as PubTr<Pub>>::AssocTy;
        //~^ ERROR type `priv_parent_substs::Priv` is private
        impl PubTr for u8 {}
        //~^ ERROR type `priv_parent_substs::Priv` is private
    }
}
fn priv_parent_substs() {
    priv_parent_substs::mac!();
}

fn main() {}
