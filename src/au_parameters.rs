use crate::{
    nsarray_to_vec,
    prelude::*,
};

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
#[repr(C)]
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
#[repr(C)]
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
pub type AUParameterObserver = block::RcBlock<(AUParameterAddress, AUValue), ()>;

///    @typedef    AUParameterRecordingObserver
///    @brief        A block called to record parameter changes as automation events.
///    @discussion
///        See the discussion of -[AUParameterNode tokenByAddingParameterRecordingObserver:].
///    @param numberEvents
///        The number of events being delivered.
///    @param events
///        The events being delivered.
// typedef void (^AUParameterRecordingObserver)(NSInteger numberEvents, const AURecordedParameterEvent *events);
use cocoa_foundation::foundation::NSInteger;
pub type AUParameterRecordingObserver =
    block::RcBlock<(NSInteger, *const AURecordedParameterEvent), ()>;

///    @typedef    AUParameterAutomationObserver
///    @brief        A block called to record parameter changes as automation events.
///    @discussion
///        See the discussion of -[AUParameterNode tokenByAddingParameterAutomationObserver:].
///    @param numberEvents
///        The number of events being delivered.
///    @param events
///        The events being delivered.

// typedef void (^AUParameterAutomationObserver)(NSInteger numberEvents, const AUParameterAutomationEvent *events);
pub type AUParameterAutomationObserver =
    block::RcBlock<(NSInteger, *const AUParameterAutomationEvent), ()>;

///    @typedef    AUParameterObserverToken
///    @brief        A token representing an installed AUParameterObserver, AUParameterRecordingObserver,
///                or AUParameterAutomationObserver.

// typedef void *AUParameterObserverToken;
pub struct AUParameterObserverToken(*const std::ffi::c_void);

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

    ///    @method   tokenByAddingParameterObserver:
    ///    @brief    Add an observer for a parameter or all parameters in a group/tree.
    ///    @discussion
    ///        An audio unit view can use an AUParameterObserver to be notified of changes
    ///        to a single parameter, or to all parameters in a group/tree.
    ///
    ///        These callbacks are throttled so as to limit the rate of redundant notifications
    ///        in the case of frequent changes to a single parameter.
    ///
    ///        This block is called in an arbitrary thread context. It is responsible for thread-safety.
    ///        It must not make any calls to add or remove other observers, including itself;
    ///        this will deadlock.
    ///
    ///        An audio unit's implementation should interact with the parameter object via
    ///        implementorValueObserver and implementorValueProvider.
    ///
    ///        Parameter observers are bound to a specific parameter instance. If this parameter is
    ///        destroyed, e.g. if the parameter tree is re-constructed, the previously set parameter
    ///        observers will no longer be valid. Clients can observe changes to the parameter tree
    ///        via KVO. See the discussion of -[AUAudioUnit parameterTree].
    ///    @param observer
    ///        A block to call after the value of a parameter has changed.
    ///    @return
    ///        A token which can be passed to removeParameterObserver: or to -[AUParameter setValue:originator:]
    /// - (AUParameterObserverToken)tokenByAddingParameterObserver:(AUParameterObserver)observer;
    pub fn token_by_adding_parameter_observer<F>(&self, observer: F) -> AUParameterObserverToken
    where
        F: Fn(AUParameterAddress, AUValue) -> () + 'static,
    {
        let block =
            block::ConcreteBlock::new(move |address: AUParameterAddress, value: AUValue| {
                observer(address, value);
            })
            .copy();
        unsafe { msg_send![self, tokenByAddingParameterObserver: block] }
    }
    ///    @method tokenByAddingParameterRecordingObserver:
    ///    @brief    Add a recording observer for a parameter or all parameters in a group/tree.
    ///    @discussion
    ///        This is a variant of tokenByAddingParameterAutomationObserver where the callback receives
    ///        AURecordedParameterEvents instead of AUParameterAutomationEvents.
    ///
    ///        This will be deprecated in favor of tokenByAddingParameterAutomationObserver in a future
    ///        release.

    // - (AUParameterObserverToken)tokenByAddingParameterRecordingObserver:(AUParameterRecordingObserver)observer;
    pub fn token_by_adding_parameter_recording_observer<F>(&self, observer: F)
    where
        F: Fn(NSInteger, *const AURecordedParameterEvent) -> () + 'static,
    {
        let block = block::ConcreteBlock::new(
            move |len: NSInteger, data: *const AURecordedParameterEvent| {
                observer(len, data);
            },
        )
        .copy();
        unsafe { msg_send![self, tokenByAddingParameterRecordingObserver: block] }
    }

    ///    @method tokenByAddingParameterAutomationObserver:
    ///    @brief    Add a recording observer for a parameter or all parameters in a group/tree.
    ///    @discussion
    ///        An audio unit host can use an AUParameterAutomationObserver or AUParameterRecordingObserver
    ///        to capture a series of changes to a parameter value, including the timing of the events, as
    ///        generated by a UI gesture in a view, for example.
    ///
    ///        Unlike AUParameterObserver, these callbacks are not throttled.
    ///
    ///        This block is called in an arbitrary thread context. It is responsible for thread-safety.
    ///        It must not make any calls to add or remove other observers, including itself;
    ///        this will deadlock.
    ///
    ///        An audio unit's engine should interact with the parameter object via
    ///        implementorValueObserver and implementorValueProvider.
    ///    @param observer
    ///        A block to call to record the changing of a parameter value.
    ///    @return
    ///        A token which can be passed to removeParameterObserver: or to -[AUParameter
    ///        setValue:originator:]

    // - (AUParameterObserverToken)tokenByAddingParameterAutomationObserver:(AUParameterAutomationObserver)observer API_AVAILABLE(macos(10.12), ios(10.0), watchos(3.0), tvos(10.0));
    fn thing2() {}

    ///    @method removeParameterObserver:
    ///    @brief    Remove an observer created with tokenByAddingParameterObserver,
    ///        tokenByAddingParameterRecordingObserver, or tokenByAddingParameterAutomationObserver.
    ///    @discussion
    ///        This call will remove the callback corresponding to the supplied token. Note that this
    ///        will block until any callbacks currently in flight have completed.
    // - (void)removeParameterObserver:(AUParameterObserverToken)token;
    fn thing3() {}
}

