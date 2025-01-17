/// Helper macro for defining [`Vector3`](super::VECTOR3) objects
#[macro_export]
macro_rules! V {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::Vector3::new($x, $y, $z)
    };
}
/// Macro for defining ctype wrappers
///
/// Adapted from [comment](https://github.com/dtolnay/cxx/issues/254#issuecomment-747860504) by Adrian Taylor
#[macro_export]
macro_rules! ctype_wrapper {
    ($r:ident, $c:ty) => {
        #[doc = "Newtype wrapper for `"]
        #[doc = stringify!($r)]
        #[doc = "` as a [`"]
        #[doc = stringify!($c)]
        #[doc = "`]"]
        #[derive(Debug, Eq, Clone, PartialEq, Hash, Default, Copy)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct $r(pub $c);
        unsafe impl cxx::ExternType for $r {
            type Id = cxx::type_id!($r);
            type Kind = cxx::kind::Trivial;
        }
    };
    ($r:ident, $c:ty, $nice_name:ident) => {
        #[doc = "Newtype wrapper for `"]
        #[doc = stringify!($r)]
        #[doc = "` as a ["]
        #[doc = stringify!($c)]
        #[doc = "]"]
        #[derive(Debug, Eq, Clone, PartialEq, Hash, Default, Copy)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct $r(pub $c);
        unsafe impl cxx::ExternType for $r {
            type Id = cxx::type_id!($r);
            type Kind = cxx::kind::Trivial;
        }
        #[doc = "Type alias for ["]
        #[doc = stringify!($r)]
        #[doc = "]"]
        pub type $nice_name = $r;
    };
}

/// Helper macro for defining entry point into a Vessel addon
///
/// Inspired by emgre's [orbiter-rs](https://github.com/emgre/orbiter-rs/blob/107068c6e66564b9dff86c8b964515da9771a3af/orbiter/src/lib.rs#L37)
///
/// The macro should contain two function blocks `init()` and `exit()`. The `init` function takes one argument,
/// an `[SDKVessel]` instance. It is expected that this `[SDKVessel]` instance is passed to and stored in the Rust code implementing the addon.
///
/// The `exit` function is called at the end of a simulation session and can be used to perform cleanup functions.
///
/// Example:
/// ```no_run
/// fn init(vessel)
/// {
///     Surveyor::new(vessel)
/// }
/// fn exit() {}
/// ```
#[macro_export]
macro_rules! init_vessel {
    (fn init($vessel_ident:ident) $init_block:block fn exit() $body_exit:block) => {
        #[no_mangle]
        pub extern "C" fn ovcInit (hvessel: $crate::OBJHANDLE, flightmodel: i32) -> *mut $crate::ffi::VESSEL
        {
            // The init function pointer gets stored in the C++ object
            // and gets triggered on clbkSetClassCaps() before the trait method
            unsafe { $crate::ffi::vessel_ovcInit(hvessel, flightmodel, vessel_init) }
        }
        #[no_mangle]
        pub extern "C" fn ovcExit (vessel: *mut $crate::ffi::VESSEL)
        {
            $body_exit
            unsafe { $crate::ffi::vessel_ovcExit(vessel); }
        }
        pub fn vessel_init<'a> (vessel: std::pin::Pin<&'static mut $crate::ffi::VesselContext>) -> Box<dyn $crate::OrbiterVessel + 'static>
        {
            let $vessel_ident = vessel;
            let spacecraft = {
                $init_block
            };
            return Box::new(spacecraft);
        }
    };
}

/// Displays a string in the lower left corner of the viewport.
///
/// This macro uses the exact same parameters as the [`format!`] macro of the
/// standard library.
///
/// Due to how Orbiter handles this string, its length is limited to 255 characters.
/// The Rust code truncates the formatted string to 255 characters to make sure that
/// no buffer overflow occurs.
///
/// **This function should only be used for debugging purposes.**
///
/// # Examples
///
/// ```
/// use orbiter_rs::debug_string;
///
/// let some_value = 42;
/// debug_string!("This is my value: {}", 42);
/// ```
///
/// [`format!`]: https://doc.rust-lang.org/std/fmt/index.html
///
/// Macro adapted from Émile Grégoire's version at
/// <https://github.com/emgre/orbiter-rs/blob/107068c6e66564b9dff86c8b964515da9771a3af/orbiter/src/lib.rs#L96>
#[macro_export]
macro_rules! debug_string {
    ($($args:tt)+) => {
        $crate::ODebug(format!($($args)*));
    }
}
