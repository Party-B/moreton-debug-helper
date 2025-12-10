// =============================================================================
// ADDITIONAL USEFUL STD LIBRARY WRAPPERS FOR DEBUGGING
// =============================================================================

#[cfg(debug_assertions)]
#[inline(never)]
pub unsafe fn d_transmute<T, U>(t: T) -> U {
    std::mem::transmute(t)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_replace<T>(dest: &mut T, src: T) -> T {
    std::mem::replace(dest, src)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub unsafe fn d_ptr_read<T>(src: *const T) -> T {
    std::ptr::read(src)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub unsafe fn d_ptr_write<T>(dst: *mut T, val: T) {
    std::ptr::write(dst, val)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_type_name<T>() -> &'static str {
    std::any::type_name::<T>()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_try_from<T, U>(u: U) -> Result<T, U::Error> where T: std::convert::TryFrom<U> {
    T::try_from(u)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_deref<T>(t: &T) -> &T::Target where T: std::ops::Deref {
    &**t
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_deref_mut<T>(t: &mut T) -> &mut T::Target where T: std::ops::DerefMut {
    &mut **t
}

// Collection helpers
#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_vec_len<T>(v: &Vec<T>) -> usize {
    v.len()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_hashmap_len<K, V>(m: &std::collections::HashMap<K, V>) -> usize {
    m.len()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_vecdeque_len<T>(v: &std::collections::VecDeque<T>) -> usize {
    v.len()
}

// Environment variable
#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_env_var(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

// Thread info
#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_thread_name() -> Option<String> {
    std::thread::current().name().map(|s| s.to_string())
}

// Timing
#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_instant_now() -> std::time::Instant {
    std::time::Instant::now()
}

// Debug printing
#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_debug<T: std::fmt::Debug>(val: &T) {
    println!("DEBUG: {:?}", val);
}
#![allow(dead_code)]

// =============================================================================
// STRING OPERATIONS
// =============================================================================

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_trim(s: &str) -> &str {
    s.trim()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_trim_start(s: &str) -> &str {
    s.trim_start()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_trim_end(s: &str) -> &str {
    s.trim_end()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_trim_matches(s: &str, pat: char) -> &str {
    s.trim_matches(pat)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_trim_end_matches(s: &str, pat: char) -> &str {
    s.trim_end_matches(pat)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_len(s: &str) -> usize {
    s.len()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_is_empty(s: &str) -> bool {
    s.is_empty()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_contains(s: &str, pat: &str) -> bool {
    s.contains(pat)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_starts_with(s: &str, pat: &str) -> bool {
    s.starts_with(pat)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_ends_with(s: &str, pat: &str) -> bool {
    s.ends_with(pat)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_split_once<'a>(s: &'a str, delim: &str) -> Option<(&'a str, &'a str)> {
    s.split_once(delim)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

// =============================================================================
// PARSING OPERATIONS
// =============================================================================

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_parse_i32(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_parse_i64(s: &str) -> Option<i64> {
    s.parse::<i64>().ok()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_parse_u32(s: &str) -> Option<u32> {
    s.parse::<u32>().ok()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_parse_u64(s: &str) -> Option<u64> {
    s.parse::<u64>().ok()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_parse_f64(s: &str) -> Option<f64> {
    s.parse::<f64>().ok()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_parse_bool(s: &str) -> Option<bool> {
    s.parse::<bool>().ok()
}

// =============================================================================
// VEC OPERATIONS
// =============================================================================

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_vec_len<T>(v: &Vec<T>) -> usize {
    v.len()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_vec_is_empty<T>(v: &Vec<T>) -> bool {
    v.is_empty()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_vec_capacity<T>(v: &Vec<T>) -> usize {
    v.capacity()
}

// Note: Can't return &T easily in GDB, so we'll use print functions
#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_vec_print_first<T: std::fmt::Debug>(v: &Vec<T>) {
    eprintln!("First: {:?}", v.first());
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_vec_print_last<T: std::fmt::Debug>(v: &Vec<T>) {
    eprintln!("Last: {:?}", v.last());
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_vec_print_get<T: std::fmt::Debug>(v: &Vec<T>, idx: usize) {
    eprintln!("v[{}]: {:?}", idx, v.get(idx));
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_vec_print_all<T: std::fmt::Debug>(v: &Vec<T>) {
    eprintln!("Vec contents: {:?}", v);
}

// =============================================================================
// OPTION OPERATIONS
// =============================================================================

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_is_some<T>(opt: &Option<T>) -> bool {
    opt.is_some()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_is_none<T>(opt: &Option<T>) -> bool {
    opt.is_none()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_opt_print<T: std::fmt::Debug>(opt: &Option<T>) {
    eprintln!("Option: {:?}", opt);
}

// =============================================================================
// RESULT OPERATIONS
// =============================================================================

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_is_ok<T, E>(res: &Result<T, E>) -> bool {
    res.is_ok()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_is_err<T, E>(res: &Result<T, E>) -> bool {
    res.is_err()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_result_print<T: std::fmt::Debug, E: std::fmt::Debug>(res: &Result<T, E>) {
    eprintln!("Result: {:?}", res);
}

// =============================================================================
// PRINT UTILITIES (for when GDB can't display complex types)
// =============================================================================

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_print<T: std::fmt::Debug>(value: &T) {
    eprintln!("DEBUG: {:?}", value);
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_print_labeled<T: std::fmt::Debug>(label: &str, value: &T) {
    eprintln!("DEBUG [{}]: {:?}", label, value);
}

// Print and return (useful for checking without interrupting flow)
#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_inspect<T: std::fmt::Debug>(value: &T) -> &T {
    eprintln!("INSPECT: {:?}", value);
    value
}

// =============================================================================
// FORCE LINKER TO KEEP SYMBOLS (so GDB can find them)
// =============================================================================

#[cfg(debug_assertions)]
#[inline(never)]
pub fn _force_link_all() {
    // This function ACTUALLY calls all debug helpers so the linker keeps them
    // The calls are real but operate on dummy data, so there's minimal runtime cost
    let s = "";
    let _ = d_trim(s);
    let _ = d_trim_start(s);
    let _ = d_trim_end(s);
    let _ = d_trim_matches(s, ' ');
    let _ = d_trim_end_matches(s, ' ');
    let _ = d_len(s);
    let _ = d_is_empty(s);
    let _ = d_contains(s, "");
    let _ = d_starts_with(s, "");
    let _ = d_ends_with(s, "");
    let _ = d_split_once(s, "");
    let _ = d_to_lowercase(s);
    let _ = d_to_uppercase(s);
    let _ = d_parse_i32(s);
    let _ = d_parse_i64(s);
    let _ = d_parse_u32(s);
    let _ = d_parse_u64(s);
    let _ = d_parse_f64(s);
    let _ = d_parse_bool(s);
    
    let v: Vec<i32> = vec![];
    let _ = d_vec_len(&v);
    let _ = d_vec_is_empty(&v);
    let _ = d_vec_capacity(&v);
    d_vec_print_first(&v);
    d_vec_print_last(&v);
    d_vec_print_get(&v, 0);
    d_vec_print_all(&v);
    
    let opt: Option<i32> = None;
    let _ = d_is_some(&opt);
    let _ = d_is_none(&opt);
    d_opt_print(&opt);
    
    let res: Result<i32, ()> = Ok(0);
    let _ = d_is_ok(&res);
    let _ = d_is_err(&res);
    d_result_print(&res);
    
    d_print(&s);
    d_print_labeled("", &s);
    let _ = d_inspect(&s);
}

// =============================================================================
// CUSTOM PROJECT-SPECIFIC HELPERS
// =============================================================================
// Add your own domain-specific helpers below this line
// Example:
//
// #[cfg(debug_assertions)]
// #[inline(never)]
// pub fn d_parse_my_format(s: &str) -> Option<MyType> {
//     // Your custom parsing logic
// }

// =============================================================================
// USAGE EXAMPLES (commented out, for reference)
// =============================================================================

/*
In GDB:

(gdb) break main.rs:42
(gdb) run
(gdb) call debug_helpers::d_trim(my_string)
$1 = "trimmed content"

(gdb) call debug_helpers::d_split_once("key=value", "=")
$2 = Some(("key", "value"))

(gdb) call debug_helpers::d_print(&my_complex_struct)
DEBUG: MyStruct { field1: 42, field2: "hello" }

(gdb) call debug_helpers::d_vec_len(&my_vec)
$3 = 5

(gdb) call debug_helpers::d_vec_print_first(&my_vec)
First: Some(42)
*/

// =============================================================================
// COMMON RUST STD LIBRARY WRAPPERS FOR DEBUGGING
// =============================================================================

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_size_of<T>() -> usize {
    std::mem::size_of::<T>()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_align_of<T>() -> usize {
    std::mem::align_of::<T>()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_null<T>() -> *const T {
    std::ptr::null::<T>()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_null_mut<T>() -> *mut T {
    std::ptr::null_mut::<T>()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_from_utf8(v: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_slice_from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T] {
    unsafe { std::slice::from_raw_parts(data, len) }
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_slice_from_raw_parts_mut<'a, T>(data: *mut T, len: usize) -> &'a mut [T] {
    unsafe { std::slice::from_raw_parts_mut(data, len) }
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_min<T: Ord>(a: T, b: T) -> T {
    std::cmp::min(a, b)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_max<T: Ord>(a: T, b: T) -> T {
    std::cmp::max(a, b)
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_option_is_some<T>(opt: Option<T>) -> bool {
    opt.is_some()
}

#[cfg(debug_assertions)]
#[inline(never)]
pub fn d_result_is_ok<T, E>(res: Result<T, E>) -> bool {
    res.is_ok()
}
