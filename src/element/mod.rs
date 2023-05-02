mod element_type;

pub use element_type::ElementType;

#[derive(Debug)]
pub struct Element {
    pub element_type: ElementType,
    pub voltage: Option<f32>,
}

impl Element {
    pub fn new(element_type: ElementType, voltage: Option<f32>) -> Self {
        Self {
            element_type,
            voltage,
        }
    }
}
