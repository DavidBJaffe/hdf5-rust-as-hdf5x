use super::gen::gen_ascii;

pub fn random_filename() -> String {
    gen_ascii(&mut rand::thread_rng(), 8)
}

pub fn new_in_memory_file() -> hdf5x::Result<hdf5x::File> {
    let filename = random_filename();
    hdf5x::File::with_options().with_fapl(|p| p.core_filebacked(false)).create(&filename)
}
