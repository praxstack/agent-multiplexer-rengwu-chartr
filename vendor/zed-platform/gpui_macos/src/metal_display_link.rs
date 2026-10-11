//! Main-run-loop frame delivery with a drawable already supplied by Metal.
//! The CoreVideo fallback lives in display_link.rs and retains its own lifetime rules.

use cocoa::base::{id, nil};
use foreign_types::ForeignTypeRef;
use objc::{
    Encode, Encoding, class,
    declare::ClassDecl,
    msg_send,
    rc::{StrongPtr, autoreleasepool},
    runtime::{BOOL, Class, NO, Object, Sel, YES},
    sel, sel_impl,
};
use std::{ffi::c_void, rc::Rc, sync::LazyLock};

type FrameCallback = Rc<dyn Fn(&metal::MetalDrawableRef)>;
const CALLBACK: &str = "frameCallback";
const MAXIMUM_FPS: &str = "maximumFramesPerSecond";

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    static NSRunLoopCommonModes: id;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct FrameRateRange {
    minimum: f32,
    maximum: f32,
    preferred: f32,
}

// CAFrameRateRange consists of three floats in this order on both macOS architectures.
unsafe impl Encode for FrameRateRange {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("{CAFrameRateRange=fff}") }
    }
}

fn preferred_range(low_power: bool, maximum_fps: u32) -> FrameRateRange {
    if !low_power {
        return FrameRateRange::default();
    }
    let maximum = if maximum_fps == 0 { 60.0 } else { maximum_fps.min(60) as f32 };
    FrameRateRange { minimum: maximum.min(30.0), maximum, preferred: maximum }
}

unsafe fn update_preferred_range(link: id, maximum_fps: u32) {
    // This code is only reached with CAMetalDisplayLink (macOS 14+), where the
    // effective Low Power Mode getter is available. Check on resume and on ticks
    // so a live power-policy transition doesn't need a process-wide observer.
    unsafe {
        let process: id = msg_send![class!(NSProcessInfo), processInfo];
        let low_power: BOOL = msg_send![process, isLowPowerModeEnabled];
        let wanted = preferred_range(low_power == YES, maximum_fps);
        let current: FrameRateRange = msg_send![link, preferredFrameRateRange];
        if current != wanted {
            let _: () = msg_send![link, setPreferredFrameRateRange: wanted];
        }
    }
}

static DELEGATE_CLASS: LazyLock<&'static Class> = LazyLock::new(|| unsafe {
    let mut class = ClassDecl::new("ChartrMetalDisplayLinkDelegate", class!(NSObject)).unwrap();
    class.add_ivar::<*mut c_void>(CALLBACK);
    class.add_ivar::<u32>(MAXIMUM_FPS);
    class.add_method(
        sel!(metalDisplayLink:needsUpdate:),
        needs_update as extern "C" fn(&Object, Sel, id, id),
    );
    class.add_method(sel!(dealloc), dealloc as extern "C" fn(&Object, Sel));
    class.register()
});

extern "C" fn needs_update(delegate: &Object, _: Sel, link: id, update: id) {
    autoreleasepool(|| unsafe {
        update_preferred_range(link, *delegate.get_ivar(MAXIMUM_FPS));
        let raw: *mut c_void = *delegate.get_ivar(CALLBACK);
        // The callback may close its own window and release the source/delegate.
        // Clone before invoking it; don't access the delegate again afterward.
        let callback = (&*raw.cast::<FrameCallback>()).clone();
        let drawable: id = msg_send![update, drawable];
        if !drawable.is_null() {
            callback(metal::MetalDrawableRef::from_ptr(drawable.cast()));
        }
    });
}

extern "C" fn dealloc(delegate: &Object, _: Sel) {
    unsafe {
        let raw: *mut c_void = *delegate.get_ivar(CALLBACK);
        drop(Box::from_raw(raw.cast::<FrameCallback>()));
        let _: () = msg_send![super(delegate, class!(NSObject)), dealloc];
    }
}

/// Owned by one window, and accessed exclusively on the main thread.
pub(crate) struct MetalFrameSource {
    link: Option<StrongPtr>,
    delegate: StrongPtr,
    // Keep the layer alive until after link invalidation, even during window close.
    _layer: metal::MetalLayer,
}

impl MetalFrameSource {
    /// None selects the existing CVDisplayLink implementation on macOS 11–13.
    pub fn new(
        layer: &metal::MetalLayerRef,
        callback: impl Fn(&metal::MetalDrawableRef) + 'static,
    ) -> Option<Self> {
        let class = Class::get("CAMetalDisplayLink")?;
        unsafe {
            let link: id = msg_send![class, alloc];
            let link: id = msg_send![link, initWithMetalLayer: layer.as_ptr()];
            if link.is_null() {
                return None;
            }
            let link = StrongPtr::new(link);
            let delegate: id = msg_send![*DELEGATE_CLASS, new];
            let callback: FrameCallback = Rc::new(callback);
            (*delegate).set_ivar(CALLBACK, Box::into_raw(Box::new(callback)).cast::<c_void>());
            (*delegate).set_ivar(MAXIMUM_FPS, 0u32);
            let delegate = StrongPtr::new(delegate);
            let _: () = msg_send![*link, setDelegate: *delegate];
            let _: () = msg_send![*link, setPaused: YES];
            let run_loop: id = msg_send![class!(NSRunLoop), mainRunLoop];
            let _: () = msg_send![*link, addToRunLoop: run_loop forMode: NSRunLoopCommonModes];
            Some(Self { link: Some(link), delegate, _layer: layer.to_owned() })
        }
    }

    pub fn start(&mut self, maximum_fps: u32) {
        if let Some(link) = &self.link {
            unsafe {
                (**self.delegate).set_ivar(MAXIMUM_FPS, maximum_fps);
                update_preferred_range(**link, maximum_fps);
                let _: () = msg_send![**link, setPaused: NO];
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(link) = &self.link {
            unsafe {
                let _: () = msg_send![**link, setPaused: YES];
            }
        }
    }
}

impl Drop for MetalFrameSource {
    fn drop(&mut self) {
        // Invalidate exactly once. Repeated invalidation crashes on the macOS
        // version used to investigate this change; temporary stops only pause.
        if let Some(link) = self.link.take() {
            unsafe {
                let _: () = msg_send![*link, setDelegate: nil];
                let _: () = msg_send![*link, invalidate];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_policy_respects_slow_displays_and_restores_system_preference() {
        assert_eq!(preferred_range(true, 120).maximum, 60.0);
        assert_eq!(
            preferred_range(true, 24),
            FrameRateRange { minimum: 24.0, maximum: 24.0, preferred: 24.0 }
        );
        assert_eq!(preferred_range(true, 0).maximum, 60.0);
        assert_eq!(preferred_range(false, 120), FrameRateRange::default());
    }
}
