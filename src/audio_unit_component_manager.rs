use crate::{AVAudioUnitComponent, AVAudioUnitComponentRef};
use block::Block;
use cocoa_foundation::foundation::NSUInteger;
use objc::runtime::{Object, BOOL, NO, YES};

pub enum AVAudioUnitComponentManagerNative {}

foreign_obj_type! {
    type CType = AVAudioUnitComponentManagerNative;
    pub struct AVAudioUnitComponentManager;
    pub struct AVAudioUnitComponentManagerRef;
}

// BOOL (^)(AVAudioUnitComponent *comp, BOOL *stop)

pub enum ShouldStop {
    Stop,
    Continue,
}

// type AudioUnitPredicate<'a> = Block<(&'a AVAudioUnitComponentRef, *mut bool), bool>;

impl AVAudioUnitComponentManager {
    pub fn shared() -> Self {
        unsafe {
            let class = class!(AVAudioUnitComponentManager);
            msg_send![class, sharedAudioUnitComponentManager]
        }
    }
}

impl AVAudioUnitComponentManagerRef {
    // first return value is accept, second is stop
    pub fn components_passing_test(
        &self,
        test: fn(&AVAudioUnitComponentRef) -> (bool, ShouldStop),
    ) -> Vec<AVAudioUnitComponent> {
        // let mut vec = vec![];

        let block = block::ConcreteBlock::new(
            move |component: &AVAudioUnitComponentRef, stop: *mut BOOL| -> BOOL {
                // println!("{}", buffer.label());

                let (accept, stop_test) = test(component);
                unsafe {
                    *stop = match stop_test {
                        ShouldStop::Continue => NO,
                        ShouldStop::Stop => YES,
                    };
                }

                if accept {
                    YES
                } else {
                    NO
                }
            },
        )
        .copy();

        unsafe {
            let array: *const Object = msg_send![self, componentsPassingTest: block];

            let count: NSUInteger = msg_send![array, count];
            let ret = (0..count)
                .map(|i| msg_send![array, objectAtIndex: i])
                // The elements of this array are references---we convert them to owned references
                // (which just means that we increment the reference count here, and it is
                // decremented in the `Drop` impl for `Device`)
                .map(|unit: *mut Object| msg_send![unit, retain])
                .collect();
            let () = msg_send![array, release];
            ret
        }
        // vec
    }
}
