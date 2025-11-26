// variable_util.rs
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

/// Simple type-tag enum for the u32 attribute slot.
/// Using repr(u32) so it can be stored directly in the 32-bit slot.
#[repr(u32)]
pub enum ValueKind {
    String = 1,
    Int = 2,
    Pointer = 3,
}

impl ValueKind {
    pub fn from_u32(raw: u32) -> Option<Self> {
        match raw {
            1 => Some(ValueKind::String),
            2 => Some(ValueKind::Int),
            3 => Some(ValueKind::Pointer),
            _ => None,
        }
    }
}

/// Helper to log/debug the “type” of a variable based on:
/// - its label string
/// - its u32 attribute slot
///
/// Any type implementing IData<D> + IAttribute<u32>
/// (i.e. IVariable<D, u32>) can use this.
pub fn debug_variable_type<D, V>(v: &V)
where
    V: IData<D> + IAttribute<u32>,
{
    let label = v.label();
    let tag = *v.attribute();

    let kind = ValueKind::from_u32(tag);
    match kind {
        Some(ValueKind::String) => {
            println!("[TYPE] {label} => String (tag={tag})");
        }
        Some(ValueKind::Int) => {
            println!("[TYPE] {label} => Int (tag={tag})");
        }
        Some(ValueKind::Pointer) => {
            println!("[TYPE] {label} => Pointer/Index (tag={tag})");
        }
        None => {
            println!("[TYPE] {label} => Unknown (raw tag={tag})");
        }
    }
}
