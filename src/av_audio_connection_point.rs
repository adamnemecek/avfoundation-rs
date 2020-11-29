use crate::{
    AVAudioNodeBus,
    AVAudioNodeRef,
};

/// @class AVAudioConnectionPoint
/// @abstract A representation of either a source or destination connection point in AVAudioEngine.
/// @discussion
///     AVAudioConnectionPoint describes either a source or destination connection point (node, bus)
///     in AVAudioEngine's graph.
///
///     Instances of this class are immutable.
pub enum AVAudioConnectionPointFFI {}

foreign_obj_type! {
    type CType = AVAudioConnectionPointFFI;
    pub struct AVAudioConnectionPoint;
    pub struct AVAudioConnectionPointRef;
}

impl AVAudioConnectionPoint {
    // - (instancetype)init NS_UNAVAILABLE;
    pub fn new() -> Self {
        panic!("unavailable")
    }

    /// @method initWithNode:bus:
    /// @abstract Create a connection point object.
    /// @param node the source or destination node
    /// @param bus the output or input bus on the node
    /// @discussion
    ///     If the node is nil, this method fails (returns nil).
    // - (instancetype)initWithNode:(AVAudioNode *)node bus:(AVAudioNodeBus)bus NS_DESIGNATED_INITIALIZER;
    pub fn with_node(node: &AVAudioNodeRef, bus: AVAudioNodeBus) -> Self {
        unsafe {
            let class = class!(AVAudioConnectionPoint);
            let alloc: *const AVAudioConnectionPointFFI = msg_send![class, alloc];
            msg_send![alloc, initWithNode: node bus: bus]
        }
    }
}

impl AVAudioConnectionPointRef {
    /// @property node
    /// @abstract Returns the node in the connection point.
    /// @property (nonatomic, readonly, weak) AVAudioNode *node;

    pub fn node(&self) -> &AVAudioNodeRef {
        unsafe { msg_send![self, node] }
    }

    /// @property bus
    /// @abstract Returns the bus on the node in the connection point.
    /// @property (nonatomic, readonly) AVAudioNodeBus bus;
    pub fn bus(&self) -> AVAudioNodeBus {
        unsafe { msg_send![self, bus] }
    }
}
