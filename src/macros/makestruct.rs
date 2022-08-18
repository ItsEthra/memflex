/// Emulates C++ parenting, with constrain that child may only has ONE parent.
/// # Behavior
/// * Each struct declared within `makestruct` macro, will have C-like layout.
/// * For each struct declared within `makestruct` macro with specified parent,
/// will be generated:
///     * Additional first field of parent type and name of `parent`
///     * Deref<Target = Parent> implementation.
/// ```
/// memflex::makestruct! {
///     #[derive(Default)]
///     pub struct Parent {
///         first: f32
///     }
///     
///     pub struct Child(Parent) {
///         second: i32
///     }
/// }
/// ```
#[macro_export]
macro_rules! makestruct {
    {
        $(
            $( #[$($outter:tt)*] )*
            $vs:vis struct $sname:ident $( ($sparent:ident) )?  {
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
                $(parent: $sparent,)?
                $(
                    $( #[ $($foutter)* ] )*
                    $fvs $fname: $fty
                ),*
            }

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
