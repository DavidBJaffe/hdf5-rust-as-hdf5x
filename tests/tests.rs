#[test]
fn roundtrip_compound_type() {
    use hdf5x::H5Type;
    #[derive(H5Type)]
    #[repr(C)]
    struct Compound {
        a: u8,
        b: u8,
    }

    let dt = hdf5x::Datatype::from_type::<Compound>().unwrap();
    let td = dt.to_descriptor().unwrap();
    assert_eq!(td, Compound::type_descriptor());
}
