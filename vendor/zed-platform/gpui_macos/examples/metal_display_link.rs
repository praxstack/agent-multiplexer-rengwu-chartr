//! Native lifetime check for the production Metal frame source (macOS 14+).
//! Requires a logged-in desktop; opens one disposable window for a few seconds.
#![allow(deprecated, dead_code, unexpected_cfgs)]

#[path = "../src/metal_display_link.rs"]
mod metal_display_link;

use cocoa::{
    appkit::{
        NSApp, NSApplication, NSApplicationActivationPolicy, NSBackingStoreType, NSView, NSWindow,
        NSWindowStyleMask,
    },
    base::{id, nil},
    foundation::{NSPoint, NSRect, NSSize},
};
use metal_display_link::MetalFrameSource;
use objc::{class, msg_send, rc::autoreleasepool, runtime::YES, sel, sel_impl};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::{Duration, Instant},
};

fn pump(seconds: f64) {
    let until = Instant::now() + Duration::from_secs_f64(seconds);
    while Instant::now() < until {
        autoreleasepool(|| unsafe {
            let date: id = msg_send![class!(NSDate), dateWithTimeIntervalSinceNow: 0.005f64];
            let run_loop: id = msg_send![class!(NSRunLoop), mainRunLoop];
            let _: () = msg_send![run_loop, runUntilDate: date];
        });
    }
}

fn main() {
    autoreleasepool(|| unsafe {
        let app = NSApp();
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
        );
        let _: () = msg_send![app, finishLaunching];
        let rect = NSRect::new(NSPoint::new(100., 100.), NSSize::new(320., 200.));
        let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            rect,
            NSWindowStyleMask::NSTitledWindowMask,
            NSBackingStoreType::NSBackingStoreBuffered,
            false,
        );
        let _: () = msg_send![window, setReleasedWhenClosed: false];
        let view = window.contentView();
        view.setWantsLayer(YES);
        let device = metal::Device::system_default().expect("Metal device");
        let layer = metal::MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
        layer.set_drawable_size(core_graphics::geometry::CGSize::new(320., 200.));
        view.setLayer(layer.as_ref() as *const _ as id);
        window.orderFrontRegardless();
        let _: () = msg_send![class!(CATransaction), flush];
        let process: id = msg_send![class!(NSProcessInfo), processInfo];
        let low_power: objc::runtime::BOOL = msg_send![process, isLowPowerModeEnabled];
        println!("low_power={}", low_power == YES);

        let callbacks = Rc::new(Cell::new(0u32));
        let presented = Rc::new(Cell::new(0u32));
        let draw = Rc::new(Cell::new(false));
        let dimensions = Rc::new(Cell::new((0u64, 0u64)));
        let queue = device.new_command_queue();
        let callback = {
            let (callbacks, presented, draw, dimensions) =
                (callbacks.clone(), presented.clone(), draw.clone(), dimensions.clone());
            let queue = queue.clone();
            move |drawable: &metal::MetalDrawableRef| {
                callbacks.set(callbacks.get() + 1);
                dimensions.set((drawable.texture().width(), drawable.texture().height()));
                if !draw.get() {
                    return;
                }
                let pass = metal::RenderPassDescriptor::new();
                let color = pass.color_attachments().object_at(0).unwrap();
                color.set_texture(Some(drawable.texture()));
                color.set_load_action(metal::MTLLoadAction::Clear);
                color.set_store_action(metal::MTLStoreAction::Store);
                color.set_clear_color(metal::MTLClearColor::new(0.1, 0.3, 0.2, 1.));
                let buffer = queue.new_command_buffer();
                buffer.new_render_command_encoder(pass).end_encoding();
                buffer.present_drawable(drawable);
                buffer.commit();
                presented.set(presented.get() + 1);
            }
        };
        let mut source = MetalFrameSource::new(&layer, callback.clone())
            .expect("CAMetalDisplayLink requires macOS 14+");
        source.start(120);
        pump(1.);
        assert!(callbacks.get() > 10, "no unused-drawable callbacks");
        assert_eq!(presented.get(), 0);
        println!("unused_callbacks={}", callbacks.get());
        draw.set(true);
        pump(1.);
        assert!(
            presented.get() > 10,
            "no sustained submission after idle: {} callbacks, {} submissions; requires an awake desktop",
            callbacks.get(),
            presented.get(),
        );
        println!("submitted_after_idle={}", presented.get());
        source.stop();
        let stopped = callbacks.get();
        pump(0.25);
        assert_eq!(callbacks.get(), stopped, "callback after pause");
        layer.set_drawable_size(core_graphics::geometry::CGSize::new(480., 300.));
        source.start(120);
        pump(1.);
        assert!(callbacks.get() > stopped + 10, "resume did not deliver frames");
        assert_eq!(dimensions.get(), (480, 300));
        println!("resize_resume_callbacks={}", callbacks.get() - stopped);
        drop(source);
        let dropped = callbacks.get();
        pump(0.25);
        assert_eq!(callbacks.get(), dropped, "callback after invalidation");

        // AppKit activation/resize draws synchronously. Pausing a Metal link is
        // insufficient: nextDrawable throws until the link is invalidated.
        // Alternate direct transactions and supplied frames to catch this boundary.
        for _ in 0..8 {
            layer.set_presents_with_transaction(true);
            autoreleasepool(|| {
                let drawable =
                    layer.next_drawable().expect("transaction drawable after invalidation");
                let buffer = queue.new_command_buffer();
                buffer.commit();
                buffer.wait_until_scheduled();
                drawable.present();
            });
            layer.set_presents_with_transaction(false);
            let before = callbacks.get();
            let mut source = MetalFrameSource::new(&layer, callback.clone()).unwrap();
            source.start(120);
            pump(0.1);
            assert!(callbacks.get() > before, "Metal link did not resume after transaction");
            drop(source);
        }
        println!("transaction_recreate_cycles=8");

        // Exercise destruction from inside the delegate invocation itself.
        let owner: Rc<RefCell<Option<MetalFrameSource>>> = Rc::new(RefCell::new(None));
        let weak_owner = Rc::downgrade(&owner);
        let closed = Rc::new(Cell::new(0u32));
        *owner.borrow_mut() = MetalFrameSource::new(&layer, {
            let closed = closed.clone();
            move |_| {
                closed.set(closed.get() + 1);
                let owner = weak_owner.upgrade().unwrap();
                let source = owner.borrow_mut().take();
                drop(source);
            }
        });
        owner.borrow_mut().as_mut().unwrap().start(120);
        pump(0.5);
        assert_eq!(closed.get(), 1, "reentrant drop did not stop callbacks");
        assert!(owner.borrow().is_none());
        println!("paused_callbacks=0 post_drop_callbacks=0 reentrant_drop_callbacks=1");
        window.close();
        let _: () = msg_send![window, release];
        println!(
            "PASS: unused updates, presentation, pause, resize/resume, transactions, drop, callback-owned drop"
        );
    });
}
