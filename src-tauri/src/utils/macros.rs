/// Defers evaluation of a block of code until the end of the scope.
#[cfg(feature = "default")]
#[doc(hidden)]
macro_rules! defer {
    ($($body:tt)*) => {
        let _guard = {
            pub struct Guard<F: FnOnce()>(Option<F>);

            impl<F: FnOnce()> Drop for Guard<F> {
                fn drop(&mut self) {
                    (self.0).take().map(|f| f());
                }
            }

            Guard(Some(|| {
                let _ = { $($body)* };
            }))
        };
    };
}

/// Declares unstable items.
#[doc(hidden)]
#[allow(unused_macros)]
macro_rules! cfg_unstable {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "unstable")]
            #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
            $item
        )*
    }
}

/// Declares unstable and default items.
#[doc(hidden)]
#[allow(unused_macros)]
macro_rules! cfg_unstable_default {
    ($($item:item)*) => {
        $(
            #[cfg(all(feature = "default", feature = "unstable"))]
            #[cfg_attr(feature = "docs", doc(unstable))]
            $item
        )*
    }
}

/// Declares Unix-specific items.
#[doc(hidden)]
#[allow(unused_macros)]
macro_rules! cfg_unix {
    ($($item:item)*) => {
        $(
            #[cfg(any(unix, feature = "docs"))]
            #[cfg_attr(feature = "docs", doc(cfg(unix)))]
            $item
        )*
    }
}

/// Declares Windows-specific items.
#[doc(hidden)]
#[allow(unused_macros)]
macro_rules! cfg_windows {
    ($($item:item)*) => {
        $(
            #[cfg(any(windows, feature = "docs"))]
            #[cfg_attr(feature = "docs", doc(cfg(windows)))]
            $item
        )*
    }
}

/// Declares items when the "docs" feature is enabled.
#[doc(hidden)]
#[allow(unused_macros)]
macro_rules! cfg_docs {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "docs")]
            $item
        )*
    }
}

/// Declares items when the "docs" feature is disabled.
#[doc(hidden)]
#[allow(unused_macros)]
macro_rules! cfg_not_docs {
    ($($item:item)*) => {
        $(
            #[cfg(not(feature = "docs"))]
            $item
        )*
    }
}

/// Declares std items.
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! cfg_std {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "std")]
            $item
        )*
    }
}

/// Declares no-std items.
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! cfg_alloc {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "alloc")]
            $item
        )*
    }
}

/// Declares default items.
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! cfg_default {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "default")]
            $item
        )*
    }
}
