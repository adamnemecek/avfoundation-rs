pub enum NSErrorFFI {}

foreign_obj_type! {
    type CType = NSErrorFFI;
    pub struct NSError;
    pub struct NSErrorRef;
}
