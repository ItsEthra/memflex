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
/// assert_eq!(obj_flex.vmt.at::<1, extern fn(i32, i32) -> f32>() as usize, dummy2 as usize);
/// # }
/// ```
#[repr(transparent)]
pub struct VmtPtr {
    vmt: *const usize
}

impl VmtPtr {
    /// Creates a slice of all functions in vmt.
    /// # Safety
    /// * `self` must be a valid pointer with at most `usize::MAX - 1` zero terminated elements.
    pub unsafe fn dump(&self) -> &[usize] {
        crate::terminated_array(self.vmt, 0)
    }

    /// Returns a function pointer at index `N`.
    /// # Safety
    /// * `T` must be a function pointer type
    /// * `self` must be a valid pointer.
    pub unsafe fn at<const N: usize, T: Copy>(&self) -> T {
        self.vmt.add(N)
            .cast::<T>()
            .read()
    }
}