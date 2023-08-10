#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_werks() {
        // Testing code from RandomX/src/tests/api-example1.c
        unsafe {
            let myKey = b"RandomX example key";
            let myInput = b"RandomX example input";
            let hash = [0u8; RANDOMX_HASH_SIZE as usize];

            let flags = randomx_get_flags();
            let myCache = randomx_alloc_cache(flags);
            randomx_init_cache(
                myCache,
                myKey.as_ptr() as *const std::ffi::c_void,
                myKey.len(),
            );
            let myMachine = randomx_create_vm(flags, myCache, std::ptr::null_mut());

            randomx_calculate_hash(
                myMachine,
                myInput.as_ptr() as *const std::ffi::c_void,
                myInput.len(),
                hash.as_ptr() as *mut std::ffi::c_void,
            );

            randomx_destroy_vm(myMachine);
            randomx_release_cache(myCache);

            assert_eq!(
                hash,
                [
                    210, 164, 216, 149, 3, 68, 116, 1, 239, 110, 111, 48, 180, 102, 53, 180, 91,
                    84, 242, 90, 101, 12, 71, 70, 75, 83, 17, 249, 214, 253, 71, 89
                ]
            );
        }
    }
}
