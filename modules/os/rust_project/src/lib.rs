mod os;
pub mod ffi {
    pub mod os {
        use crate::os;
        use eon_core::addons::*;
        use std::ffi::c_void;

        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$os$fread")]
        pub unsafe extern "C" fn fread(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let path = <String>::from_vm_unsafe(vm, vm_funcs);
                let ret: Option<String> = os::fread(path);
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$os$fwrite")]
        pub unsafe extern "C" fn fwrite(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let contents = <String>::from_vm_unsafe(vm, vm_funcs);
                let path = <String>::from_vm_unsafe(vm, vm_funcs);
                let ret: () = os::fwrite(path, contents);
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$os$fexists")]
        pub unsafe extern "C" fn fexists(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let path = <String>::from_vm_unsafe(vm, vm_funcs);
                let ret: bool = os::fexists(path);
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$os$fremove")]
        pub unsafe extern "C" fn fremove(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let path = <String>::from_vm_unsafe(vm, vm_funcs);
                let ret: () = os::fremove(path);
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$os$frename")]
        pub unsafe extern "C" fn frename(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let new_path = <String>::from_vm_unsafe(vm, vm_funcs);
                let old_path = <String>::from_vm_unsafe(vm, vm_funcs);
                let ret: () = os::frename(old_path, new_path);
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$os$fcopy")]
        pub unsafe extern "C" fn fcopy(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let dest = <String>::from_vm_unsafe(vm, vm_funcs);
                let src = <String>::from_vm_unsafe(vm, vm_funcs);
                let ret: () = os::fcopy(src, dest);
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
        /// # Safety
        /// The caller must ensure that `vm` is a valid pointer and `vm_funcs` points to valid function pointers.
        #[unsafe(export_name = "EON_ffi$os$fappend")]
        pub unsafe extern "C" fn fappend(vm: *mut c_void, vm_funcs: *const EonVmFunctions) {
            unsafe {
                let vm_funcs: &EonVmFunctions = &*vm_funcs;
                let contents = <String>::from_vm_unsafe(vm, vm_funcs);
                let path = <String>::from_vm_unsafe(vm, vm_funcs);
                let ret: () = os::fappend(path, contents);
                ret.to_vm_unsafe(vm, vm_funcs);
            }
        }
    }
}
