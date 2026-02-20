use crate::memalloc::{free, malloc};

//frees both of the strings
pub fn strcat(first: &'static str, second: &'static str) -> &'static str {
    let (bytes1, bytes2) = (first.as_bytes(), second.as_bytes());
    let dest = malloc(bytes1.len() + bytes2.len());
    dest[..bytes1.len()].copy_from_slice(bytes1);
    dest[bytes1.len()..].copy_from_slice(bytes2);
    free(bytes1);
    free(bytes2);
    str::from_utf8(dest).unwrap()
}
