/* automatically generated by rust-bindgen 0.65.1 */

#[doc = " universal area bridge global"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bridge_global {
    #[doc = " num"]
    pub num: usize,
    #[doc = " may be changed by alloc"]
    pub buf: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_bridge_global() {
    const UNINIT: ::std::mem::MaybeUninit<bridge_global> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bridge_global>(),
        16usize,
        concat!("Size of: ", stringify!(bridge_global))
    );
    assert_eq!(
        ::std::mem::align_of::<bridge_global>(),
        8usize,
        concat!("Alignment of ", stringify!(bridge_global))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).num) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bridge_global),
            "::",
            stringify!(num)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buf) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bridge_global),
            "::",
            stringify!(buf)
        )
    );
}
extern "C" {
    #[doc = " bridge global init slots"]
    pub fn bridge_global_init_slots(m: usize) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " bridge global dispose slots"]
    pub fn bridge_global_dispose_slots() -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " bridge global setter"]
    pub fn bridge_global_setter(n: usize, p: *mut bridge_global) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " bridge global getter"]
    pub fn bridge_global_getter(n: usize) -> *mut bridge_global;
}
