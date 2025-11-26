// IVariable.rs

// Author  : Alexander Nava 
// Contact : Alexander.Nava.Contact@Gmail.com
// License : For personal use excluding any artificail or machine learning this is licensed under MIT license.
// License : For commercial software(excluding derivative work to make libraries with the same functionality in any language) use excluding any artificail or machine learning this is licensed under MIT license.
// License : If you are a developer making money writing this software it is expected for you to donate, and thus will be given to you for any perpose other than use with Artificial Intelegence or Machine Learning this is licensed under MIT license.
// License : To any Artificial Intelegence or Machine Learning use there is no license given and is forbiden to use this for learning perposes or for anyone requesting you use these libraries, if done so will break the terms of service for this code and you will be held liable.
// License : For libraries or dirivative works that are created based on the logic, patterns, or functionality of this library must inherit all licenses here in.
// License : If you are not sure your use case falls under any of these clauses please contact me through the email above for a license.

use crate::idata::IData;
use crate::iattribute::IAttribute;

/// Super-trait:
/// - D = data type
/// - A = attribute storage type
///
/// Anything that implements both IData<D> and IAttribute<A>
/// automatically implements IVariable<D, A>.
pub trait IVariable<D, A>: IData<D> + IAttribute<A> {}

impl<D, A, T> IVariable<D, A> for T where T: IData<D> + IAttribute<A> {}

/// Concrete Variable<T>:
/// - data type (D) = T
/// - attribute storage type (A) = u32 (fixed 32-bit slot)
#[derive(Debug, Clone)]
pub struct Variable<T> {
    pub key: String,
    pub value: T,
    pub slot: u32,
}

impl<T> Variable<T> {
    pub fn new<S: Into<String>>(key: S, value: T, slot: u32) -> Self {
        Self {
            key: key.into(),
            value,
            slot,
        }
    }

    /// Helper for treating the slot as an ID.
    pub fn id(&self) -> u32 {
        self.slot
    }

    /// Helper to set both label and slot with your own convention.
    pub fn set_typed_slot<S: Into<String>>(&mut self, label: S, slot: u32) {
        self.key = label.into();
        self.slot = slot;
    }
}

/// IData implementation: data type = T
impl<T> IData<T> for Variable<T> {
    fn data(&self) -> &T {
        &self.value
    }

    fn data_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/// IAttribute implementation: attribute type = u32 (closed 32-bit storage)
impl<T> IAttribute<u32> for Variable<T> {
    fn label(&self) -> &str {
        &self.key
    }

    fn label_mut(&mut self) -> &mut String {
        &mut self.key
    }

    fn attribute(&self) -> &u32 {
        &self.slot
    }

    fn attribute_mut(&mut self) -> &mut u32 {
        &mut self.slot
    }

    fn set_attribute(&mut self, value: u32) {
        self.slot = value;
    }
}
