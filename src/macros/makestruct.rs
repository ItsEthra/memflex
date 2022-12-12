use core::ops::{Deref, DerefMut};

/// Emulates C++ parenting, with a constraint that a child may only has ONE parent.
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
///     // `pub` means that `parent` field will be `pub`
///     // but Deref<Target = Parent> implementation will be generated regardless.
///     struct Child : pub Parent {
///         second: i32
///     }
///
///     // Implements `Foo` interface on `Nested`
///     struct Nested impl Foo : Child {
///         third: bool
///     }
///
///     struct ParentWithVmt impl ParentVmt {
///         vmt: usize,
///         t1: f32,
///         t2: bool
///     }
///
///     // By using `dyn ParentWithVmt`, child offsets all of their vfunc indices by the number of functions in `ParentWithVmt`,
///     // should work with nested inheritance but hasn't been tested!
///     struct ChildInheritsParentVmt impl ChildVmt(dyn ParentWithVmt) : pub ParentWithVmt {
///         t3: u64,
///         t4: i8
///     }
/// }
///
/// memflex::interface! {
///     trait Foo {
///         extern fn foo() = #0;
///     }
///
///     trait ParentVmt {
///         fn f1() -> i32 = #0;
///         fn f2() -> i32 = #1;
///     }
///
///     trait ChildVmt {
///         fn f3(a: i32) = #0;
///         fn f4(a: i32) = #1;
///     }
/// }
/// ```
#[macro_export]
macro_rules! makestruct {
    {
        $(
            $( #[$($outter:tt)*] )*
            $vs:vis struct $sname:ident
                $(impl $($iface:ident $((dyn $piface:ty))? ),* )?
                $( : $pvis:vis $sparent:ident )?
            {
                $(
                    $( #[ $($foutter:tt)* ] )*
                    $fvs:vis $fname:ident: $fty:ty
                ),*$(,)?
            }
        )*
    } => {
        $(
            $( #[$($outter)*] )*
            #[repr(C)]
            $vs struct $sname {
                $($pvis parent: $sparent,)?
                $(
                    $( #[ $($foutter)* ] )*
                    $fvs $fname: $fty
                ),*
            }

            $(
                unsafe impl $crate::Child for $sname {
                    type Parent = $sparent;
                }

                unsafe impl $crate::Parent<$sname> for $sparent { }
            )?

            $(
                $(
                    unsafe impl $iface for $sname {
                        $( const INDEX_OFFSET: usize = <$piface>::FUNCTION_COUNT + <$piface>::INDEX_OFFSET; )?
                    }
                )*
            )?

            $(
                impl core::ops::Deref for $sname {
                    type Target = $sparent;

                    fn deref(&self) -> &Self::Target {
                        &self.parent
                    }
                }

                impl core::ops::DerefMut for $sname {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.parent
                    }
                }
            )?
        )*
    };
}

/// Struct that is the parent for an other struct.
/// # Safety
/// This trait should not be implemented manually.
pub unsafe trait Parent<C>: Sized
where
    C: Child<Parent = Self>,
{
    /// Casts parent to an immutable child reference
    /// # Safety
    /// There is no way of checking the actual type.
    #[inline(always)]
    unsafe fn downcast(&self) -> &C {
        &*(self as *const Self as *const C)
    }

    /// Casts parent to a mutable child reference
    /// # Safety
    /// There is no way of checking the actual type.
    #[inline(always)]
    unsafe fn downcast_mut(&mut self) -> &mut C {
        &mut *(self as *mut Self as *mut C)
    }
}

/// Struct that is a child of the other struct.
/// # Safety
/// This trait should not be implemented manually.
pub unsafe trait Child: Sized
where
    Self: Deref<Target = Self::Parent> + DerefMut,
{
    /// Type parent
    type Parent: Parent<Self>;

    /// Upcasts child to an immutable parent reference.
    /// # Safety
    /// Parent field must be the first.
    #[inline(always)]
    unsafe fn upcast(&self) -> &Self::Parent {
        &*(self as *const Self as *const Self::Parent)
    }

    /// Upcasts child to a mutable parent reference.
    /// # Safety
    /// Parent field must be the first.
    #[inline(always)]
    unsafe fn upcast_mut(&mut self) -> &mut Self::Parent {
        &mut *(self as *mut Self as *mut Self::Parent)
    }
}

// Methods below are just for convenience because in order to use methods declared in the trait, it
// needs to be in the scope.

/// Downcasts an immutable parent reference to an immutable child reference.
/// # Unsafe
/// See [`Parent::downcast`]
#[inline(always)]
pub fn downcast<C: Child<Parent = P>, P: Parent<C>>(parent: &P) -> &C {
    unsafe { P::downcast(parent) }
}

/// Downcasts a mutable parent reference to a mutable child reference.
/// # Unsafe
/// See [`Parent::downcast_mut`]
#[inline(always)]
pub fn downcast_mut<C: Child<Parent = P>, P: Parent<C>>(parent: &mut P) -> &mut C {
    unsafe { P::downcast_mut(parent) }
}

/// Upcasts an immutable child reference to an immutable parent reference.
/// # Unsafe
/// See [`Child::upcast`]
#[inline(always)]
pub fn upcast<P: Parent<C>, C: Child<Parent = P>>(child: &C) -> &P {
    unsafe { child.upcast() }
}

/// Upcasts a mutable child reference to a mutable parent reference.
/// # Unsafe
/// See [`Child::upcast_mut`]
#[inline(always)]
pub fn upcast_mut<P: Parent<C>, C: Child<Parent = P>>(child: &mut C) -> &mut P {
    unsafe { child.upcast_mut() }
}
