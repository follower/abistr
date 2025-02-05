#[cfg(any(doc, test))] use crate::*;


#[cfg(test)] macro_rules! assert_abi_compatible {
    ( $left:ty, $right:ty ) => {{
        assert!(
            std::mem::size_of::<$left>() == std::mem::size_of::<$right>(),
            "ABIs not compatible: size_of::<{}>() == {} != {} == size_of::<{}>()",
            stringify!($left), std::mem::size_of::<$left>(), std::mem::size_of::<$right>(), stringify!($right)
        );
        assert!(
            std::mem::align_of::<$left>() == std::mem::align_of::<$right>(),
            "ABIs not compatible: align_of::<{}>() == {} != {} == align_of::<{}>()",
            stringify!($left), std::mem::align_of::<$left>(), std::mem::align_of::<$right>(), stringify!($right)
        );
    }};
}

/// Create a <code>&[CStrNonNull]</code> literal at compile time
#[cfg(doc)]
#[macro_export]
macro_rules! cstr {
    ( $string:literal ) => {
        $crate::abistr_macros::cstr8_impl!(($crate) $string)
    };
}

/// Create a <code>&[CStrNonNull]</code> literal at compile time
#[cfg(not(doc))] // use wildcards for better error messages from proc macro
#[macro_export]
macro_rules! cstr {
    ( $($tt:tt)+ ) => {
        $crate::abistr_macros::cstr8_impl!(($crate) $($tt)+)
    };
}

/// Create a <code>&[CStrNonNull]<[u8]></code> literal at compile time
#[cfg(doc)]
#[macro_export]
macro_rules! cstr8 {
    ( $string:literal ) => {
        $crate::abistr_macros::cstr8_impl!(($crate) $string)
    };
}

/// Create a <code>&[CStrNonNull]<[u8]></code> literal at compile time
#[cfg(not(doc))] // use wildcards for better error messages from proc macro
#[macro_export]
macro_rules! cstr8 {
    ( $($tt:tt)+ ) => {
        $crate::abistr_macros::cstr8_impl!(($crate) $($tt)+)
    };
}

/// Create a <code>&[CStrNonNull]<[u16]></code> literal at compile time
#[cfg(doc)]
#[macro_export]
macro_rules! cstr16 {
    ( $string:literal ) => {
        $crate::abistr_macros::cstr16_impl!(($crate) $string)
    };
}

/// Create a <code>&[CStrNonNull]<[u16]></code> literal at compile time
#[cfg(not(doc))] // use wildcards for better error messages from proc macro
#[macro_export]
macro_rules! cstr16 {
    ( $($tt:tt)+ ) => {
        $crate::abistr_macros::cstr16_impl!(($crate) $($tt)+)
    };
}

/// Create a <code>&[CStrNonNull]<[u32]></code> literal at compile time
#[cfg(doc)]
#[macro_export]
macro_rules! cstr32 {
    ( $string:literal ) => {
        $crate::abistr_macros::cstr32_impl!(($crate) $string)
    };
}

/// Create a <code>&[CStrNonNull]<[u32]></code> literal at compile time
#[cfg(not(doc))] // use wildcards for better error messages from proc macro
#[macro_export]
macro_rules! cstr32 {
    ( $($tt:tt)+ ) => {
        $crate::abistr_macros::cstr32_impl!(($crate) $($tt)+)
    };
}



#[test] fn basics() {
    fn a(_: CStrNonNull<'static>) {}
    fn b(_: CStrNonNull) {}

    let empty       = cstr!("");
    let example     = cstr!("example");
    let not_unicode = cstr!(b"\xFF\xFF");

    assert_eq!(empty        .to_bytes(), b"");
    assert_eq!(example      .to_bytes(), b"example");
    assert_eq!(not_unicode  .to_bytes(), b"\xFF\xFF");

    a(empty);
    b(empty);
    a(example);
    b(example);
    a(not_unicode);
    b(not_unicode);
}

#[test] fn basics8() {
    fn a(_: CStrNonNull<'static>) {}
    fn b(_: CStrNonNull) {}

    let empty       = cstr8!("");
    let example     = cstr8!("example");
    let not_unicode = cstr8!(b"\xFF\xFF");

    assert_eq!(empty        .to_units(), b"");
    assert_eq!(example      .to_units(), b"example");
    assert_eq!(not_unicode  .to_units(), b"\xFF\xFF");

    a(empty);
    b(empty);
    a(example);
    b(example);
    a(not_unicode);
    b(not_unicode);
}

#[test] fn basics16() {
    fn a(_: CStrNonNull<'static, u16>) {}
    fn b(_: CStrNonNull<u16>) {}

    let empty       = cstr16!("");
    let example     = cstr16!("example");

    assert_eq!(empty        .to_units(), []);
    assert_eq!(example      .to_units(), [b'e' as u16, b'x' as u16, b'a' as u16, b'm' as u16, b'p' as u16, b'l' as u16, b'e' as u16]);

    a(empty);
    b(empty);
    a(example);
    b(example);
}

#[test] fn basics32() {
    fn a(_: CStrNonNull<'static, u32>) {}
    fn b(_: CStrNonNull<u32>) {}

    let empty       = cstr32!("");
    let example     = cstr32!("example");

    assert_eq!(empty        .to_units(), []);
    assert_eq!(example      .to_units(), [b'e' as u32, b'x' as u32, b'a' as u32, b'm' as u32, b'p' as u32, b'l' as u32, b'e' as u32]);

    a(empty);
    b(empty);
    a(example);
    b(example);
}

mod compile_tests {
    /// ```no_run
    /// use abistr::*;
    /// let _ =  cstr!(b"\xFF");
    /// let _ = cstr8!(b"\xFF");
    /// ```
    #[allow(dead_code)] struct HexInRange8;

    /// ```no_run
    /// use abistr::*;
    /// let _ =  cstr!("\x7F");
    /// let _ = cstr8!("\x7F");
    /// ```
    #[allow(dead_code)] struct HexInRange7;

    /// ```compile_fail
    /// use abistr::*;
    /// let _ =  cstr!("\xFF"); // no b prefix means max is 7F
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// let _ = cstr8!("\xFF"); // no b prefix means max is 7F
    /// ```
    #[allow(dead_code)] struct HexOutOfRange;

    /// ```compile_fail
    /// use abistr::*;
    /// let _ =  cstr16!("\xFF");
    /// ```
    ///
    /// ```compile_fail
    /// use abistr::*;
    /// let _ = cstr32!("\xFF");
    /// ```
    #[allow(dead_code)] struct HexAmbiguous;
}
