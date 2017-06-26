use std::cell::RefCell;
use std::slice::from_raw_parts_mut;
use libc::{c_int, c_void};
use qobject::QObjectContent;
use qvariant;
use qvariant::QVariantRefMut;

pub(crate) fn invoke_slot<T>(object: *mut c_void, slot_name: *mut c_void,
                                argc: c_int, argv: *mut *mut c_void)
    where T: QObjectContent {
    let object_ptr = object as *mut RefCell<T>;
    let object = unsafe { object_ptr.as_mut() }.unwrap();
    let slice = unsafe { from_raw_parts_mut(argv, argc as usize) };
    let vec: Vec<QVariantRefMut> = slice.iter()
        .skip(1)
        .map(|&variant| qvariant::qvariantrefmut::from_ptr(variant))
        .collect();
    let slot_name: String = qvariant::qvariantrefmut::from_ptr(slot_name).into();

    let mut content = object.borrow_mut();
    if let Some(returned) = content.invoke_slot(&slot_name, vec) {
        let mut output = qvariant::qvariantrefmut::from_ptr(slice[0]);
        output.set(&returned);
    };
}