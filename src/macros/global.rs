/// Declares global variables with fixed offset from module
/// ```
/// fn get_module_by_address(module: &str, offset: usize) -> usize {
///     todo!()
/// }
///
/// memflex::global! {
///     // Uses default ldr resolver
///     pub static MY_GLOBAL: i32 = "app.exe"#0xAABB;
///
///     // Or use another function to get offset
///     pub static HEALTH: f32 = (get_module_by_address)"app.exe"#0xFFEE;
/// }
/// ```
#[macro_export]
macro_rules! global {
    {
        $(
            $vs:vis static $gname:ident: $gtype:ty = $( ($resolver:ident) )? $modname:literal $sep:tt $offset:expr;
        )*
    } => {
        $(
            $vs static $gname: $crate::Global<$gtype> = $crate::Global::new($crate::__resolver!( $($resolver)? ), $modname, $offset);
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __resolver {
    () => {
        $crate::__default_resolver
    };
    ($($tt:tt)*) => {
        $($tt)*
    }
}
