
///    @typedef    AUValue
///    @brief      A value of an audio unit parameter.

pub type AUValue = f32;

///    @typedef    AUParameterAddress
///    @brief        Numeric identifier for audio unit parameter.
///    @discussion
///        Note that parameter addresses are not necessarily persistent, unless the individual audio
///        unit documents a promise to maintain its addressing scheme. Hosts should bind to parameters'
///        key paths.

pub type AUParameterAddress = u64;

///    @enum        AUParameterAutomationEventType
///    @brief       Identifies the different types of parameter automation events.
///
///    @discussion
///        Audio Units may generate parameter changes from their user interfaces. Hosts may attach
///        significance to the beginning and end of a UI gesture (typically touching and releasing
///        a fader). These gestures are conveyed through these types of automation events.
///
///    @constant AUParameterAutomationEventTypeValue
///        The event contains an updated value for the parameter.
///    @constant AUParameterAutomationEventTypeTouch
///        The event marks an initial "touch" gesture on a UI element.
///    @constant AUParameterAutomationEventTypeRelease
///        The event marks a final "release" gesture on a UI element.

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUParameterAutomationEventType {
    Value = 0,
    Touch = 1,
    Release = 2,
}

///    @typedef    AURecordedParameterEvent
///    @brief      An event recording the changing of a parameter at a particular host time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AURecordedParameterEvent {
    ///< The host time at which the event occurred.
    pub host_time: u64,
    ///< The address of the parameter whose value changed.
    pub address: AUParameterAddress,
    ///< The value of the parameter at that time.
    pub value: AUValue,
}

///    @typedef    AUParameterAutomationEvent
///    @brief      An event recording the changing of a parameter, possibly including events
///                such as touch and release gestures, at a particular host time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUParameterAutomationEvent {
     ///< The host time at which the event occurred.
    pub host_time: u64,
    ///< The address of the parameter whose value changed.
    pub address: AUParameterAddress,
    ///< The value of the parameter at that time.
    pub value: AUValue,
    ///< The type of the event.
    pub event_type: AUParameterAutomationEventType,
    pub reserved: u64,
}

pub enum AUParameterNodeFFI {}

foreign_obj_type! {
    type CType = AUParameterNodeFFI;
    pub struct AUParameterNode;
    pub struct AUParameterNodeRef;
}

pub enum AUParameterFFI {}

foreign_obj_type! {
    type CType = AUParameterFFI;
    pub struct AUParameter;
    pub struct AUParameterRef;
    type ParentType = AUParameterNodeRef;
}

pub enum AUParameterGroupFFI {}

foreign_obj_type! {
    type CType = AUParameterGroupFFI;
    pub struct AUParameterGroup;
    pub struct AUParameterGroupRef;
    type ParentType = AUParameterNodeRef;
}

pub enum AUParameterTreeFFI {}

foreign_obj_type! {
    type CType = AUParameterTreeFFI;
    pub struct AUParameterTree;
    pub struct AUParameterTreeRef;
    type ParentType = AUParameterGroupRef;
}
