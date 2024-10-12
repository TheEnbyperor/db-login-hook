use base64::Engine;
use objc::runtime::{Class, Object, Sel};
use objc::{msg_send, sel, sel_impl};
use objc::declare::ClassDecl;

#[link(name = "Foundation", kind = "framework")]
#[link(name = "AppKit", kind = "framework")]
extern {}

extern fn objc_did_finish_launching(this: &Object, _: Sel, _: u64) {
    let application_cls = Class::get("NSApplication").unwrap();
    let app: *mut Object = unsafe { msg_send![application_cls, sharedApplication] };
    let _:() = unsafe { msg_send![app, terminate: this] };
}

extern fn objc_application_open_urls(_this: &Object, _: Sel, _application: u64, urls: u64) {
    let urls = urls as *mut Object;
    let url: *mut Object = unsafe { msg_send![urls, objectAtIndex: 0] };
    let path: *mut Object = unsafe { msg_send![url, absoluteString] };
    let path_cstr: *const i8 = unsafe { msg_send![path, UTF8String] };
    let path_str = unsafe { std::ffi::CStr::from_ptr(path_cstr) }
        .to_string_lossy()
        .to_string();

    let path_encoded = base64::engine::general_purpose::URL_SAFE.encode(&path_str);
    let new_url_string = format!("https://vdv-pkpass.magicalcodewit.ch/account/db_login/callback?url={}", path_encoded);

    let string_cls = Class::get("NSString").unwrap();
    let new_url_str: *mut Object = unsafe { msg_send![string_cls, alloc] };
    let new_url_str: *mut Object = unsafe { msg_send![
        new_url_str,
        initWithBytes:new_url_string.as_ptr()
        length:new_url_string.len()
        encoding: 4
    ] };
    let url_cls = Class::get("NSURL").unwrap();
    let new_url: *mut Object = unsafe { msg_send![url_cls, alloc] };
    let new_url: *mut Object = unsafe { msg_send![new_url, initWithString:new_url_str] };

    let workspace_cls = Class::get("NSWorkspace").unwrap();
    let workspace: *mut Object = unsafe { msg_send![workspace_cls, sharedWorkspace] };
    let _:() = unsafe { msg_send![workspace, openURL:new_url] };
}

fn main() {
    let application_cls = Class::get("NSApplication").unwrap();
    let app: *mut Object = unsafe { msg_send![application_cls, sharedApplication] };
    let pool_cls = Class::get("NSAutoreleasePool").unwrap();
    let pool: *mut Object = unsafe { msg_send![pool_cls, new] };
    let _:*mut Object = unsafe { msg_send![pool, init] };

    let object_cls = Class::get("NSObject").unwrap();
    let mut delegate = ClassDecl::new("DBNavHookDelegate", object_cls).unwrap();
    let f: extern fn(&Object, Sel, u64, u64) = objc_application_open_urls;
    unsafe { delegate.add_method(sel!(application:openURLs:), f) };
    let f: extern fn(&Object, Sel, u64) = objc_did_finish_launching;
    unsafe { delegate.add_method(sel!(applicationDidFinishLaunching:), f) };
    let delegate_class = delegate.register();
    let delegate: *mut Object = unsafe { msg_send![delegate_class, alloc] };

    let _: () = unsafe { msg_send![app, setDelegate: delegate] };
    let _:() = unsafe { msg_send![app, run] };
}
