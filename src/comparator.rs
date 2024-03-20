// Copyright 2020 Tyler Neely
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use libc::{c_char, c_int, c_uchar, c_void, size_t};
use std::cmp::Ordering;
use std::ffi::CString;
use std::slice;

pub type CompareFn = dyn Fn(&[u8], &[u8]) -> Ordering;
pub type CompareTSFn = dyn Fn(&[u8], &[u8]) -> Ordering;
pub type CompareWithoutTSFn = dyn Fn(&[u8], bool, &[u8], bool) -> Ordering;

pub struct ComparatorCallback {
    pub name: CString,
    pub f: Box<CompareFn>,
}

pub unsafe extern "C" fn destructor_callback(raw_cb: *mut c_void) {
    drop(Box::from_raw(raw_cb as *mut ComparatorCallback));
}

pub unsafe extern "C" fn name_callback(raw_cb: *mut c_void) -> *const c_char {
    let cb: &mut ComparatorCallback = &mut *(raw_cb as *mut ComparatorCallback);
    let ptr = cb.name.as_ptr();
    ptr as *const c_char
}

pub unsafe extern "C" fn compare_callback(
    raw_cb: *mut c_void,
    a_raw: *const c_char,
    a_len: size_t,
    b_raw: *const c_char,
    b_len: size_t,
) -> c_int {
    let cb: &mut ComparatorCallback = &mut *(raw_cb as *mut ComparatorCallback);
    let a: &[u8] = slice::from_raw_parts(a_raw as *const u8, a_len);
    let b: &[u8] = slice::from_raw_parts(b_raw as *const u8, b_len);
    match (cb.f)(a, b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

pub struct ComparatorWithTSCallback {
    pub name: CString,
    pub compare: Box<CompareFn>,
    pub compare_ts: Box<CompareTSFn>,
    pub compare_without_ts: Box<CompareWithoutTSFn>,
}

pub unsafe extern "C" fn destructor_with_ts_callback(raw_cb: *mut c_void) {
    drop(Box::from_raw(raw_cb as *mut ComparatorWithTSCallback));
}

pub unsafe extern "C" fn name_with_ts_callback(raw_cb: *mut c_void) -> *const c_char {
    let cb: &mut ComparatorWithTSCallback = &mut *(raw_cb as *mut ComparatorWithTSCallback);
    let ptr = cb.name.as_ptr();
    ptr as *const c_char
}

pub unsafe extern "C" fn compare_with_ts_compare_callback(
    raw_cb: *mut c_void,
    a_raw: *const c_char,
    a_len: size_t,
    b_raw: *const c_char,
    b_len: size_t,
) -> c_int {
    let cb: &mut ComparatorWithTSCallback = &mut *(raw_cb as *mut ComparatorWithTSCallback);
    let a: &[u8] = slice::from_raw_parts(a_raw as *const u8, a_len);
    let b: &[u8] = slice::from_raw_parts(b_raw as *const u8, b_len);
    match (cb.compare)(a, b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

pub unsafe extern "C" fn compare_with_ts_compare_ts_callback(
    raw_cb: *mut c_void,
    ts1_raw: *const c_char,
    ts1_len: size_t,
    ts2_raw: *const c_char,
    ts2_len: size_t,
) -> c_int {
    let cb: &mut ComparatorWithTSCallback = &mut *(raw_cb as *mut ComparatorWithTSCallback);
    let ts1: &[u8] = slice::from_raw_parts(ts1_raw as *const u8, ts1_len);
    let ts2: &[u8] = slice::from_raw_parts(ts2_raw as *const u8, ts2_len);
    match (cb.compare_ts)(ts1, ts2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

pub unsafe extern "C" fn compare_with_ts_compare_without_ts_callback(
    raw_cb: *mut c_void,
    a_raw: *const c_char,
    a_len: size_t,
    a_has_ts: c_uchar,
    b_raw: *const c_char,
    b_len: size_t,
    b_has_ts: c_uchar,
) -> c_int {
    let cb: &mut ComparatorWithTSCallback = &mut *(raw_cb as *mut ComparatorWithTSCallback);
    let a: &[u8] = slice::from_raw_parts(a_raw as *const u8, a_len);
    let a_has_ts: bool = a_has_ts != 0;
    let b: &[u8] = slice::from_raw_parts(b_raw as *const u8, b_len);
    let b_has_ts: bool = b_has_ts != 0;
    match (cb.compare_without_ts)(a, a_has_ts, b, b_has_ts) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}
