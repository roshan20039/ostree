// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{translate::*};

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord)]
    pub struct CollectionRef(Boxed<ffi::OstreeCollectionRef>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::ostree_collection_ref_get_type(), ptr as *mut _) as *mut ffi::OstreeCollectionRef,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::ostree_collection_ref_get_type(), ptr as *mut _),
        type_ => || ffi::ostree_collection_ref_get_type(),
    }
}

impl CollectionRef {
    #[doc(alias = "ostree_collection_ref_new")]
    pub fn new(collection_id: Option<&str>, ref_name: &str) -> CollectionRef {
        unsafe {
            from_glib_full(ffi::ostree_collection_ref_new(collection_id.to_glib_none().0, ref_name.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_collection_ref_equal")]
     fn equal(&self, ref2: &CollectionRef) -> bool {
        unsafe {
            from_glib(ffi::ostree_collection_ref_equal(ToGlibPtr::<*const ffi::OstreeCollectionRef>::to_glib_none(self).0 as glib::ffi::gconstpointer, ToGlibPtr::<*const ffi::OstreeCollectionRef>::to_glib_none(ref2).0 as glib::ffi::gconstpointer))
        }
    }

    #[doc(alias = "ostree_collection_ref_hash")]
     fn hash(&self) -> u32 {
        unsafe {
            ffi::ostree_collection_ref_hash(ToGlibPtr::<*const ffi::OstreeCollectionRef>::to_glib_none(self).0 as glib::ffi::gconstpointer)
        }
    }
}

impl PartialEq for CollectionRef {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for CollectionRef {}

impl std::hash::Hash for CollectionRef {
    #[inline]
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        std::hash::Hash::hash(&self.hash(), state)
    }
}
