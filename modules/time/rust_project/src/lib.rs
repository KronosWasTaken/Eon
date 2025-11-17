mod time;
pub mod ffi {
    pub mod time {
        use crate::time;
        use eon_core::addons::*;
        use std::ffi::c_void;

        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$time$get_time")]
        pub unsafe extern "C" fn get_time(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let ret: f64 = time::get_time();
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$time$sleep")]
        pub unsafe extern "C" fn sleep(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let seconds = <f64>::from_vm_unsafe(vm, vm_funcs);
                let ret: () = time::sleep(seconds);
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
    }
}
