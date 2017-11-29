include!(concat!("../", file!(), ".bis.rs"));

#[test]
fn it_works() {
    assert_eq!("src/lib.rs", file!());
}
