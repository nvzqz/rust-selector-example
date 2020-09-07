#[repr(transparent)]
#[derive(Debug)]
pub struct SEL(*mut objc_selector);

unsafe impl Send for SEL {}
unsafe impl Sync for SEL {}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct objc_selector([u8; 0]);

#[link_section = "__TEXT,__objc_methname,cstring_literals"]
#[no_mangle]
#[used]
static aClass: [u8; 6] = *b"class\0";

#[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
#[used]
static CLASS_SEL: SEL = SEL(&aClass as *const _ as *mut _);

fn main() {
    let sel: &SEL = &CLASS_SEL;

    println!("{:?}", sel);
}
