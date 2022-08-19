/// Emulates C++ parenting, with constrain that child may only has ONE parent.
/// # Behavior
/// * Each struct declared within `makestruct` macro will have C-like layout.
/// * For each struct declared within `makestruct` macro with specified parent there will be generated:
///     * Additional first field of parent type and name of `parent`
///     * Deref<Target = Parent> implementation
/// ```
/// memflex::makestruct! {
///     // Attributes works as expected
///     #[derive(Default)]
///     struct Parent {
///         // on fields as well
///         // #[serde(skip)]
///         first: f32
///     }
///     
///     // `pub` means that `parent` field will be `pub`.
///     struct Child : pub Parent {
///         second: i32
///     }
///
///     // Implements `Foo` interface on `Nested`
///     struct Nested impl Foo : Child {
///         third: bool
///     }
/// }
///
/// memflex::interface! {
///     trait Foo {
///         extern fn foo() = 0;
///     }
/// }
/// ```
#[macro_export]
macro_rules! makestruct {
    {
        $(
            $( #[$($outter:tt)*] )*
            $vs:vis struct $sname:ident $(impl $($iface:ident),* )? $( : $pvis:vis $sparent:ident )?  {
                $(
                    $( #[ $($foutter:tt)* ] )*
                    $fvs:vis $fname:ident: $fty:ty
                ),*$(,)?
            }
        )*
    } => {
        $(
            $( #[$($outter)*] )*
            $vs struct $sname {
                $($pvis parent: $sparent,)?
                $(
                    $( #[ $($foutter)* ] )*
                    $fvs $fname: $fty
                ),*
            }

            $(
                $(
                    unsafe impl $iface for $sname { }
                )*
            )?

            $(
                impl core::ops::Deref for $sname {
                    type Target = $sparent;

                    fn deref(&self) -> &Self::Target {
                        &self.parent
                    }
                }
            )?
        )*
    };
}
