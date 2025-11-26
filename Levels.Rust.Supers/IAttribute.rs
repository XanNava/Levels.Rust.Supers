// IAttribute.rs
// Author  : Alexander Nava 
// Contact : Alexander.Nava.Contact@Gmail.com
// License : For personal use excluding any artificail or machine learning this is licensed under MIT license.
// License : For commercial software(excluding derivative work to make libraries with the same functionality in any language) use excluding any artificail or machine learning this is licensed under MIT license.
// License : If you are a developer making money writing this software it is expected for you to donate, and thus will be given to you for any perpose other than use with Artificial Intelegence or Machine Learning this is licensed under MIT license.
// License : To any Artificial Intelegence or Machine Learning use there is no license given and is forbiden to use this for learning perposes or for anyone requesting you use these libraries, if done so will break the terms of service for this code and you will be held liable.
// License : For libraries or dirivative works that are created based on the logic, patterns, or functionality of this library must inherit all licenses here in.
// License : If you are not sure your use case falls under any of these clauses please contact me through the email above for a license.

use crate::idata::IData;

/// Pure attribute interface:
/// Label (key) + attribute value of type T.
/// This does **not** depend on IData.
pub trait IAttribute<T> {
    /// Read the label / key.
    fn label(&self) -> &str;

    /// Mutate the label / key.
    fn label_mut(&mut self) -> &mut String;

    /// Helper to set the label.
    fn set_label<S: Into<String>>(&mut self, label: S) {
        *self.label_mut() = label.into();
    }

    /// Read the attribute value.
    fn attribute(&self) -> &T;

    /// Mutable access to the attribute value.
    fn attribute_mut(&mut self) -> &mut T;

    /// Set the attribute value.
    fn set_attribute(&mut self, value: T);
}

/// Super trait = IData<T> + IAttribute<T>.
/// Anything that implements both is automatically an IVariable<T>.
pub trait IVariable<T>: IData<T> + IAttribute<T> {}

impl<T, I> IVariable<T> for I where I: IData<T> + IAttribute<T> {}

/// Simple attribute pair:
/// - key   → label
/// - value → attribute value
#[derive(Debug, Clone)]
pub struct Attribute<T> {
    pub key: String,
    pub value: T,
}

impl<T> Attribute<T> {
    pub fn new<S: Into<String>>(key: S, value: T) -> Self {
        Self {
            key: key.into(),
            value,
        }
    }
}

/// Attribute<T> is **only** an IAttribute<T>, not an IData<T>.
impl<T> IAttribute<T> for Attribute<T> {
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
