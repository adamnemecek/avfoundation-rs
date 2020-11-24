use crate::{AVAudioUnitComponent, AVAudioUnitComponentRef};
use block::Block;
use objc::runtime::{Object, BOOL, YES};
use cocoa_foundation::foundation::NSUInteger;

pub enum AVAudioUnitComponentManagerNative {}

foreign_obj_type! {
    type CType = AVAudioUnitComponentManagerNative;
    pub struct AVAudioUnitComponentManager;
    pub struct AVAudioUnitComponentManagerRef;
}

// BOOL (^)(AVAudioUnitComponent *comp, BOOL *stop)

type AudioUnitPredicate<'a> = Block<(&'a AVAudioUnitComponentRef, *mut bool), bool>;


impl AVAudioUnitComponentManager {
    pub fn shared() -> Self {
        unsafe {
            let class = class!(AVAudioUnitComponentManager);
            msg_send![class, sharedAudioUnitComponentManager]
        }
    }
}

impl AVAudioUnitComponentManagerRef {
    pub fn components_passing_test(&self) -> Vec<AVAudioUnitComponent> {
        // let mut vec = vec![];

        let block = block::ConcreteBlock::new(move |component: &AVAudioUnitComponentRef, stop: *mut BOOL| -> BOOL {
            // println!("{}", buffer.label());
            println!("stuff");
            YES
        }).copy();
        println!("1");

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
