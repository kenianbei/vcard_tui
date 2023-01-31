use crate::state::popup::property_n::PropertyNState;

#[derive(Clone)]
pub struct VcardAddState<'a> {
    pub property_n: PropertyNState<'a>,
}

impl<'a> Default for VcardAddState<'a> {
    fn default() -> Self {
        Self {
            property_n: PropertyNState::default(),
        }
    }
}
