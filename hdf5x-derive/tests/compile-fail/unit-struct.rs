extern crate hdf5x_derive;
use hdf5x_derive::H5Type;

#[derive(H5Type)]
//~^ ERROR proc-macro derive
//~^^ HELP Cannot derive H5Type for unit structs
struct Foo;

fn main() {}
