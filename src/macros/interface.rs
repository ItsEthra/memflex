/// Generates a trait that will emulate behaviour of C++ virtual functions
/// ```
/// /// The target struct cannot be zero sized!
/// #[repr(C)]
/// pub struct ConcreteType(usize);
///
/// memflex::interface! {
///     pub trait IPlayer impl for ConcreteType {
///         // Notice missing `&self`, this is intentional and macro will implicitly add it.
///         // Functions without `&self` in interface doesn't make much sense.
///         extern "C" fn get_health() -> i32 = 0; // 0 - Index of the virtual function.
///         // *Returns old health*
///         extern "C" fn set_health(new: i32) -> i32 = 1; // 1 - Index of the virtual function.
///     }
/// }
///
/// /* C++ Code
/// class IPlayer {
/// public:
///     virtual int get_health();
///     virtual int set_health(int new);
/// }
/// */
/// ```
#[macro_export]
macro_rules! interface {
    {
        $(
            $vs:vis trait $iname:ident $(impl for $($implt:ident),* )? {
                $(
                    $(extern $($abi:literal)?)? fn $fname:ident( $($arg_name:ident: $arg_ty:ty),* ) $(-> $ret:ty)? = $idx:expr;
                )*
            }
        )*
    } => {
        $(
            $vs unsafe trait $iname: Sized {
                const INDEX_OFFSET: usize = 0;
                const FUNCTION_COUNT: usize = $( $crate::__count_fns!($fname) + )*0;

                $(
                    $(extern $($abi)?)? fn $fname<'this>(&'this self, $($arg_name: $arg_ty),* ) $(-> $ret)? {
                        unsafe {
                            type Fn = $(extern $($abi)?)? fn(*const (), $($arg_ty),*) $(-> $ret)?;

                            ((**core::mem::transmute::<_, *const *const [Fn; $idx + 1]>(self))[Self::INDEX_OFFSET + $idx])(
                                self as *const Self as _,
                                $($arg_name),*
                            )
                        }
                    }
                )*
            }

            $(
                $(
                    unsafe impl $iname for $implt { }
                )*
            )?
        )*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __count_fns {
    ($a:ident) => { 1 };
}