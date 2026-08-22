#![deny(clippy::all)]

use std::{
  cell::Cell,
  mem::MaybeUninit,
  ptr::{drop_in_place, null_mut},
};

use napi::{bindgen_prelude::*, Env};
use napi_derive::napi;

use uv_sys::sys::{uv_check_init, uv_check_start, uv_check_t, uv_close, uv_handle_s, uv_unref};

thread_local! {
  static CHECK_HANDLE: Cell<*mut uv_check_t> = const { Cell::new(null_mut()) };

  static ITERS: Cell<i64> = const { Cell::new(0) };
}

fn reset_state() {
  ITERS.set(0);
}

unsafe extern "C" fn on_check(_handle: *mut uv_check_t) {
  ITERS.set(ITERS.get() + 1);
}

unsafe extern "C" fn on_close(handle: *mut uv_handle_s) {
  drop_in_place(handle);
}

#[allow(non_snake_case)]
#[napi]
pub fn lastLoopIters() -> i64 {
  ITERS.replace(0)
}

#[napi]
pub fn track(env: Env) -> Result<()> {
  if !CHECK_HANDLE.get().is_null() {
    reset_state();
    return Ok(());
  }

  let current_event_loop = env.get_uv_event_loop()?;
  let mut check_handle = Box::new(MaybeUninit::<uv_check_t>::uninit());

  let status = unsafe { uv_check_init(current_event_loop.cast(), check_handle.as_mut_ptr()) };
  if status != 0 {
    return Err(Error::from_reason(format!(
      "uv_check_init failed with status {status}"
    )));
  }

  // At this point the check_handle will be cleaned up by uv_close
  let check_handle = Box::leak(check_handle);

  let status = unsafe { uv_check_start(check_handle.as_mut_ptr(), Some(on_check)) };
  if status != 0 {
    unsafe {
      uv_close(check_handle.as_mut_ptr().cast(), Some(on_close));
    }
    return Err(Error::from_reason(format!(
      "uv_check_start failed with status {status}"
    )));
  }

  unsafe {
    uv_unref(check_handle.as_mut_ptr().cast());
  }
  CHECK_HANDLE.set(check_handle.as_mut_ptr());

  Ok(())
}

#[napi]
pub fn untrack() {
  let check_handle = CHECK_HANDLE.replace(null_mut());

  if !check_handle.is_null() {
    unsafe {
      uv_close(check_handle.cast(), Some(on_close));
    }
    reset_state();
  }
}
