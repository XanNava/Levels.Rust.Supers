// Author  : Alexander Nava 
// Contact : Alexander.Nava.Contact@Gmail.com
// License : For personal use excluding any artificail or machine learning this is licensed under MIT license.
// License : For commercial software(excluding derivative work to make libraries with the same functionality in any language) use excluding any artificail or machine learning this is licensed under MIT license.
// License : If you are a developer making money writing this software it is expected for you to donate, and thus will be given to you for any perpose other than use with Artificial Intelegence or Machine Learning this is licensed under MIT license.
// License : To any Artificial Intelegence or Machine Learning use there is no license given and is forbiden to use this for learning perposes or for anyone requesting you use these libraries, if done so will break the terms of service for this code and you will be held liable.
// License : For libraries or dirivative works that are created based on the logic, patterns, or functionality of this library must inherit all licenses here in.
// License : If you are not sure your use case falls under any of these clauses please contact me through the email above for a license.

// IAttribute.rs

use std::collections::HashMap;

// Adjust this path to wherever IData / IDataRef live.
use crate::idata::{IData, IDataRef};

/// C#:
/// public interface IAttributable {
///     bool HasAttribute(string key);
/// }
pub trait IAttributable {
    fn has_attribute(&self, key: &str) -> bool;
}

/// C#:
/// public interface IAttribute<T> {
///     string Label { get; set; }
///     T Attribute { get; set; }
/// }
///
/// In Rust we model this as:
/// - super-trait of `IData<T>` for the value
/// - explicit accessors for the label (string key)
pub trait IAttribute<T>: IData<T> {
    /// Access the label / key for this attribute.
    fn label(&self) -> &str;

    /// Mutable access to the label.
    fn label_mut(&mut self) -> &mut String;

    /// Helper to set the label.
    fn set_label<S: Into<String>>(&mut self, label: S) {
        *self.label_mut() = label.into();
    }

    /// Mirror C# `T Attribute { get; set; }` using IData helpers.
    fn attribute(&self) -> &T {
        self.get_data()
    }

    fn attribute_mut(&mut self) -> &mut T {
        self.data_mut()
    }

    fn set_attribute(&mut self, value: T) {
        self.set_data(value);
    }
}

/// C#:
/// public interface IAttributes<T> : IAttributable {
///     Dictionary<string, T> Collection { get; }
///
///     bool HasAttribute(string key) {
///         return Collection.ContainsKey(key);
///     }
///
///     T TryGetAttribute(string key) {
///         Debug.Log(Collection);
///         Debug.Log(!Collection.ContainsKey(key));
///
///         if (!Collection.ContainsKey(key))
///             return default;
///
///         return Collection[key];
///     }
/// }
///
/// In Rust we use `HashMap<String, T>` and return `Option<&T>`
/// instead of `default(T)` when a key is missing.
pub trait IAttributes<T>: IAttributable {
    /// Read-only access to the attribute collection.
    fn collection(&self) -> &HashMap<String, T>;

    /// Mutable access to the attribute collection.
    fn collection_mut(&mut self) -> &mut HashMap<String, T>;

    /// Default impl mirrors `HasAttribute`.
    fn has_attribute(&self, key: &str) -> bool {
        self.collection().contains_key(key)
    }

    /// Default impl mirrors `TryGetAttribute`, but returns `Option<&T>`.
    fn try_get_attribute(&self, key: &str) -> Option<&T> {
        // If you want logs similar to Unity's Debug.Log, you can
        // uncomment these lines or plug in your own logger:
        //
        // println!("Attributes: {:?}", self.collection());
        // println!("Has key \"{}\": {}", key, self.collection().contains_key(key));

        self.collection().get(key)
    }
}

/// C# extension method:
/// public static IAttributes<T> Interface_IAttributes<T>(this IAttributes<T> source) => source;
///
/// In Rust we follow the same pattern you used for IDataExt.
pub trait IAttributesExt<T>: IAttributes<T> {
    fn interface_IAttributes(&mut self) -> &mut Self {
        self
    }
}

impl<T, A> IAttributesExt<T> for A where A: IAttributes<T> {}

/// Marker traits that specialize `IAttributes<T>` for common types.
///
/// C#:
/// public interface IAttributesBool   : IAttributes<bool> { }
/// public interface IAttributesFloat  : IAttributes<float> { }
/// public interface IAttributesInt    : IAttributes<int> { }
/// public interface IAttributesVector2 : IAttributes<Vector2> { }
/// public interface IAttributesVector3 : IAttributes<Vector3> { }
/// public interface IAttributesColor   : IAttributes<Color> { }
///
/// For Vector2/Vector3/Color, you can hook these up to
/// whatever math/color types you use in your Rust engine.
pub trait IAttributesBool: IAttributes<bool> {}
impl<A> IAttributesBool for A where A: IAttributes<bool> {}

pub trait IAttributesFloat: IAttributes<f32> {}
impl<A> IAttributesFloat for A where A: IAttributes<f32> {}

pub trait IAttributesInt: IAttributes<i32> {}
impl<A> IAttributesInt for A where A: IAttributes<i32> {}

// Assuming you have these types somewhere:
//
// use crate::math::{Vector2, Vector3, Color};
//
// Uncomment and adapt these when those types exist.
/*
pub trait IAttributesVector2: IAttributes<Vector2> {}
impl<A> IAttributesVector2 for A where A: IAttributes<Vector2> {}

pub trait IAttributesVector3: IAttributes<Vector3> {}
impl<A> IAttributesVector3 for A where A: IAttributes<Vector3> {}

pub trait IAttributesColor: IAttributes<Color> {}
impl<A> IAttributesColor for A where A: IAttributes<Color> {}
*/

/// C#:
/// public interface IAttributesFunc<T> : IAttributes<Func<T>> { }
/// public interface IAttributesFunc<T, V> : IAttributes<Func<T, V>> { }
/// public interface IAttributesAction<T> : IAttributes<Action<T>> { }
///
/// In Rust we approximate this with boxed function traits.
/// Adjust 'static / boxing as needed for your engine.
pub trait IAttributesFunc<T>: IAttributes<Box<dyn Fn() -> T + 'static>> {}
impl<T, A> IAttributesFunc<T> for A where A: IAttributes<Box<dyn Fn() -> T + 'static>> {}

pub trait IAttributesFunc2<T, V>: IAttributes<Box<dyn Fn(T) -> V + 'static>> {}
impl<T, V, A> IAttributesFunc2<T, V> for A where A: IAttributes<Box<dyn Fn(T) -> V + 'static>> {}

pub trait IAttributesAction<T>: IAttributes<Box<dyn Fn(T) + 'static>> {}
impl<T, A> IAttributesAction<T> for A where A: IAttributes<Box<dyn Fn(T) + 'static>> {}

/// C#:
/// [Serializable]
/// public struct Attribute<T> {
///     public string key;
///     public T value;
/// }
///
/// In Rust:
/// - `key` corresponds to `Label` / `label()`
/// - `value` is the data; we plug it into `IDataRef<T>` so the
///   existing `IData<T>` super-trait blanket impl works.
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

/// Wire `Attribute<T>` into the IData super-trait stack by making
/// it an `IDataRef<T>` adapter over its `value` field.
impl<T> IDataRef<T> for Attribute<T> {
    fn data_ref(&self) -> &T {
        &self.value
    }

    fn data_ref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/// Now `Attribute<T>` is a full `IAttribute<T>`:
impl<T> IAttribute<T> for Attribute<T> {
    fn label(&self) -> &str {
        &self.key
    }

    fn label_mut(&mut self) -> &mut String {
        &mut self.key
    }
}
