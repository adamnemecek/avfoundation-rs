use crate::{
    AUAudioUnitRef,
    DispatchQueue,
};
// //
// //  AUViewController.h
// //	Framework: CoreAudioKit
// //
// //  Copyright © 2015 Apple Inc. All rights reserved.
// //
// #if __OBJC2__

// #import <AudioUnit/AudioUnit.h>
// #import <Foundation/NSExtensionRequestHandling.h>

// #if TARGET_OS_IPHONE
// #import <UIKit/UIKit.h>
// typedef UIViewController AUViewControllerBase;
// #else
// #import <AppKit/AppKit.h>
// typedef NSViewController AUViewControllerBase;
// #endif

// NS_ASSUME_NONNULL_BEGIN

// API_AVAILABLE(macos(10.11), ios(9.0)) API_UNAVAILABLE(watchos)
// @interface AUViewController : AUViewControllerBase <NSExtensionRequestHandling>
// @end

pub enum AUViewControllerFFI {}

foreign_obj_type! {
    type CType = AUViewControllerFFI;
    pub struct AUViewController;
    pub struct AUViewControllerRef;
}

// /*!	@class	AUAudioUnitViewConfiguration
// 	@brief	Properties of the configuration that a host uses to embed the view of an audio unit.
// 	@discussion
// 		Hosts may support embedding the view of an audio unit in different configurations. These
// 		configurations may vary in the size reserved for the audio unit's view and the additional
// 		control surfaces that are displayed along with it. The host can propose several view
// 		configurations and the audio unit should report the ones which it supports.

// 		See the documentation for supportedViewConfigurations.
// */
pub enum AUAudioUnitViewConfigurationFFI {}

foreign_obj_type! {
    type CType = AUAudioUnitViewConfigurationFFI;
    pub struct AUAudioUnitViewConfiguration;
    pub struct AUAudioUnitViewConfigurationRef;
}

impl AUAudioUnitViewConfiguration {
    // API_AVAILABLE(macos(10.13), ios(11.0)) API_UNAVAILABLE(watchos)
    // @interface AUAudioUnitViewConfiguration : NSObject <NSSecureCoding>

    // /*!	@method		initWithWidth
    // 	@brief		Designated initializer.
    // 	@param		width
    // 		The width associated with this view configuration.
    // 	@param		height
    // 		The height associated with this view configuration.
    // 	@param		hostHasController
    // 		This property controls whether the host shows its own control surface in this view
    // 		configuration.
    // 	@return		Returns the newly created view configuration object.
    // */
    // - (instancetype)initWithWidth:(CGFloat)width height:(CGFloat)height hostHasController:(BOOL)hostHasController;
}

impl AUAudioUnitViewConfigurationRef {
    // /*!	@property	width
    // 	@brief		The width of the view, measured in points.
    // 	@discussion
    // 		Setting the width to 0 will match any width.
    // */
    // @property (NS_NONATOMIC_IOSONLY, readonly) CGFloat width;

    // /*!	@property	height
    // 	@brief		The height of the view, measured in points.
    // 	@discussion
    // 		Setting the height to 0 will match any height.
    // */
    // @property (NS_NONATOMIC_IOSONLY, readonly) CGFloat height;

    // /*!	@property	hostHasController
    // 	@brief		Boolean property specifying whether the host displays its own control surface
    // 				when showing the view of the audio unit.
    // */
    // @property (NS_NONATOMIC_IOSONLY, readonly) BOOL hostHasController;

    // @end
}

use cocoa_foundation::base::{
    id,
    nil,
};

// struct Dispatcher {
//     queue: DispatchQueue
// }

// impl Dispatcher {
//     pub fn new() -> Self {
//         Self {
//             queue: DispatchQueue::new("dispatcher")
//         }
//     }

//     pub fn async_fn() {

//     }
// }

// use std::sync::Once;

// static DISPATCHER: Once = Once::new();
// fn get_dispatcher() -> Dispatcher

struct SyncController {
    inner: *mut NSViewControllerFFI,
}

unsafe impl Send for SyncController {}
unsafe impl Sync for SyncController {}

pub enum NSViewControllerFFI {}

foreign_obj_type! {
    type CType = NSViewControllerFFI;
    pub struct NSViewController;
    pub struct NSViewControllerRef;
}

// unsafe impl Send for NSViewControllerFFI {}
// unsafe impl Sync for NSViewControllerFFI {}

// unsafe impl Send for NSViewController {}
// unsafe impl Sync for NSViewController {}

