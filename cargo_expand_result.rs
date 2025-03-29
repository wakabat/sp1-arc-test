#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::sync::Arc;
use rkyv::Archive;
#[archive_attr(derive(Debug))]
pub struct Inner {
    pub a: usize,
    pub b: usize,
}
#[automatically_derived]
///An archived [`Inner`]
#[repr(C)]
pub struct ArchivedInner
where
    usize: ::rkyv::Archive,
    usize: ::rkyv::Archive,
{
    ///The archived counterpart of [`Inner::a`]
    pub a: ::rkyv::Archived<usize>,
    ///The archived counterpart of [`Inner::b`]
    pub b: ::rkyv::Archived<usize>,
}
#[automatically_derived]
impl ::core::fmt::Debug for ArchivedInner
where
    usize: ::rkyv::Archive,
    usize: ::rkyv::Archive,
{
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "ArchivedInner",
            "a",
            &self.a,
            "b",
            &&self.b,
        )
    }
}
#[automatically_derived]
///The resolver for an archived [`Inner`]
pub struct InnerResolver
where
    usize: ::rkyv::Archive,
    usize: ::rkyv::Archive,
{
    a: ::rkyv::Resolver<usize>,
    b: ::rkyv::Resolver<usize>,
}
#[automatically_derived]
const _: () = {
    use ::core::marker::PhantomData;
    use ::rkyv::{out_field, Archive, Archived};
    impl Archive for Inner
    where
        usize: ::rkyv::Archive,
        usize: ::rkyv::Archive,
    {
        type Archived = ArchivedInner;
        type Resolver = InnerResolver;
        #[allow(clippy::unit_arg)]
        #[inline]
        unsafe fn resolve(
            &self,
            pos: usize,
            resolver: Self::Resolver,
            out: *mut Self::Archived,
        ) {
            let (fp, fo) = {
                #[allow(unused_unsafe)]
                unsafe {
                    let fo = &raw mut (*out).a;
                    (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                }
            };
            ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
            let (fp, fo) = {
                #[allow(unused_unsafe)]
                unsafe {
                    let fo = &raw mut (*out).b;
                    (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                }
            };
            ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
        }
    }
};
#[automatically_derived]
impl ::core::default::Default for Inner {
    #[inline]
    fn default() -> Inner {
        Inner {
            a: ::core::default::Default::default(),
            b: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Inner {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Inner",
            "a",
            &self.a,
            "b",
            &&self.b,
        )
    }
}
#[archive_attr(derive(Debug))]
pub struct Outer {
    pub inner: Arc<Inner>,
}
#[automatically_derived]
///An archived [`Outer`]
#[repr(C)]
pub struct ArchivedOuter
where
    Arc<Inner>: ::rkyv::Archive,
{
    ///The archived counterpart of [`Outer::inner`]
    pub inner: ::rkyv::Archived<Arc<Inner>>,
}
#[automatically_derived]
impl ::core::fmt::Debug for ArchivedOuter
where
    Arc<Inner>: ::rkyv::Archive,
{
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "ArchivedOuter",
            "inner",
            &&self.inner,
        )
    }
}
#[automatically_derived]
///The resolver for an archived [`Outer`]
pub struct OuterResolver
where
    Arc<Inner>: ::rkyv::Archive,
{
    inner: ::rkyv::Resolver<Arc<Inner>>,
}
#[automatically_derived]
const _: () = {
    use ::core::marker::PhantomData;
    use ::rkyv::{out_field, Archive, Archived};
    impl Archive for Outer
    where
        Arc<Inner>: ::rkyv::Archive,
    {
        type Archived = ArchivedOuter;
        type Resolver = OuterResolver;
        #[allow(clippy::unit_arg)]
        #[inline]
        unsafe fn resolve(
            &self,
            pos: usize,
            resolver: Self::Resolver,
            out: *mut Self::Archived,
        ) {
            let (fp, fo) = {
                #[allow(unused_unsafe)]
                unsafe {
                    let fo = &raw mut (*out).inner;
                    (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                }
            };
            ::rkyv::Archive::resolve((&self.inner), pos + fp, resolver.inner, fo);
        }
    }
};
#[automatically_derived]
impl ::core::default::Default for Outer {
    #[inline]
    fn default() -> Outer {
        Outer {
            inner: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Outer {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "Outer",
            "inner",
            &&self.inner,
        )
    }
}
