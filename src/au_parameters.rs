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

///    @typedef    AUParameterObserver
///    @brief      A block called to signal that the value of a parameter has changed.
///    @discussion
///        See the discussion of -[AUParameterNode tokenByAddingParameterObserver:].
///    @param address
///        The address of the parameter whose value changed.
///    @param value
///        The current value of the parameter.

// typedef void (^AUParameterObserver)(AUParameterAddress address, AUValue value);
// pub type AUParameterObserver = Fn(AUParameterAddress, AUValue) -> ();

///    @typedef    AUParameterRecordingObserver
///    @brief        A block called to record parameter changes as automation events.
///    @discussion
///        See the discussion of -[AUParameterNode tokenByAddingParameterRecordingObserver:].
///    @param numberEvents
///        The number of events being delivered.
///    @param events
///        The events being delivered.

// typedef void (^AUParameterRecordingObserver)(NSInteger numberEvents, const AURecordedParameterEvent *events);

///    @typedef    AUParameterAutomationObserver
///    @brief        A block called to record parameter changes as automation events.
///    @discussion
///        See the discussion of -[AUParameterNode tokenByAddingParameterAutomationObserver:].
///    @param numberEvents
///        The number of events being delivered.
///    @param events
///        The events being delivered.

// typedef void (^AUParameterAutomationObserver)(NSInteger numberEvents, const AUParameterAutomationEvent *events);

///    @typedef    AUParameterObserverToken
///    @brief        A token representing an installed AUParameterObserver, AUParameterRecordingObserver,
///                or AUParameterAutomationObserver.

// typedef void *AUParameterObserverToken;

///    @class        AUParameterNode
///    @brief        A node in an audio unit's tree of parameters.
///    @discussion
///        Nodes are instances of either AUParameterGroup or AUParameter.

pub enum AUParameterNodeFFI {}

foreign_obj_type! {
    type CType = AUParameterNodeFFI;
    pub struct AUParameterNode;
    pub struct AUParameterNodeRef;
}

impl AUParameterNodeRef {
    ///    @property    identifier
    ///    @brief       A non-localized, permanent name for a parameter or group.
    ///    @discussion
    ///        The identifier must be unique for all child nodes under any given parent. From release to
    ///        release, an audio unit must not change its parameters' identifiers; this will invalidate any
    ///        hosts' documents that refer to the parameters.
    pub fn identifier(&self) -> &str {
        unsafe { crate::nsstring_as_str(msg_send![self, identifier]) }
    }

    ///    @property    keyPath
    ///    @brief       Generated by concatenating the identifiers of a node's parents with its own.
    ///    @discussion
    ///        Unless an audio unit specifically documents that its parameter addresses are stable and
    ///        persistent, hosts, when recording parameter values, should bind to the keyPath.
    ///
    ///        The individual node identifiers in a key path are separated by periods. (".")
    ///        
    ///        Passing a node's keyPath to -[tree valueForKeyPath:] should return the same node.
    pub fn key_path(&self) -> &str {
        unsafe { crate::nsstring_as_str(msg_send![self, keyPath]) }
    }

    ///    @property    displayName
    ///    @brief        A localized name to display for the parameter.
    pub fn display_name(&self) -> &str {
        unsafe { crate::nsstring_as_str(msg_send![self, displayName]) }
    }

    ///    @method       displayNameWithLength:
    ///    @brief        A version of displayName possibly abbreviated to the given desired length, in characters.
    ///    @discussion
    ///        The default implementation simply returns displayName.
    pub fn display_name_with_length(&self, maximum_length: u64) -> &str {
        unsafe { crate::nsstring_as_str(msg_send![self, displayNameWithLength: maximum_length]) }
    }
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
