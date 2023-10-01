use std::sync::{
    atomic::{self, AtomicBool},
    Arc,
};

use std::ffi::{c_void, c_char, CStr, CString};

use serde_json::json;
use warp::{reply::json, Rejection, Reply};

// Health handler, responds with '{ ok: true }' when running and '{ ok: false}'
// when shutting down
pub(super) async fn health(running: Arc<AtomicBool>) -> Result<impl Reply, Rejection> {
    if running.load(atomic::Ordering::Relaxed) {
        Ok(warp::reply::with_status(
            json(&json!({"ok": true})),
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            json(&json!({"ok": false})),
            warp::http::StatusCode::SERVICE_UNAVAILABLE,
        ))
    }
}

/*
pub unsafe extern "C" fn malloc_stats_print(
    write_cb: Option<unsafe extern "C" fn(_: *mut c_void, _: *const c_char)>,
    cbopaque: *mut c_void,
    opts: *const c_char
)
*/
unsafe extern "C" fn cb(rust_str_in: *mut c_void, stats_as_c_str: *const c_char) {
    let stats_as_rust_str: &CStr = unsafe { CStr::from_ptr(stats_as_c_str) };
    let rust_str = std::mem::transmute::<*mut c_void, &mut String>(rust_str_in);
    *rust_str += &String::from_utf8_lossy(stats_as_rust_str.to_bytes());
}

// malloc_stats handler, responds with the output of malloc_stats_print()
#[cfg(feature = "tikv-jemallocator")]
pub(super) async fn malloc_stats() -> Result<impl Reply, Rejection> {
    let mut malloc_stats = String::new();
    unsafe {
        tikv_jemalloc_sys::malloc_stats_print(Some(cb), &mut malloc_stats as *mut _ as *mut c_void, 0 as *const c_char);
    }
    Ok(warp::reply::with_status(malloc_stats,warp::http::StatusCode::OK))
}

/*
pub unsafe extern "C" fn mallctl(
    name: *const c_char,
    oldp: *mut c_void,
    oldlenp: *mut size_t,
    newp: *mut c_void,
    newlen: size_t
) -> c_int
*/

// activate_profiling handler
#[cfg(feature = "tikv-jemallocator")]
pub(super) async fn activate_profiling() -> Result<impl Reply, Rejection> {
    let ret: i32;
    let raw_action = b"prof.active".to_vec();
    let mut switch = true;
    unsafe {
        let action_c_string = CString::from_vec_unchecked(raw_action);
        ret = tikv_jemalloc_sys::mallctl(
            action_c_string.as_ptr(),
            0 as *mut c_void,
            0 as *mut usize,
            &mut switch as *mut _ as *mut c_void,
            std::mem::size_of::<bool>()
        );
    }
    Ok(warp::reply::with_status(format!("{{prof.active: {ret}}}\n"), warp::http::StatusCode::OK))
}

// prof_dump handler
#[cfg(feature = "tikv-jemallocator")]
pub(super) async fn prof_dump() -> Result<impl Reply, Rejection> {
    let ret: i32;
    let raw_action = b"prof.dump".to_vec();
    unsafe {
        let action_c_string = CString::from_vec_unchecked(raw_action);
        ret = tikv_jemalloc_sys::mallctl(
            action_c_string.as_ptr(),
            0 as *mut c_void,
            0 as *mut usize,
            0 as *mut c_void,
            0
        );
    }
    Ok(warp::reply::with_status(format!("{{prof.dump: {ret}}}\n"), warp::http::StatusCode::OK))
}