// unsafe impl Send for NSViewControllerRef {}
// unsafe impl Sync for NSViewControllerRef {}

pub type RequestAUAudioUnitViewController<'a> =
    block::RcBlock<(Option<&'a NSViewControllerRef>,), ()>;

#[derive(Debug)]
pub enum AVFoundationEvent {
    AVAudioUnitHandler(Result<crate::AVAudioUnit, crate::NSError>),
    RequestViewController(Option<NSViewController>),
}

// struct CB<F: FnOnce(Option<NSViewController>) + 'static + Send> {
//     f: F
// }

// impl<F: FnOnce(Option<NSViewController>) + 'static + Send> Clone for CB<F> {
//     fn clone(&self) -> Self {
//         todo!()
//     }
// }

// impl<F: FnOnce(Option<NSViewController>) + 'static + Send> CB<F> {
//     pub fn new(f: F) -> Self {
//         Self {
//             f
//         }
//     }
// }

// impl<F: FnOnce(Option<NSViewController>) + 'static + Send> FnOnce<(f32,)> for CB<F> {
//     type Output = i32;
//     extern "rust-call" fn call_once(self, args: (f32, )) -> Self::Output {
//         todo!()
//     }
// }

// impl<F: FnOnce(Option<NSViewController>) + 'static + Send> FnMut<(f32,)> for CB<F> {

//     extern "rust-call" fn call_mut(&mut self, args: (f32, )) -> Self::Output {
//         todo!()
//     }
// }

// struct MainTreadSafe<T>(pub T);

// unsafe impl<T> Send for MainTreadSafe<T> {}

// impl<T> std::ops::Deref for MainTreadSafe<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// static A: u32 = 0;
// struct Dispatcher {}

// impl Dispatcher {
//     fn send(&self, e: Option<&NSViewControllerRef>) {}
// }

// static DISPATCHER: Dispatcher = Dispatcher {};

// pub fn register(tx: std::sync::mpsc::Sender<AVFoundationEvent>) {}

impl AUAudioUnitRef {
    // API_AVAILABLE(macos(10.12), ios(9.0)) API_UNAVAILABLE(watchos)
    // @interface AUAudioUnit (AUAudioUnit_ViewController)
    // /*!	@method	requestViewControllerWithCompletionHandler:
    // 	@brief	Obtains an audio unit's view controller (and thereby a view).
    // 	@discussion
    // 		Asynchronously requests the audio unit's view controller. This method will later call the
    // 		completion handler, in a thread/dispatch queue context internal to the implementation, with
    // 		a view controller, or nil in the case of an audio unit without a custom view controller.
    // */
    // - (void)requestViewControllerWithCompletionHandler:(void (^)(AUViewControllerBase * __nullable viewController))completionHandler;
    pub fn request_view_controller(&self, block: RequestAUAudioUnitViewController) {
        unsafe { msg_send![self, requestViewControllerWithCompletionHandler: block] }
    }

    pub fn request_view_controller_fn(&self, f: impl Fn(Option<NSViewController>) + 'static) {
        let block = block::ConcreteBlock::new(move |controller: Option<&NSViewControllerRef>| {
            f(controller.map(|x| x.to_owned()))
        })
        .copy();
        self.request_view_controller(block);
    }

    pub fn request_view_controller_fn1(&self, f: impl Fn(Option<&NSViewControllerRef>) + 'static) {
        let block = block::ConcreteBlock::new(move |controller: Option<&NSViewControllerRef>| {
            // f(controller.map(|x| x.to_owned()))
            f(controller)
        })
        .copy();
        self.request_view_controller(block);
    }

    pub fn request_view_controller_async_fn(
        &self,
        f: impl FnOnce(Option<&NSViewControllerRef>) + 'static,
    ) {
        // let queue = DispatchQueue::new("hello");
        // let cb = move |controller: Option<&NSViewControllerRef>| {
        //     let vc = controller.map(|x| x.to_owned());
        //     DispatchQueue::current().dispatch_async(move || {
        //         f(vc);
        //     });
        //     // queue.dispatch_async(move || {
        //         // f(vc);
        //     // });
        // };
        // let block = block::ConcreteBlock::new(move |controller: Option<&NSViewControllerRef>| {

        //     DispatchQueue::current().dispatch_async(move || {
        //         let vc = controller.map(|x| x.to_owned());
        //         f(vc);
        //     });
        //     cb(controller);

        // })
        // .copy();
        // unsafe { msg_send![self, requestViewControllerWithCompletionHandler: block] }

        // self.request_view_controller_fn1(|vc| {

        //     DispatchQueue::current().dispatch_async(move || {
        //         f(vc);
        //     });
        // });

        // fn fn_async(f: impl Fn() + Send) {
        //     dispatch::Queue::main().exec_async(|| {
        //         f()
        //     })

        // }
        // let cb = CB::new(f);

        // let ff = MainTreadSafe(f);
        // let queue = dispatch::Queue::main();
        // let queue = DispatchQueue::main();

        // self.request_view_controller_fn1(move |vc| {
        //     // let z = vc.map(|x| x.to_owned()).unwrap();

        //     queue.dispatch_async(move || {
        //         // let z = A;
        //         DISPATCHER.send(vc);
        //         // cb.call_once((10.0,));
        //         // ff(Some(&z));
        //     });
        // });
        // queue.exec_async(work)
        // self.request_view_controller_fn(move |vc| {
        // let queue = dispatch::Queue::main();
        // let cb = CB::new();

        // queue.exec_async(|| {
        //     cb.call_once((10.0,));
        // });
        // queue.exec_async(|| {
        //         f(vc);
        // });
        // });
    }
    pub fn request_view_controller_tx1(&self, tx: &std::sync::mpsc::Sender<AVFoundationEvent>) {
        // let tx = tx.clone();

        // // let block = block::ConcreteBlock::new(move |vc: Option<&NSViewControllerRef>| {
        // //     DispatchQueue::current().dispatch_async(move || {
        // //         let vc = vc.map(|x| x.to_owned());
        // //         let r = tx.send(AVFoundationEvent::RequestViewController(vc));
        // //         println!("view controller tx {:?}", r);

        // //     });
        // // })
        // // .copy();

        // self.request_view_controller(block);
    }

    pub fn request_view_controller_tx(&self, tx: &std::sync::mpsc::Sender<AVFoundationEvent>) {
        let tx = tx.clone();

        let block = block::ConcreteBlock::new(move |vc: Option<&NSViewControllerRef>| {
            // queue.dispatch_async(move || {
            let vc = vc.map(|x| x.to_owned());
            let r = tx.send(AVFoundationEvent::RequestViewController(vc));
            println!("view controller tx {:?}", r);

            // });
        })
        .copy();

        self.request_view_controller(block);
    }

    // /*!	@method		supportedViewConfigurations
    // 	@param		availableViewConfigurations
    // 		The list of all available view configurations supported by the host.
    // 	@return
    // 		A set of indices of view configurations from the availableViewConfigurations array that the
    // 		audio unit supports.
    // 	@brief      Query the list of supported view configurations.
    // 	@discussion
    // 		The host can query the audio unit for all the view configurations it supports.
    // 		Hosts can support multiple configurations in which they can display the user interfaces of
    // 		audio units (for example: full screen, normal, live mode, etc). These configurations can be
    // 		of different sizes and the host might display its own control surfaces along with the view
    // 		of the audio unit. The host will call this method and pass an array of supported
    // 		configurations.

    // 		The audio unit should override this method and implement its own logic to report all the
    // 		view configurations it supports. The size of the view in the selected configuration should
    // 		be large enough to fit the view of the audio unit, otherwise it might be truncated and a
    // 		scroll bar might be necessary to navigate it.

    // 		In case an empty set is returned from this method, it is considered that the plugin only
    // 		supports the largest available view configuration.
    // */
    // - (NSIndexSet *)supportedViewConfigurations:(NSArray<AUAudioUnitViewConfiguration *> *)availableViewConfigurations API_AVAILABLE(macosx(10.13), ios(11.0)) API_UNAVAILABLE(watchos, tvos);

    // /*!	@method		selectViewConfiguration
    // 	@param		viewConfiguration
    //         The requested view configuration.

    //         The view configuration passed to this method should be one which was indicated as supported
    //         via supportedViewConfigurations. If any other, unsupported, view configuration is passed or
    //         if supportedViewConfigurations returns an empty set, the audio unit implementation should
    //         fall back to its default (largest available) view configuration.

    // 	@brief		Request a view configuration from the audio unit.
    // 	@discussion
    // 		The host can use this method to switch the audio unit's view into a new configuration.
    // 		Audio Units should override this method with the logic needed to adapt their view controller
    // 		to the requested configuration.
    // */
    // - (void)selectViewConfiguration:(AUAudioUnitViewConfiguration *)viewConfiguration API_AVAILABLE(macosx(10.13), ios(11.0)) API_UNAVAILABLE(watchos, tvos);

    // @end

    // NS_ASSUME_NONNULL_END

    // #endif
}
