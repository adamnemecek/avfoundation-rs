use crate::{
    AVAudioUnitComponent,
    AVAudioUnitComponentRef,
    AudioComponentDescription,
};
use block::Block;
use cocoa_foundation::foundation::NSUInteger;
use objc::runtime::{
    Object,
    BOOL,
    NO,
    YES,
};

pub enum AVAudioUnitComponentManagerFFI {}

foreign_obj_type! {
    type CType = AVAudioUnitComponentManagerFFI;
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
        let block = block::ConcreteBlock::new(
            move |component: &AVAudioUnitComponentRef, stop: *mut BOOL| -> BOOL {
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
            crate::nsarray_to_vec(array)
        }
    }

    pub fn components_matching_description(
        &self,
        desc: AudioComponentDescription,
    ) -> Vec<AVAudioUnitComponent> {
        unsafe {
            let array: *const Object = msg_send![self, componentsMatchingDescription: desc];
            crate::nsarray_to_vec(array)
        }
    }
}
