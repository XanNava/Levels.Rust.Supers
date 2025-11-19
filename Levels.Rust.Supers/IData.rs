// IData.rs

/// IData<T> in C# exposes a get/set property `_Data`,
/// plus protected helpers `SetData` / `GetData`.
/// In Rust, traits can't have fields, so we model this as
/// required accessors plus default helper methods.
pub trait IData<T> {
    /// Immutable access to the underlying value (like `get`).
    fn data(&self) -> &T;

    /// Mutable access to the underlying value (needed for `set`).
    fn data_mut(&mut self) -> &mut T;

    /// Mirrors `SetData(T value)` default impl in the C# interface.
    fn set_data(&mut self, value: T) {
        *self.data_mut() = value;
    }

    /// Mirrors `GetData()` default impl returning `_Data`.
    fn get_data(&self) -> &T {
        self.data()
    }
}

/// IDataRef<T> in C# inherits IData<T>, exposes a `ref T _DataRef`
/// and explicitly maps `IData<T>._Data` to that ref.
/// In Rust, we express this as a super-trait plus a blanket impl
/// that forwards IData<T> to the ref-based accessors here.
pub trait IDataRef<T> {
    /// Immutable reference to the backing `_DataRef`.
    fn data_ref(&self) -> &T;

    /// Mutable reference to the backing `_DataRef`.
    fn data_ref_mut(&mut self) -> &mut T;

    /// Convenience helpers analogous to the C# protected ones.
    fn set_data_ref(&mut self, value: T) {
        *self.data_ref_mut() = value;
    }

    fn get_data_ref(&self) -> &T {
        self.data_ref()
    }
}

/// Blanket implementation:
/// Any type that implements `IDataRef<T>` automatically
/// satisfies `IData<T>` by forwarding through its ref accessors.
/// This mirrors the explicit interface forwarding in your C#:
/// `T IData<T>._Data { get => _DataRef; set => _DataRef = value; }`
impl<T, I> IData<T> for I
where
    I: IDataRef<T>,
{
    fn data(&self) -> &T {
        self.data_ref()
    }

    fn data_mut(&mut self) -> &mut T {
        self.data_ref_mut()
    }
}

/// C# extension method:
/// `IData_Extends.Interface_IData<T, I>(this I self) => self;`
/// In Rust we provide a tiny extension trait; returning `&mut Self`
/// is the closest practical equivalent (returning a trait object is
/// possible but rarely useful here).
pub trait IDataExt<T>: IData<T> {
    fn interface_IData(&mut self) -> &mut Self {
        self
    }
}

impl<T, I> IDataExt<T> for I where I: IData<T> {}


// ---------------------------------------------------------------------
// Example implementors (optional, just to illustrate usage):
// ---------------------------------------------------------------------

/// Stores the value directly. Implements `IDataRef<T>` by returning
/// references to its field; therefore it also implements `IData<T>`
/// via the blanket impl above.
pub struct DataBox<T> {
    inner: T,
}

impl<T> DataBox<T> {
    pub fn new(value: T) -> Self {
        Self { inner: value }
    }
}

impl<T> IDataRef<T> for DataBox<T> {
    fn data_ref(&self) -> &T {
        &self.inner
    }

    fn data_ref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

// Usage:
// let mut b = DataBox::new(42);
// b.set_data_ref(100);
// assert_eq!(*b.get_data(), 100);
// b.interface_IOata(); // no-op "extension" method