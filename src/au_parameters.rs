pub enum AUParameterNodeFFI {}

foreign_obj_type! {
    type CType = AUParameterNodeFFI;
    pub struct AUParameterNode;
    pub struct AUParameterNodeRef;
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
