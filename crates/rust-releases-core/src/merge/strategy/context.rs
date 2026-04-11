use crate::merge::ContextMerge;

/// Trivial context merge for the unit type.
pub struct UnitContext;

impl ContextMerge<(), ()> for UnitContext {
    type Output = ();

    fn merge_context(&self, _left: (), _right: ()) {}
}
