use crate::merge::MergeContext;

/// Trivial context merge for the unit type.
pub struct UnitContext;

impl MergeContext<(), ()> for UnitContext {
    type Output = ();

    fn merge_context(&self, _left: (), _right: ()) {}
}
