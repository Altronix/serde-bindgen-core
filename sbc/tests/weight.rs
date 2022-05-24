pub use serde;
use serde_bindgen_core::binding;
use serde_json_core;

#[binding(prefix = "test")]
pub struct Remote<'a> {
    ///sbc: len = 4
    id0: &'a str,
}

#[binding(prefix = "test")]
pub struct Foo<'a> {
    id0: u8,
    id1: i8,
    id2: u16,
    id3: i16,
    id4: u32,
    id5: i32,
    id6: bool,
    /// sbc: len = 5
    id7: &'a str,
    id8: Remote<'a>,
    id9: Remote<'a>,
    id10: [u8; 3],
    id11: [Remote<'a>; 3],
}

const DATA: &'static str = r#"
{
    "id0": 255,
    "id1": -128,
    "id2": 65535,
    "id3": -32767,
    "id4": 4294967295,
    "id5": -2147483647,
    "id6": false,
    "id7": "12345",
    "id8": {"id0": "1234"},
    "id9": {"id0": "5678"},
    "id10": [255,255,255],
    "id11": [{"id0": "1234"},{"id0": "1234"},{"id0": "1234"}]
}
"#;

//"id10": [{"id0": "1234"},{"id0":"5678"}]
#[test]
fn can_calculate_weight() {
    let mut foo = std::mem::MaybeUninit::<Foo>::uninit();
    let l = DATA.len();
    let p = DATA.as_ptr() as *const u8;
    let ret = unsafe { test_parse_foo(&mut *foo.as_mut_ptr(), p, l) };
    assert!(ret > 0);

    let foo = unsafe { foo.assume_init() };
    let mut buffer: [u8; 2096] = [0; 2096];
    let mut len = 2096;

    let ret = unsafe { test_print_foo(&foo, &mut *buffer.as_mut_ptr(), &mut len) };
    assert_eq!(len, FOO_MAX_LEN);
}
