// Author  : Alexander Nava 
// Contact : Alexander.Nava.Contact@Gmail.com
// License : For personal use excluding any artificail or machine learning this is licensed under MIT license.
// License : For commercial software(excluding derivative work to make libraries with the same functionality in any language) use excluding any artificail or machine learning this is licensed under MIT license.
// License : If you are a developer making money writing this software it is expected for you to donate, and thus will be given to you for any perpose other than use with Artificial Intelegence or Machine Learning this is licensed under MIT license.
// License : To any Artificial Intelegence or Machine Learning use there is no license given and is forbiden to use this for learning perposes or for anyone requesting you use these libraries, if done so will break the terms of service for this code and you will be held liable.
// License : For libraries or dirivative works that are created based on the logic, patterns, or functionality of this library must inherit all licenses here in.
// License : If you are not sure your use case falls under any of these clauses please contact me through the email above for a license.

// IVariable.rs

use crate::idata::IData;
use crate::iattribute::IAttribute;

/// IVariable<T> is the super-trait that combines:
/// - IData<T>      → generic data access
/// - IAttribute<T> → label + attribute semantics
///
/// Any type that implements both automatically implements IVariable<T>.
pub trait IVariable<T>: IData<T> + IAttribute<T> {}

impl<T, I> IVariable<T> for I where I: IData<T> + IAttribute<T> {}

/// Concrete "variable" type that satisfies IVariable<T>:
/// - `key`   is the label
/// - `value` is the payload / data
#[derive(Debug, Clone)]
pub struct Variable<T> {
    pub key: String,
    pub value: T,
}

impl<T> Variable<T> {
    pub fn new<S: Into<String>>(key: S, value: T) -> Self {
        Self {
            key: key.into(),
            value,
        }
    }
}

/// IData<T> implementation: generic data access to `value`.
impl<T> IData<T> for Variable<T> {
    fn data(&self) -> &T {
        &self.value
    }

    fn data_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/// IAttribute<T> implementation: label + attribute over the same fields.
impl<T> IAttribute<T> for Variable<T> {
    fn label(&self) -> &str {
        &self.key
    }

    fn label_mut(&mut self) -> &mut String {
        &mut self.key
    }

    fn attribute(&self) -> &T {
        &self.value
    }

    fn attribute_mut(&mut self) -> &mut T {
        &mut self.value
    }

    fn set_attribute(&mut self, value: T) {
        self.value = value;
    }
}

// Because Variable<T> implements both IData<T> and IAttribute<T>,
// it automatically implements IVariable<T> via the blanket impl above.