///    @class    AUParameterGroup
///    @brief    A group of related parameters.
///    @discussion
///        A parameter group is KVC-compliant for its children; e.g. valueForKey:@"volume" will
///
pub enum AUParameterGroupFFI {}

foreign_obj_type! {
    type CType = AUParameterGroupFFI;
    pub struct AUParameterGroup;
    pub struct AUParameterGroupRef;
    type ParentType = AUParameterNodeRef;
}

impl AUParameterGroupRef {
    /// The group's child nodes (AUParameterGroupNode).

    // @property (NS_NONATOMIC_IOSONLY, readonly) NSArray<AUParameterNode *> *children;
    pub fn children(&self) -> Vec<AUParameterNode> {
        unsafe { nsarray_to_vec(msg_send![self, children]) }
    }

    /// Returns a flat array of all parameters in the group, including those in child groups.
    // @property (NS_NONATOMIC_IOSONLY, readonly) NSArray<AUParameter *> *allParameters;
    pub fn all_parameters(&self) -> Vec<AUParameter> {
        unsafe { nsarray_to_vec(msg_send![self, allParameters]) }
    }
}

///    @class    AUParameterTree
///    @brief    The top level group node, representing all of an audio unit's parameters.
///    @discussion
///        An audio unit's parameters are organized into a tree containing groups and parameters.
///        Groups may be nested.
///
///        The tree is KVO-compliant. For example, if the tree contains a group named "LFO" ,
///        containing a parameter named rate, then "LFO.rate" refers to that parameter object, and
///        "LFO.rate.value" refers to that parameter's value.
///
///        An audio unit may choose to dynamically rearrange the tree. When doing so, it must
///        issue a KVO notification on the audio unit's parameterTree property. The tree's elements are
///        mostly immutable (except for values and implementor hooks); the only way to modify them
///        is to publish a new tree.

pub enum AUParameterTreeFFI {}

foreign_obj_type! {
    type CType = AUParameterTreeFFI;
    pub struct AUParameterTree;
    pub struct AUParameterTreeRef;
    type ParentType = AUParameterGroupRef;
}

impl AUParameterTreeRef {
    ///    @method    parameterWithAddress:
    ///    @brief    Search a tree for a parameter with a specific address.
    ///    @return
    ///        The parameter corresponding to the supplied address, or nil if no such parameter exists.
    pub fn parameter_with_address(&self, address: AUParameterAddress) -> Option<&AUParameterRef> {
        unsafe { msg_send![self, parameterWithAddress: address] }
    }

    ///    @method    parameterWithID:scope:element:
    ///    @brief    Search a tree for a specific v2 audio unit parameter.
    ///    @discussion
    ///        V2 audio units publish parameters identified by a parameter ID, scope, and element.
    ///        A host that knows that it is dealing with a v2 unit can locate parameters using this method,
    ///        for example, for the Apple-supplied system audio units.
    ///    @return
    ///        The parameter corresponding to the supplied ID/scope/element, or nil if no such parameter
    ///        exists, or if the audio unit is not a v2 unit.
    pub fn parameter_with_id(&self, address: AudioUnitParameterID) -> Option<&AUParameterRef> {
        unsafe { msg_send![self, parameterWithAddress: address] }
    }
}

///    @class    AUParameter
///    @brief    A node representing a single parameter.

pub enum AUParameterFFI {}

foreign_obj_type! {
    type CType = AUParameterFFI;
    pub struct AUParameter;
    pub struct AUParameterRef;
    type ParentType = AUParameterNodeRef;
}

impl AUParameterRef {
    /// The parameter's minimum value.
    // @property (NS_NONATOMIC_IOSONLY, readonly) AUValue minValue;
    pub fn min_value(&self) -> AUValue {
        unsafe { msg_send![self, minValue] }
    }

    /// The parameter's maximum value.
    // @property (NS_NONATOMIC_IOSONLY, readonly) AUValue maxValue;

    pub fn max_value(&self) -> AUValue {
        unsafe { msg_send![self, maxValue] }
    }

