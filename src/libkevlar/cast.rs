use std::cast::{transmute};
use std::mem::{size_of};
use std::raw::{Slice};

pub unsafe fn coerce_from_bytes<'a, T>(bytes: &'a [u8]) -> &'a T {
  assert!(size_of::<T>() == bytes.len());
  let slice: Slice<u8> = transmute(bytes);
  let ptr = slice.data;
  let val: &'a T = transmute(ptr);
  val
}

pub unsafe fn coerce_as_bytes<'a, T>(val: &'a T) -> &'a [u8] {
  let ptr: *u8 = transmute(val);
  let slice = Slice::<u8>{
    data: ptr,
    len: size_of::<T>(),
  };
  let bytes: &[u8] = transmute(slice);
  bytes
}
