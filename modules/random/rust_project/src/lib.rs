mod random;
pub mod ffi {
    pub mod random {
        use crate::random;
        use eon_core::addons::*;
        use std::ffi::c_void;

        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$random$random_float")]
        pub unsafe extern "C" fn random_float(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let max = <f64>::from_vm_unsafe(vm, vm_funcs);
                let min = <f64>::from_vm_unsafe(vm, vm_funcs);
                let ret: f64 = random::random_float(min, max);
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$random$random_int")]
        pub unsafe extern "C" fn random_int(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let max = <i64>::from_vm_unsafe(vm, vm_funcs);
                let min = <i64>::from_vm_unsafe(vm, vm_funcs);
                let ret: i64 = random::random_int(min, max);
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
    }
}
