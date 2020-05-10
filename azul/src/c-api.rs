// WARNING: autogenerated code for azul api version 0.1.0

use core::ffi::c_void;
use azul_core::dom::Dom;
use azul_core::callbacks::LayoutInfo;
use azul_css::Css;
use azul_core::window::WindowCreateOptions;
#[cfg(not(target_arch = "wasm32"))]
use azul_desktop::app::{App, AppConfig};
#[cfg(target_arch = "wasm32")]
use azul_web::app::{App, AppConfig};

/// The data model
pub type AzDataModelPtr = *mut c_void;
/// The layout() callback fn
pub type AzLayoutCallbackPtr = fn(&AzDataModelPtr, &AzLayoutInfoPtr) -> AzDomPtr;


/// Pointer to rust-allocated `LayoutInfo` struct
#[no_mangle] #[repr(C)] pub struct AzLayoutInfoPtr { ptr: *mut c_void }
/// Destructor: Takes ownership of the `LayoutInfo` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_layout_info_delete(ptr: AzLayoutInfoPtr) { let _ = unsafe { Box::<LayoutInfo>::from_raw(ptr.ptr  as *mut LayoutInfo) }; }

/// Pointer to rust-allocated `Dom` struct
#[no_mangle] #[repr(C)] pub struct AzDomPtr { ptr: *mut c_void }
// Creates a new `Dom` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `Dom::div()` constructor.
#[no_mangle] pub extern "C" fn az_dom_div() -> AzDomPtr { AzDomPtr { ptr: Box::into_raw(Box::new(Dom::<AzDataModelPtr>::div())) as *mut c_void } }
/// Destructor: Takes ownership of the `Dom` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_dom_delete(ptr: AzDomPtr) { let _ = unsafe { Box::<Dom<AzDataModelPtr>>::from_raw(ptr.ptr  as *mut Dom<AzDataModelPtr>) }; }
/// (private): Downcasts the `AzDomPtr` to a `Box<Dom<AzDataModelPtr>>`. Note that this takes ownership of the pointer.
fn az_dom_downcast(ptr: AzDomPtr) -> Box<Dom<AzDataModelPtr>> { unsafe { Box::<Dom<AzDataModelPtr>>::from_raw(ptr.ptr  as *mut Dom<AzDataModelPtr>) } }

/// Pointer to rust-allocated `AppConfig` struct
#[no_mangle] #[repr(C)] pub struct AzAppConfigPtr { ptr: *mut c_void }
// Creates a new `AppConfig` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `AppConfig::new()` constructor.
#[no_mangle] pub extern "C" fn az_app_config_new() -> AzAppConfigPtr { AzAppConfigPtr { ptr: Box::into_raw(Box::new(AppConfig::default())) as *mut c_void } }
/// Destructor: Takes ownership of the `AppConfig` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_app_config_delete(ptr: AzAppConfigPtr) { let _ = unsafe { Box::<AppConfig>::from_raw(ptr.ptr  as *mut AppConfig) }; }
/// (private): Downcasts the `AzAppConfigPtr` to a `Box<AppConfig>`. Note that this takes ownership of the pointer.
fn az_app_config_downcast(ptr: AzAppConfigPtr) -> Box<AppConfig> { unsafe { Box::<AppConfig>::from_raw(ptr.ptr  as *mut AppConfig) } }

/// Pointer to rust-allocated `Css` struct
#[no_mangle] #[repr(C)] pub struct AzCssPtr { ptr: *mut c_void }
// Creates a new `Css` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `Css::native()` constructor.
#[no_mangle] pub extern "C" fn az_css_native() -> AzCssPtr { AzCssPtr { ptr: Box::into_raw(Box::new(azul_native_style::native())) as *mut c_void } }
/// Destructor: Takes ownership of the `Css` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_css_delete(ptr: AzCssPtr) { let _ = unsafe { Box::<Css>::from_raw(ptr.ptr  as *mut Css) }; }
/// (private): Downcasts the `AzCssPtr` to a `Box<Css>`. Note that this takes ownership of the pointer.
fn az_css_downcast(ptr: AzCssPtr) -> Box<Css> { unsafe { Box::<Css>::from_raw(ptr.ptr  as *mut Css) } }

/// Pointer to rust-allocated `WindowCreateOptions` struct
#[no_mangle] #[repr(C)] pub struct AzWindowCreateOptionsPtr { ptr: *mut c_void }
// Creates a new `WindowCreateOptions` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `WindowCreateOptions::new()` constructor.
#[no_mangle] pub extern "C" fn az_window_create_options_new(css: AzCssPtr) -> AzWindowCreateOptionsPtr { AzWindowCreateOptionsPtr { ptr: Box::into_raw(Box::new(WindowCreateOptions::<AzDataModelPtr>::new(*az_css_downcast(css)))) as *mut c_void } }
/// Destructor: Takes ownership of the `WindowCreateOptions` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_window_create_options_delete(ptr: AzWindowCreateOptionsPtr) { let _ = unsafe { Box::<WindowCreateOptions<AzDataModelPtr>>::from_raw(ptr.ptr  as *mut WindowCreateOptions<AzDataModelPtr>) }; }
/// (private): Downcasts the `AzWindowCreateOptionsPtr` to a `Box<WindowCreateOptions<AzDataModelPtr>>`. Note that this takes ownership of the pointer.
fn az_window_create_options_downcast(ptr: AzWindowCreateOptionsPtr) -> Box<WindowCreateOptions<AzDataModelPtr>> { unsafe { Box::<WindowCreateOptions<AzDataModelPtr>>::from_raw(ptr.ptr  as *mut WindowCreateOptions<AzDataModelPtr>) } }

/// Pointer to rust-allocated `App` struct
#[no_mangle] #[repr(C)] pub struct AzAppPtr { ptr: *mut c_void }
/// Creates a new App instance.
#[no_mangle] pub extern "C" fn az_app_new(data: AzDataModelPtr, config: AzAppConfigPtr, callback: AzLayoutCallbackPtr) -> AzAppPtr { AzAppPtr { ptr: Box::into_raw(Box::new(App::new_with_callback(data, *az_app_config_downcast(config), callback))) as *mut c_void } }
// Equivalent to the Rust `App::run()` function.
#[no_mangle] pub extern "C" fn az_app_run(app: AzAppPtr, window: AzWindowCreateOptionsPtr) { az_app_downcast(app).run(*az_window_create_options_downcast(window)) }
/// Destructor: Takes ownership of the `App` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_app_delete(ptr: AzAppPtr) { let _ = unsafe { Box::<App<AzDataModelPtr>>::from_raw(ptr.ptr  as *mut App<AzDataModelPtr>) }; }
/// (private): Downcasts the `AzAppPtr` to a `Box<App<AzDataModelPtr>>`. Note that this takes ownership of the pointer.
fn az_app_downcast(ptr: AzAppPtr) -> Box<App<AzDataModelPtr>> { unsafe { Box::<App<AzDataModelPtr>>::from_raw(ptr.ptr  as *mut App<AzDataModelPtr>) } }