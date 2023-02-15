use core::slice::from_raw_parts;

/// Virtual method table pointer
/// ```
/// #[repr(C)]
/// struct ActualVmt {
///     f1: extern fn() -> i32,
///     f2: extern fn(i32, i32) -> f64,
///     zero: usize,
/// }
///
/// extern fn dummy1() -> i32 { 0 }
/// extern fn dummy2(_: i32, _: i32) -> f64 { 0. }
///
/// #[repr(C)]
/// struct Obj {
///     vmt: *const ActualVmt
/// }
///
/// let obj = Box::into_raw(Box::new(Obj {
///     vmt: Box::into_raw(Box::new(ActualVmt {
///         f1: dummy1,
///         f2: dummy2,
///         zero: 0
///     })) as _,
/// }));
///
/// use memflex::types::VmtPtr;
/// #[repr(C)]
/// struct ObjFlex {
///     vmt: VmtPtr
/// }
///
/// # unsafe {
/// let obj_flex: &ObjFlex = &*(obj as *const Obj as *const ObjFlex);
/// assert_eq!(obj_flex.vmt.at::<extern fn(i32, i32) -> f32>(1) as usize, dummy2 as usize);
/// # }
/// ```
#[repr(transparent)]
pub struct VmtPtr {
    vmt: *const usize,
}

impl VmtPtr {
    /// Creates a slice of all functions in vmt until meets `0`.
    /// # Safety
    /// * `self` must be a valid pointer with at most `usize::MAX - 1` zero terminated elements.
    pub unsafe fn dump_terminated(&self) -> &[usize] {
        crate::terminated_array(self.vmt, 0)
    }

    /// Creates a slice of `count` functions in vmt.
    /// # Safety
    /// * `self` must be a valid pointer for a slice of `count` usize's.
    pub unsafe fn dump(&self, count: usize) -> &[usize] {
        from_raw_parts(self.vmt, count)
    }

    /// Returns a function pointer at index `idx`.
    /// # Safety
    /// * `T` must be a function pointer type.
    /// * `self` must be a valid pointer to a virtual method table that contains at least `idx` elements.
    pub unsafe fn at<T: Copy>(&self, idx: usize) -> T {
        self.vmt.add(idx).cast::<T>().read()
    }
}
