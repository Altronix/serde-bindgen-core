use serde;
use serde_bindgen_core::binding;
use serde_json_core;

#[binding(prefix = "test", rename_all = "camelCase")]
pub struct Foo {
    this_is_a_thing: u8,
}

#[test]
fn can_rename_all() {
    let mut parsed = std::mem::MaybeUninit::<FooBorrowed>::uninit();
    let data = "{\"thisIsAThing\":3}";
    let l = data.len();
    let p = data.as_ptr() as *const u8;
    let ret = unsafe { test_parse_foo(&mut *parsed.as_mut_ptr(), p, l) };
    assert_eq!(ret, l as i32);
    let parsed = unsafe { parsed.assume_init() };
    assert_eq!(parsed.this_is_a_thing, 3);
}