    /// The parameter's unit of measurement.
    // @property (NS_NONATOMIC_IOSONLY, readonly) AudioUnitParameterUnit unit;
    pub fn unit(&self) -> crate::AudioUnitParameterUnit {
        unsafe { msg_send![self, unit] }
    }

    /// A localized name for the parameter's unit. Supplied by the AU if kAudioUnitParameterUnit_CustomUnit; else by the framework.
    // @property (NS_NONATOMIC_IOSONLY, readonly, copy, nullable) NSString *unitName;
    pub fn unit_name(&self) -> AUValue {
        todo!()
    }

    /// Various details of the parameter.
    // @property (NS_NONATOMIC_IOSONLY, readonly) AudioUnitParameterOptions flags;
    pub fn flags(&self) -> AUValue {
        todo!()
    }

    /// The parameter's address.
    // @property (NS_NONATOMIC_IOSONLY, readonly) AUParameterAddress address;
    pub fn address(&self) -> AUParameterAddress {
        unsafe { msg_send![self, address] }
    }

    /// For parameters with kAudioUnitParameterUnit_Indexed, localized strings corresponding
    ///    to the values.
    // @property (NS_NONATOMIC_IOSONLY, readonly, copy, nullable) NSArray<NSString *> *valueStrings;
    pub fn value_strings(&self) -> AUValue {
        todo!()
    }

    ///    @brief        Parameters whose values may change as a side effect of this parameter's value
    ///                  changing.
    ///    @discussion
    ///        Each array value is an NSNumber representing AUParameterAddress.

    // @property (NS_NONATOMIC_IOSONLY, readonly, copy, nullable) NSArray<NSNumber *> *dependentParameters;
    pub fn dependent_parameters(&self) -> AUValue {
        todo!()
    }

    /// The parameter's current value.
    // @property (NS_NONATOMIC_IOSONLY) AUValue value;
    pub fn value(&self) -> AUValue {
        unsafe { msg_send![self, value] }
    }

    ///
    ///    @brief    Set the parameter's value, avoiding redundant notifications to the originator.
    ///    @discussion
    ///            Bridged to the v2 function AudioUnitSetParameter.

    // - (void)setValue:(AUValue)value originator:(AUParameterObserverToken __nullable)originator;
    pub fn set_value(&self, value: AUValue, originator: AUParameterObserverToken) {
        unsafe {
            msg_send![
                self,
                setValue: value
                originator: originator.0
            ]
        }
    }

    ///    @brief    Convenience for setValue:originator:atHostTime:eventType:
    ///    @discussion
    ///            Bridged to the v2 function AudioUnitSetParameter.

    // - (void)setValue:(AUValue)value originator:(AUParameterObserverToken __nullable)originator atHostTime:(uint64_t)hostTime;
    pub fn originator(&self) -> AUValue {
        todo!()
    }

    ///    @brief    Set the parameter's value, preserving the host time of the gesture that initiated the
    ///            change, and associating an event type such as touch/release.
    ///    @discussion
    ///        In general, this method should only be called from a user interface. It initiates a change
    ///        to a parameter in a way that captures the gesture such that it can be recorded later --
    ///        any AUParameterAutomationObservers will receive the host time and event type associated
    ///        with the parameter change.
    ///
    ///        From an audio playback engine, a host should schedule automated parameter changes through
    ///        AUAudioUnit's scheduleParameterBlock.
    ///
    ///        Bridged to the v2 function AudioUnitSetParameter.

    // - (void)setValue:(AUValue)value originator:(AUParameterObserverToken __nullable)originator atHostTime:(uint64_t)hostTime eventType:(AUParameterAutomationEventType)eventType API_AVAILABLE(macos(10.12), ios(10.0), watchos(3.0), tvos(10.0));
    pub fn set_value_at(
        &self,
        value: AUValue,
        originator: AUParameterObserverToken,
        at: u64,
        event_type: AUParameterAutomationEventType,
    ) {
        unsafe {
            let _: () = msg_send![self, setValue: value
                                      originator: originator
                                      atHostTime: at
                                       eventType: event_type];
        }
    }

    ///    @brief Get a textual representation of a value for the parameter. Use value==nil to use the
    ///           current value. Bridged to the v2 property kAudioUnitProperty_ParameterStringFromValue.
    ///    @discussion
    ///        This is currently only supported for parameters whose flags include
    ///        kAudioUnitParameterFlag_ValuesHaveStrings.

    // - (NSString *)stringFromValue:(const AUValue *__nullable)value;
    pub fn string_for_value(&self, value: Option<AUValue>) -> &str {
        todo!()
    }

    ///    @brief Convert a textual representation of a value to a numeric one.
    ///    @discussion
    ///        This is currently only supported for parameters whose flags include
    ///        kAudioUnitParameterFlag_ValuesHaveStrings.
    // - (AUValue)valueFromString:(NSString *)string;
    pub fn value_for_string(&self, string: &str) -> &str {
        let s = crate::nsstring_from_str(string);
        unsafe { crate::nsstring_as_str(msg_send![self, valueForString: s]) }
    }
}
