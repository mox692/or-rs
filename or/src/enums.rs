//! A concrete implementation of the type Or that represents values of multiple types.
//!
//! Different enum types `OrX` (where X is the number of types the enum can contain) are provided
//! depending on the number of types it can contain.
//!
//! The implementation of these enums includes several basic function such as `is_tx`
//! for assertion and `as_tx` for cast, and also have some util functions, like `map`, `fold`.

use std::any::TypeId;

/// `Or2` is an enum representing a value that can be either of 2 types, T1 ... T2.
pub enum Or2<T1, T2> {
    T1(T1),
    T2(T2),
}

impl<T1, T2> Or2<T1, T2> {
    /// Returns true if the enum is of type T1.
    pub fn is_t1(&self) -> bool {
        match self {
            Self::T1(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T2.
    pub fn is_t2(&self) -> bool {
        match self {
            Self::T2(_) => true,
            _ => false,
        }
    }

    /// Converts the enum to an Option containing the T1 value, if it is of type T1.
    pub fn as_t1(self) -> Option<T1> {
        match self {
            Self::T1(t1) => Some(t1),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T2 value, if it is of type T2.
    pub fn as_t2(self) -> Option<T2> {
        match self {
            Self::T2(t2) => Some(t2),
            _ => None,
        }
    }

    /// Transforms the T1 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t1<F, B>(self, f: F) -> Or2<B, T2>
    where
        F: FnOnce(T1) -> B,
    {
        match self {
            Self::T1(t1) => Or2::<B, T2>::T1(f(t1)),
            Self::T2(t2) => Or2::<B, T2>::T2(t2),
        }
    }

    /// Transforms the T2 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t2<F, B>(self, f: F) -> Or2<T1, B>
    where
        F: FnOnce(T2) -> B,
    {
        match self {
            Self::T1(t1) => Or2::<T1, B>::T1(t1),
            Self::T2(t2) => Or2::<T1, B>::T2(f(t2)),
        }
    }

    /// Consolidates the `Or2` enum into a single value of type `T`,
    /// by applying provided functions.
    pub fn fold<T, F0, F1, F2>(self, f1: F1, f2: F2) -> T
    where
        F1: FnOnce(T1) -> T,
        F2: FnOnce(T2) -> T,
    {
        match self {
            Self::T1(t1) => f1(t1),
            Self::T2(t2) => f2(t2),
        }
    }
}

/// Extension to `Or2` to check if the enum's type matches a arbitrary type.
/// Currently, these functions depend on the rustc intrinsics, and the constraints
/// of the intrinsics require that the type must satisfy `'static'`.
impl<T1, T2> Or2<T1, T2>
where
    T1: 'static,
    T2: 'static,
{
    pub fn is_type<T: 'static>(&self) -> bool {
        match self {
            Self::T1(_) => TypeId::of::<T>() == TypeId::of::<T1>(),
            Self::T2(_) => TypeId::of::<T>() == TypeId::of::<T2>(),
        }
    }
}

/// `Or3` is an enum representing a value that can be either of 3 types, T1 ... T3.
#[derive(Clone, Copy)]
pub enum Or3<T1, T2, T3> {
    T1(T1),
    T2(T2),
    T3(T3),
}

impl<T1, T2, T3> Or3<T1, T2, T3> {
    /// Returns true if the enum is of type T1.
    pub fn is_t1(&self) -> bool {
        match self {
            Self::T1(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T2.
    pub fn is_t2(&self) -> bool {
        match self {
            Self::T2(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T3.
    pub fn is_t3(&self) -> bool {
        match self {
            Self::T3(_) => true,
            _ => false,
        }
    }

    /// Converts the enum to an Option containing the T1 value, if it is of type T1.
    pub fn as_t1(self) -> Option<T1> {
        match self {
            Self::T1(t1) => Some(t1),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T2 value, if it is of type T2.
    pub fn as_t2(self) -> Option<T2> {
        match self {
            Self::T2(t2) => Some(t2),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T3 value, if it is of type T3.
    pub fn as_t3(self) -> Option<T3> {
        match self {
            Self::T3(t3) => Some(t3),
            _ => None,
        }
    }

    /// Transforms the T1 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t1<F, B>(self, f: F) -> Or3<B, T2, T3>
    where
        F: FnOnce(T1) -> B,
    {
        match self {
            Self::T1(t1) => Or3::<B, T2, T3>::T1(f(t1)),
            Self::T2(t2) => Or3::<B, T2, T3>::T2(t2),
            Self::T3(t3) => Or3::<B, T2, T3>::T3(t3),
        }
    }

    /// Transforms the T2 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t2<F, B>(self, f: F) -> Or3<T1, B, T3>
    where
        F: FnOnce(T2) -> B,
    {
        match self {
            Self::T1(t1) => Or3::<T1, B, T3>::T1(t1),
            Self::T2(t2) => Or3::<T1, B, T3>::T2(f(t2)),
            Self::T3(t3) => Or3::<T1, B, T3>::T3(t3),
        }
    }

    /// Transforms the T3 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t3<F, B>(self, f: F) -> Or3<T1, T2, B>
    where
        F: FnOnce(T3) -> B,
    {
        match self {
            Self::T1(t1) => Or3::<T1, T2, B>::T1(t1),
            Self::T2(t2) => Or3::<T1, T2, B>::T2(t2),
            Self::T3(t3) => Or3::<T1, T2, B>::T3(f(t3)),
        }
    }

    /// Consolidates the `Or3` enum into a single value of type `T`,
    /// by applying provided functions.
    pub fn fold<T, F0, F1, F2, F3>(self, f1: F1, f2: F2, f3: F3) -> T
    where
        F1: FnOnce(T1) -> T,
        F2: FnOnce(T2) -> T,
        F3: FnOnce(T3) -> T,
    {
        match self {
            Self::T1(t1) => f1(t1),
            Self::T2(t2) => f2(t2),
            Self::T3(t3) => f3(t3),
        }
    }
}

/// Extension to `Or3` to check if the enum's type matches a arbitrary type.
/// Currently, these functions depend on the rustc intrinsics, and the constraints
/// of the intrinsics require that the type must satisfy `'static'`.
impl<T1, T2, T3> Or3<T1, T2, T3>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
{
    pub fn is_type<T: 'static>(&self) -> bool {
        match self {
            Self::T1(_) => TypeId::of::<T>() == TypeId::of::<T1>(),
            Self::T2(_) => TypeId::of::<T>() == TypeId::of::<T2>(),
            Self::T3(_) => TypeId::of::<T>() == TypeId::of::<T3>(),
        }
    }
}

/// `Or4` is an enum representing a value that can be either of 4 types, T1 ... T4.
pub enum Or4<T1, T2, T3, T4> {
    T1(T1),
    T2(T2),
    T3(T3),
    T4(T4),
}

impl<T1, T2, T3, T4> Or4<T1, T2, T3, T4> {
    /// Returns true if the enum is of type T1.
    pub fn is_t1(&self) -> bool {
        match self {
            Self::T1(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T2.
    pub fn is_t2(&self) -> bool {
        match self {
            Self::T2(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T3.
    pub fn is_t3(&self) -> bool {
        match self {
            Self::T3(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T4.
    pub fn is_t4(&self) -> bool {
        match self {
            Self::T4(_) => true,
            _ => false,
        }
    }

    /// Converts the enum to an Option containing the T1 value, if it is of type T1.
    pub fn as_t1(self) -> Option<T1> {
        match self {
            Self::T1(t1) => Some(t1),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T2 value, if it is of type T2.
    pub fn as_t2(self) -> Option<T2> {
        match self {
            Self::T2(t2) => Some(t2),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T3 value, if it is of type T3.
    pub fn as_t3(self) -> Option<T3> {
        match self {
            Self::T3(t3) => Some(t3),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T4 value, if it is of type T4.
    pub fn as_t4(self) -> Option<T4> {
        match self {
            Self::T4(t4) => Some(t4),
            _ => None,
        }
    }

    /// Transforms the T1 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t1<F, B>(self, f: F) -> Or4<B, T2, T3, T4>
    where
        F: FnOnce(T1) -> B,
    {
        match self {
            Self::T1(t1) => Or4::<B, T2, T3, T4>::T1(f(t1)),
            Self::T2(t2) => Or4::<B, T2, T3, T4>::T2(t2),
            Self::T3(t3) => Or4::<B, T2, T3, T4>::T3(t3),
            Self::T4(t4) => Or4::<B, T2, T3, T4>::T4(t4),
        }
    }

    /// Transforms the T2 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t2<F, B>(self, f: F) -> Or4<T1, B, T3, T4>
    where
        F: FnOnce(T2) -> B,
    {
        match self {
            Self::T1(t1) => Or4::<T1, B, T3, T4>::T1(t1),
            Self::T2(t2) => Or4::<T1, B, T3, T4>::T2(f(t2)),
            Self::T3(t3) => Or4::<T1, B, T3, T4>::T3(t3),
            Self::T4(t4) => Or4::<T1, B, T3, T4>::T4(t4),
        }
    }

    /// Transforms the T3 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t3<F, B>(self, f: F) -> Or4<T1, T2, B, T4>
    where
        F: FnOnce(T3) -> B,
    {
        match self {
            Self::T1(t1) => Or4::<T1, T2, B, T4>::T1(t1),
            Self::T2(t2) => Or4::<T1, T2, B, T4>::T2(t2),
            Self::T3(t3) => Or4::<T1, T2, B, T4>::T3(f(t3)),
            Self::T4(t4) => Or4::<T1, T2, B, T4>::T4(t4),
        }
    }

    /// Transforms the T4 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t4<F, B>(self, f: F) -> Or4<T1, T2, T3, B>
    where
        F: FnOnce(T4) -> B,
    {
        match self {
            Self::T1(t1) => Or4::<T1, T2, T3, B>::T1(t1),
            Self::T2(t2) => Or4::<T1, T2, T3, B>::T2(t2),
            Self::T3(t3) => Or4::<T1, T2, T3, B>::T3(t3),
            Self::T4(t4) => Or4::<T1, T2, T3, B>::T4(f(t4)),
        }
    }

    /// Consolidates the `Or4` enum into a single value of type `T`,
    /// by applying provided functions.
    pub fn fold<T, F0, F1, F2, F3, F4>(self, f1: F1, f2: F2, f3: F3, f4: F4) -> T
    where
        F1: FnOnce(T1) -> T,
        F2: FnOnce(T2) -> T,
        F3: FnOnce(T3) -> T,
        F4: FnOnce(T4) -> T,
    {
        match self {
            Self::T1(t1) => f1(t1),
            Self::T2(t2) => f2(t2),
            Self::T3(t3) => f3(t3),
            Self::T4(t4) => f4(t4),
        }
    }
}

/// Extension to `Or4` to check if the enum's type matches a arbitrary type.
/// Currently, these functions depend on the rustc intrinsics, and the constraints
/// of the intrinsics require that the type must satisfy `'static'`.
impl<T1, T2, T3, T4> Or4<T1, T2, T3, T4>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
{
    pub fn is_type<T: 'static>(&self) -> bool {
        match self {
            Self::T1(_) => TypeId::of::<T>() == TypeId::of::<T1>(),
            Self::T2(_) => TypeId::of::<T>() == TypeId::of::<T2>(),
            Self::T3(_) => TypeId::of::<T>() == TypeId::of::<T3>(),
            Self::T4(_) => TypeId::of::<T>() == TypeId::of::<T4>(),
        }
    }
}

/// `Or5` is an enum representing a value that can be either of 5 types, T1 ... T5.
pub enum Or5<T1, T2, T3, T4, T5> {
    T1(T1),
    T2(T2),
    T3(T3),
    T4(T4),
    T5(T5),
}

impl<T1, T2, T3, T4, T5> Or5<T1, T2, T3, T4, T5> {
    /// Returns true if the enum is of type T1.
    pub fn is_t1(&self) -> bool {
        match self {
            Self::T1(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T2.
    pub fn is_t2(&self) -> bool {
        match self {
            Self::T2(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T3.
    pub fn is_t3(&self) -> bool {
        match self {
            Self::T3(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T4.
    pub fn is_t4(&self) -> bool {
        match self {
            Self::T4(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T5.
    pub fn is_t5(&self) -> bool {
        match self {
            Self::T5(_) => true,
            _ => false,
        }
    }

    /// Converts the enum to an Option containing the T1 value, if it is of type T1.
    pub fn as_t1(self) -> Option<T1> {
        match self {
            Self::T1(t1) => Some(t1),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T2 value, if it is of type T2.
    pub fn as_t2(self) -> Option<T2> {
        match self {
            Self::T2(t2) => Some(t2),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T3 value, if it is of type T3.
    pub fn as_t3(self) -> Option<T3> {
        match self {
            Self::T3(t3) => Some(t3),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T4 value, if it is of type T4.
    pub fn as_t4(self) -> Option<T4> {
        match self {
            Self::T4(t4) => Some(t4),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T5 value, if it is of type T5.
    pub fn as_t5(self) -> Option<T5> {
        match self {
            Self::T5(t5) => Some(t5),
            _ => None,
        }
    }

    /// Transforms the T1 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t1<F, B>(self, f: F) -> Or5<B, T2, T3, T4, T5>
    where
        F: FnOnce(T1) -> B,
    {
        match self {
            Self::T1(t1) => Or5::<B, T2, T3, T4, T5>::T1(f(t1)),
            Self::T2(t2) => Or5::<B, T2, T3, T4, T5>::T2(t2),
            Self::T3(t3) => Or5::<B, T2, T3, T4, T5>::T3(t3),
            Self::T4(t4) => Or5::<B, T2, T3, T4, T5>::T4(t4),
            Self::T5(t5) => Or5::<B, T2, T3, T4, T5>::T5(t5),
        }
    }

    /// Transforms the T2 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t2<F, B>(self, f: F) -> Or5<T1, B, T3, T4, T5>
    where
        F: FnOnce(T2) -> B,
    {
        match self {
            Self::T1(t1) => Or5::<T1, B, T3, T4, T5>::T1(t1),
            Self::T2(t2) => Or5::<T1, B, T3, T4, T5>::T2(f(t2)),
            Self::T3(t3) => Or5::<T1, B, T3, T4, T5>::T3(t3),
            Self::T4(t4) => Or5::<T1, B, T3, T4, T5>::T4(t4),
            Self::T5(t5) => Or5::<T1, B, T3, T4, T5>::T5(t5),
        }
    }

    /// Transforms the T3 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t3<F, B>(self, f: F) -> Or5<T1, T2, B, T4, T5>
    where
        F: FnOnce(T3) -> B,
    {
        match self {
            Self::T1(t1) => Or5::<T1, T2, B, T4, T5>::T1(t1),
            Self::T2(t2) => Or5::<T1, T2, B, T4, T5>::T2(t2),
            Self::T3(t3) => Or5::<T1, T2, B, T4, T5>::T3(f(t3)),
            Self::T4(t4) => Or5::<T1, T2, B, T4, T5>::T4(t4),
            Self::T5(t5) => Or5::<T1, T2, B, T4, T5>::T5(t5),
        }
    }

    /// Transforms the T4 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t4<F, B>(self, f: F) -> Or5<T1, T2, T3, B, T5>
    where
        F: FnOnce(T4) -> B,
    {
        match self {
            Self::T1(t1) => Or5::<T1, T2, T3, B, T5>::T1(t1),
            Self::T2(t2) => Or5::<T1, T2, T3, B, T5>::T2(t2),
            Self::T3(t3) => Or5::<T1, T2, T3, B, T5>::T3(t3),
            Self::T4(t4) => Or5::<T1, T2, T3, B, T5>::T4(f(t4)),
            Self::T5(t5) => Or5::<T1, T2, T3, B, T5>::T5(t5),
        }
    }

    /// Transforms the T5 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t5<F, B>(self, f: F) -> Or5<T1, T2, T3, T4, B>
    where
        F: FnOnce(T5) -> B,
    {
        match self {
            Self::T1(t1) => Or5::<T1, T2, T3, T4, B>::T1(t1),
            Self::T2(t2) => Or5::<T1, T2, T3, T4, B>::T2(t2),
            Self::T3(t3) => Or5::<T1, T2, T3, T4, B>::T3(t3),
            Self::T4(t4) => Or5::<T1, T2, T3, T4, B>::T4(t4),
            Self::T5(t5) => Or5::<T1, T2, T3, T4, B>::T5(f(t5)),
        }
    }

    /// Consolidates the `Or5` enum into a single value of type `T`,
    /// by applying provided functions.
    pub fn fold<T, F0, F1, F2, F3, F4, F5>(self, f1: F1, f2: F2, f3: F3, f4: F4, f5: F5) -> T
    where
        F1: FnOnce(T1) -> T,
        F2: FnOnce(T2) -> T,
        F3: FnOnce(T3) -> T,
        F4: FnOnce(T4) -> T,
        F5: FnOnce(T5) -> T,
    {
        match self {
            Self::T1(t1) => f1(t1),
            Self::T2(t2) => f2(t2),
            Self::T3(t3) => f3(t3),
            Self::T4(t4) => f4(t4),
            Self::T5(t5) => f5(t5),
        }
    }
}

/// Extension to `Or5` to check if the enum's type matches a arbitrary type.
/// Currently, these functions depend on the rustc intrinsics, and the constraints
/// of the intrinsics require that the type must satisfy `'static'`.
impl<T1, T2, T3, T4, T5> Or5<T1, T2, T3, T4, T5>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
{
    pub fn is_type<T: 'static>(&self) -> bool {
        match self {
            Self::T1(_) => TypeId::of::<T>() == TypeId::of::<T1>(),
            Self::T2(_) => TypeId::of::<T>() == TypeId::of::<T2>(),
            Self::T3(_) => TypeId::of::<T>() == TypeId::of::<T3>(),
            Self::T4(_) => TypeId::of::<T>() == TypeId::of::<T4>(),
            Self::T5(_) => TypeId::of::<T>() == TypeId::of::<T5>(),
        }
    }
}

/// `Or6` is an enum representing a value that can be either of 6 types, T1 ... T6.
pub enum Or6<T1, T2, T3, T4, T5, T6> {
    T1(T1),
    T2(T2),
    T3(T3),
    T4(T4),
    T5(T5),
    T6(T6),
}

impl<T1, T2, T3, T4, T5, T6> Or6<T1, T2, T3, T4, T5, T6> {
    /// Returns true if the enum is of type T1.
    pub fn is_t1(&self) -> bool {
        match self {
            Self::T1(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T2.
    pub fn is_t2(&self) -> bool {
        match self {
            Self::T2(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T3.
    pub fn is_t3(&self) -> bool {
        match self {
            Self::T3(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T4.
    pub fn is_t4(&self) -> bool {
        match self {
            Self::T4(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T5.
    pub fn is_t5(&self) -> bool {
        match self {
            Self::T5(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T6.
    pub fn is_t6(&self) -> bool {
        match self {
            Self::T6(_) => true,
            _ => false,
        }
    }

    /// Converts the enum to an Option containing the T1 value, if it is of type T1.
    pub fn as_t1(self) -> Option<T1> {
        match self {
            Self::T1(t1) => Some(t1),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T2 value, if it is of type T2.
    pub fn as_t2(self) -> Option<T2> {
        match self {
            Self::T2(t2) => Some(t2),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T3 value, if it is of type T3.
    pub fn as_t3(self) -> Option<T3> {
        match self {
            Self::T3(t3) => Some(t3),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T4 value, if it is of type T4.
    pub fn as_t4(self) -> Option<T4> {
        match self {
            Self::T4(t4) => Some(t4),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T5 value, if it is of type T5.
    pub fn as_t5(self) -> Option<T5> {
        match self {
            Self::T5(t5) => Some(t5),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T6 value, if it is of type T6.
    pub fn as_t6(self) -> Option<T6> {
        match self {
            Self::T6(t6) => Some(t6),
            _ => None,
        }
    }

    /// Transforms the T1 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t1<F, B>(self, f: F) -> Or6<B, T2, T3, T4, T5, T6>
    where
        F: FnOnce(T1) -> B,
    {
        match self {
            Self::T1(t1) => Or6::<B, T2, T3, T4, T5, T6>::T1(f(t1)),
            Self::T2(t2) => Or6::<B, T2, T3, T4, T5, T6>::T2(t2),
            Self::T3(t3) => Or6::<B, T2, T3, T4, T5, T6>::T3(t3),
            Self::T4(t4) => Or6::<B, T2, T3, T4, T5, T6>::T4(t4),
            Self::T5(t5) => Or6::<B, T2, T3, T4, T5, T6>::T5(t5),
            Self::T6(t6) => Or6::<B, T2, T3, T4, T5, T6>::T6(t6),
        }
    }

    /// Transforms the T2 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t2<F, B>(self, f: F) -> Or6<T1, B, T3, T4, T5, T6>
    where
        F: FnOnce(T2) -> B,
    {
        match self {
            Self::T1(t1) => Or6::<T1, B, T3, T4, T5, T6>::T1(t1),
            Self::T2(t2) => Or6::<T1, B, T3, T4, T5, T6>::T2(f(t2)),
            Self::T3(t3) => Or6::<T1, B, T3, T4, T5, T6>::T3(t3),
            Self::T4(t4) => Or6::<T1, B, T3, T4, T5, T6>::T4(t4),
            Self::T5(t5) => Or6::<T1, B, T3, T4, T5, T6>::T5(t5),
            Self::T6(t6) => Or6::<T1, B, T3, T4, T5, T6>::T6(t6),
        }
    }

    /// Transforms the T3 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t3<F, B>(self, f: F) -> Or6<T1, T2, B, T4, T5, T6>
    where
        F: FnOnce(T3) -> B,
    {
        match self {
            Self::T1(t1) => Or6::<T1, T2, B, T4, T5, T6>::T1(t1),
            Self::T2(t2) => Or6::<T1, T2, B, T4, T5, T6>::T2(t2),
            Self::T3(t3) => Or6::<T1, T2, B, T4, T5, T6>::T3(f(t3)),
            Self::T4(t4) => Or6::<T1, T2, B, T4, T5, T6>::T4(t4),
            Self::T5(t5) => Or6::<T1, T2, B, T4, T5, T6>::T5(t5),
            Self::T6(t6) => Or6::<T1, T2, B, T4, T5, T6>::T6(t6),
        }
    }

    /// Transforms the T4 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t4<F, B>(self, f: F) -> Or6<T1, T2, T3, B, T5, T6>
    where
        F: FnOnce(T4) -> B,
    {
        match self {
            Self::T1(t1) => Or6::<T1, T2, T3, B, T5, T6>::T1(t1),
            Self::T2(t2) => Or6::<T1, T2, T3, B, T5, T6>::T2(t2),
            Self::T3(t3) => Or6::<T1, T2, T3, B, T5, T6>::T3(t3),
            Self::T4(t4) => Or6::<T1, T2, T3, B, T5, T6>::T4(f(t4)),
            Self::T5(t5) => Or6::<T1, T2, T3, B, T5, T6>::T5(t5),
            Self::T6(t6) => Or6::<T1, T2, T3, B, T5, T6>::T6(t6),
        }
    }

    /// Transforms the T5 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t5<F, B>(self, f: F) -> Or6<T1, T2, T3, T4, B, T6>
    where
        F: FnOnce(T5) -> B,
    {
        match self {
            Self::T1(t1) => Or6::<T1, T2, T3, T4, B, T6>::T1(t1),
            Self::T2(t2) => Or6::<T1, T2, T3, T4, B, T6>::T2(t2),
            Self::T3(t3) => Or6::<T1, T2, T3, T4, B, T6>::T3(t3),
            Self::T4(t4) => Or6::<T1, T2, T3, T4, B, T6>::T4(t4),
            Self::T5(t5) => Or6::<T1, T2, T3, T4, B, T6>::T5(f(t5)),
            Self::T6(t6) => Or6::<T1, T2, T3, T4, B, T6>::T6(t6),
        }
    }

    /// Transforms the T6 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t6<F, B>(self, f: F) -> Or6<T1, T2, T3, T4, T5, B>
    where
        F: FnOnce(T6) -> B,
    {
        match self {
            Self::T1(t1) => Or6::<T1, T2, T3, T4, T5, B>::T1(t1),
            Self::T2(t2) => Or6::<T1, T2, T3, T4, T5, B>::T2(t2),
            Self::T3(t3) => Or6::<T1, T2, T3, T4, T5, B>::T3(t3),
            Self::T4(t4) => Or6::<T1, T2, T3, T4, T5, B>::T4(t4),
            Self::T5(t5) => Or6::<T1, T2, T3, T4, T5, B>::T5(t5),
            Self::T6(t6) => Or6::<T1, T2, T3, T4, T5, B>::T6(f(t6)),
        }
    }

    /// Consolidates the `Or6` enum into a single value of type `T`,
    /// by applying provided functions.
    pub fn fold<T, F0, F1, F2, F3, F4, F5, F6>(
        self,
        f1: F1,
        f2: F2,
        f3: F3,
        f4: F4,
        f5: F5,
        f6: F6,
    ) -> T
    where
        F1: FnOnce(T1) -> T,
        F2: FnOnce(T2) -> T,
        F3: FnOnce(T3) -> T,
        F4: FnOnce(T4) -> T,
        F5: FnOnce(T5) -> T,
        F6: FnOnce(T6) -> T,
    {
        match self {
            Self::T1(t1) => f1(t1),
            Self::T2(t2) => f2(t2),
            Self::T3(t3) => f3(t3),
            Self::T4(t4) => f4(t4),
            Self::T5(t5) => f5(t5),
            Self::T6(t6) => f6(t6),
        }
    }
}

/// Extension to `Or6` to check if the enum's type matches a arbitrary type.
/// Currently, these functions depend on the rustc intrinsics, and the constraints
/// of the intrinsics require that the type must satisfy `'static'`.
impl<T1, T2, T3, T4, T5, T6> Or6<T1, T2, T3, T4, T5, T6>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
{
    pub fn is_type<T: 'static>(&self) -> bool {
        match self {
            Self::T1(_) => TypeId::of::<T>() == TypeId::of::<T1>(),
            Self::T2(_) => TypeId::of::<T>() == TypeId::of::<T2>(),
            Self::T3(_) => TypeId::of::<T>() == TypeId::of::<T3>(),
            Self::T4(_) => TypeId::of::<T>() == TypeId::of::<T4>(),
            Self::T5(_) => TypeId::of::<T>() == TypeId::of::<T5>(),
            Self::T6(_) => TypeId::of::<T>() == TypeId::of::<T6>(),
        }
    }
}

/// `Or7` is an enum representing a value that can be either of 7 types, T1 ... T7.
pub enum Or7<T1, T2, T3, T4, T5, T6, T7> {
    T1(T1),
    T2(T2),
    T3(T3),
    T4(T4),
    T5(T5),
    T6(T6),
    T7(T7),
}

impl<T1, T2, T3, T4, T5, T6, T7> Or7<T1, T2, T3, T4, T5, T6, T7> {
    /// Returns true if the enum is of type T1.
    pub fn is_t1(&self) -> bool {
        match self {
            Self::T1(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T2.
    pub fn is_t2(&self) -> bool {
        match self {
            Self::T2(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T3.
    pub fn is_t3(&self) -> bool {
        match self {
            Self::T3(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T4.
    pub fn is_t4(&self) -> bool {
        match self {
            Self::T4(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T5.
    pub fn is_t5(&self) -> bool {
        match self {
            Self::T5(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T6.
    pub fn is_t6(&self) -> bool {
        match self {
            Self::T6(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T7.
    pub fn is_t7(&self) -> bool {
        match self {
            Self::T7(_) => true,
            _ => false,
        }
    }

    /// Converts the enum to an Option containing the T1 value, if it is of type T1.
    pub fn as_t1(self) -> Option<T1> {
        match self {
            Self::T1(t1) => Some(t1),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T2 value, if it is of type T2.
    pub fn as_t2(self) -> Option<T2> {
        match self {
            Self::T2(t2) => Some(t2),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T3 value, if it is of type T3.
    pub fn as_t3(self) -> Option<T3> {
        match self {
            Self::T3(t3) => Some(t3),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T4 value, if it is of type T4.
    pub fn as_t4(self) -> Option<T4> {
        match self {
            Self::T4(t4) => Some(t4),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T5 value, if it is of type T5.
    pub fn as_t5(self) -> Option<T5> {
        match self {
            Self::T5(t5) => Some(t5),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T6 value, if it is of type T6.
    pub fn as_t6(self) -> Option<T6> {
        match self {
            Self::T6(t6) => Some(t6),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T7 value, if it is of type T7.
    pub fn as_t7(self) -> Option<T7> {
        match self {
            Self::T7(t7) => Some(t7),
            _ => None,
        }
    }

    /// Transforms the T1 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t1<F, B>(self, f: F) -> Or7<B, T2, T3, T4, T5, T6, T7>
    where
        F: FnOnce(T1) -> B,
    {
        match self {
            Self::T1(t1) => Or7::<B, T2, T3, T4, T5, T6, T7>::T1(f(t1)),
            Self::T2(t2) => Or7::<B, T2, T3, T4, T5, T6, T7>::T2(t2),
            Self::T3(t3) => Or7::<B, T2, T3, T4, T5, T6, T7>::T3(t3),
            Self::T4(t4) => Or7::<B, T2, T3, T4, T5, T6, T7>::T4(t4),
            Self::T5(t5) => Or7::<B, T2, T3, T4, T5, T6, T7>::T5(t5),
            Self::T6(t6) => Or7::<B, T2, T3, T4, T5, T6, T7>::T6(t6),
            Self::T7(t7) => Or7::<B, T2, T3, T4, T5, T6, T7>::T7(t7),
        }
    }

    /// Transforms the T2 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t2<F, B>(self, f: F) -> Or7<T1, B, T3, T4, T5, T6, T7>
    where
        F: FnOnce(T2) -> B,
    {
        match self {
            Self::T1(t1) => Or7::<T1, B, T3, T4, T5, T6, T7>::T1(t1),
            Self::T2(t2) => Or7::<T1, B, T3, T4, T5, T6, T7>::T2(f(t2)),
            Self::T3(t3) => Or7::<T1, B, T3, T4, T5, T6, T7>::T3(t3),
            Self::T4(t4) => Or7::<T1, B, T3, T4, T5, T6, T7>::T4(t4),
            Self::T5(t5) => Or7::<T1, B, T3, T4, T5, T6, T7>::T5(t5),
            Self::T6(t6) => Or7::<T1, B, T3, T4, T5, T6, T7>::T6(t6),
            Self::T7(t7) => Or7::<T1, B, T3, T4, T5, T6, T7>::T7(t7),
        }
    }

    /// Transforms the T3 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t3<F, B>(self, f: F) -> Or7<T1, T2, B, T4, T5, T6, T7>
    where
        F: FnOnce(T3) -> B,
    {
        match self {
            Self::T1(t1) => Or7::<T1, T2, B, T4, T5, T6, T7>::T1(t1),
            Self::T2(t2) => Or7::<T1, T2, B, T4, T5, T6, T7>::T2(t2),
            Self::T3(t3) => Or7::<T1, T2, B, T4, T5, T6, T7>::T3(f(t3)),
            Self::T4(t4) => Or7::<T1, T2, B, T4, T5, T6, T7>::T4(t4),
            Self::T5(t5) => Or7::<T1, T2, B, T4, T5, T6, T7>::T5(t5),
            Self::T6(t6) => Or7::<T1, T2, B, T4, T5, T6, T7>::T6(t6),
            Self::T7(t7) => Or7::<T1, T2, B, T4, T5, T6, T7>::T7(t7),
        }
    }

    /// Transforms the T4 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t4<F, B>(self, f: F) -> Or7<T1, T2, T3, B, T5, T6, T7>
    where
        F: FnOnce(T4) -> B,
    {
        match self {
            Self::T1(t1) => Or7::<T1, T2, T3, B, T5, T6, T7>::T1(t1),
            Self::T2(t2) => Or7::<T1, T2, T3, B, T5, T6, T7>::T2(t2),
            Self::T3(t3) => Or7::<T1, T2, T3, B, T5, T6, T7>::T3(t3),
            Self::T4(t4) => Or7::<T1, T2, T3, B, T5, T6, T7>::T4(f(t4)),
            Self::T5(t5) => Or7::<T1, T2, T3, B, T5, T6, T7>::T5(t5),
            Self::T6(t6) => Or7::<T1, T2, T3, B, T5, T6, T7>::T6(t6),
            Self::T7(t7) => Or7::<T1, T2, T3, B, T5, T6, T7>::T7(t7),
        }
    }

    /// Transforms the T5 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t5<F, B>(self, f: F) -> Or7<T1, T2, T3, T4, B, T6, T7>
    where
        F: FnOnce(T5) -> B,
    {
        match self {
            Self::T1(t1) => Or7::<T1, T2, T3, T4, B, T6, T7>::T1(t1),
            Self::T2(t2) => Or7::<T1, T2, T3, T4, B, T6, T7>::T2(t2),
            Self::T3(t3) => Or7::<T1, T2, T3, T4, B, T6, T7>::T3(t3),
            Self::T4(t4) => Or7::<T1, T2, T3, T4, B, T6, T7>::T4(t4),
            Self::T5(t5) => Or7::<T1, T2, T3, T4, B, T6, T7>::T5(f(t5)),
            Self::T6(t6) => Or7::<T1, T2, T3, T4, B, T6, T7>::T6(t6),
            Self::T7(t7) => Or7::<T1, T2, T3, T4, B, T6, T7>::T7(t7),
        }
    }

    /// Transforms the T6 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t6<F, B>(self, f: F) -> Or7<T1, T2, T3, T4, T5, B, T7>
    where
        F: FnOnce(T6) -> B,
    {
        match self {
            Self::T1(t1) => Or7::<T1, T2, T3, T4, T5, B, T7>::T1(t1),
            Self::T2(t2) => Or7::<T1, T2, T3, T4, T5, B, T7>::T2(t2),
            Self::T3(t3) => Or7::<T1, T2, T3, T4, T5, B, T7>::T3(t3),
            Self::T4(t4) => Or7::<T1, T2, T3, T4, T5, B, T7>::T4(t4),
            Self::T5(t5) => Or7::<T1, T2, T3, T4, T5, B, T7>::T5(t5),
            Self::T6(t6) => Or7::<T1, T2, T3, T4, T5, B, T7>::T6(f(t6)),
            Self::T7(t7) => Or7::<T1, T2, T3, T4, T5, B, T7>::T7(t7),
        }
    }

    /// Transforms the T7 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t7<F, B>(self, f: F) -> Or7<T1, T2, T3, T4, T5, T6, B>
    where
        F: FnOnce(T7) -> B,
    {
        match self {
            Self::T1(t1) => Or7::<T1, T2, T3, T4, T5, T6, B>::T1(t1),
            Self::T2(t2) => Or7::<T1, T2, T3, T4, T5, T6, B>::T2(t2),
            Self::T3(t3) => Or7::<T1, T2, T3, T4, T5, T6, B>::T3(t3),
            Self::T4(t4) => Or7::<T1, T2, T3, T4, T5, T6, B>::T4(t4),
            Self::T5(t5) => Or7::<T1, T2, T3, T4, T5, T6, B>::T5(t5),
            Self::T6(t6) => Or7::<T1, T2, T3, T4, T5, T6, B>::T6(t6),
            Self::T7(t7) => Or7::<T1, T2, T3, T4, T5, T6, B>::T7(f(t7)),
        }
    }

    /// Consolidates the `Or7` enum into a single value of type `T`,
    /// by applying provided functions.
    pub fn fold<T, F0, F1, F2, F3, F4, F5, F6, F7>(
        self,
        f1: F1,
        f2: F2,
        f3: F3,
        f4: F4,
        f5: F5,
        f6: F6,
        f7: F7,
    ) -> T
    where
        F1: FnOnce(T1) -> T,
        F2: FnOnce(T2) -> T,
        F3: FnOnce(T3) -> T,
        F4: FnOnce(T4) -> T,
        F5: FnOnce(T5) -> T,
        F6: FnOnce(T6) -> T,
        F7: FnOnce(T7) -> T,
    {
        match self {
            Self::T1(t1) => f1(t1),
            Self::T2(t2) => f2(t2),
            Self::T3(t3) => f3(t3),
            Self::T4(t4) => f4(t4),
            Self::T5(t5) => f5(t5),
            Self::T6(t6) => f6(t6),
            Self::T7(t7) => f7(t7),
        }
    }
}

/// Extension to `Or7` to check if the enum's type matches a arbitrary type.
/// Currently, these functions depend on the rustc intrinsics, and the constraints
/// of the intrinsics require that the type must satisfy `'static'`.
impl<T1, T2, T3, T4, T5, T6, T7> Or7<T1, T2, T3, T4, T5, T6, T7>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
{
    pub fn is_type<T: 'static>(&self) -> bool {
        match self {
            Self::T1(_) => TypeId::of::<T>() == TypeId::of::<T1>(),
            Self::T2(_) => TypeId::of::<T>() == TypeId::of::<T2>(),
            Self::T3(_) => TypeId::of::<T>() == TypeId::of::<T3>(),
            Self::T4(_) => TypeId::of::<T>() == TypeId::of::<T4>(),
            Self::T5(_) => TypeId::of::<T>() == TypeId::of::<T5>(),
            Self::T6(_) => TypeId::of::<T>() == TypeId::of::<T6>(),
            Self::T7(_) => TypeId::of::<T>() == TypeId::of::<T7>(),
        }
    }
}

/// `Or8` is an enum representing a value that can be either of 8 types, T1 ... T8.
pub enum Or8<T1, T2, T3, T4, T5, T6, T7, T8> {
    T1(T1),
    T2(T2),
    T3(T3),
    T4(T4),
    T5(T5),
    T6(T6),
    T7(T7),
    T8(T8),
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> Or8<T1, T2, T3, T4, T5, T6, T7, T8> {
    /// Returns true if the enum is of type T1.
    pub fn is_t1(&self) -> bool {
        match self {
            Self::T1(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T2.
    pub fn is_t2(&self) -> bool {
        match self {
            Self::T2(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T3.
    pub fn is_t3(&self) -> bool {
        match self {
            Self::T3(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T4.
    pub fn is_t4(&self) -> bool {
        match self {
            Self::T4(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T5.
    pub fn is_t5(&self) -> bool {
        match self {
            Self::T5(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T6.
    pub fn is_t6(&self) -> bool {
        match self {
            Self::T6(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T7.
    pub fn is_t7(&self) -> bool {
        match self {
            Self::T7(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T8.
    pub fn is_t8(&self) -> bool {
        match self {
            Self::T8(_) => true,
            _ => false,
        }
    }

    /// Converts the enum to an Option containing the T1 value, if it is of type T1.
    pub fn as_t1(self) -> Option<T1> {
        match self {
            Self::T1(t1) => Some(t1),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T2 value, if it is of type T2.
    pub fn as_t2(self) -> Option<T2> {
        match self {
            Self::T2(t2) => Some(t2),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T3 value, if it is of type T3.
    pub fn as_t3(self) -> Option<T3> {
        match self {
            Self::T3(t3) => Some(t3),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T4 value, if it is of type T4.
    pub fn as_t4(self) -> Option<T4> {
        match self {
            Self::T4(t4) => Some(t4),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T5 value, if it is of type T5.
    pub fn as_t5(self) -> Option<T5> {
        match self {
            Self::T5(t5) => Some(t5),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T6 value, if it is of type T6.
    pub fn as_t6(self) -> Option<T6> {
        match self {
            Self::T6(t6) => Some(t6),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T7 value, if it is of type T7.
    pub fn as_t7(self) -> Option<T7> {
        match self {
            Self::T7(t7) => Some(t7),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T8 value, if it is of type T8.
    pub fn as_t8(self) -> Option<T8> {
        match self {
            Self::T8(t8) => Some(t8),
            _ => None,
        }
    }

    /// Transforms the T1 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t1<F, B>(self, f: F) -> Or8<B, T2, T3, T4, T5, T6, T7, T8>
    where
        F: FnOnce(T1) -> B,
    {
        match self {
            Self::T1(t1) => Or8::<B, T2, T3, T4, T5, T6, T7, T8>::T1(f(t1)),
            Self::T2(t2) => Or8::<B, T2, T3, T4, T5, T6, T7, T8>::T2(t2),
            Self::T3(t3) => Or8::<B, T2, T3, T4, T5, T6, T7, T8>::T3(t3),
            Self::T4(t4) => Or8::<B, T2, T3, T4, T5, T6, T7, T8>::T4(t4),
            Self::T5(t5) => Or8::<B, T2, T3, T4, T5, T6, T7, T8>::T5(t5),
            Self::T6(t6) => Or8::<B, T2, T3, T4, T5, T6, T7, T8>::T6(t6),
            Self::T7(t7) => Or8::<B, T2, T3, T4, T5, T6, T7, T8>::T7(t7),
            Self::T8(t8) => Or8::<B, T2, T3, T4, T5, T6, T7, T8>::T8(t8),
        }
    }

    /// Transforms the T2 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t2<F, B>(self, f: F) -> Or8<T1, B, T3, T4, T5, T6, T7, T8>
    where
        F: FnOnce(T2) -> B,
    {
        match self {
            Self::T1(t1) => Or8::<T1, B, T3, T4, T5, T6, T7, T8>::T1(t1),
            Self::T2(t2) => Or8::<T1, B, T3, T4, T5, T6, T7, T8>::T2(f(t2)),
            Self::T3(t3) => Or8::<T1, B, T3, T4, T5, T6, T7, T8>::T3(t3),
            Self::T4(t4) => Or8::<T1, B, T3, T4, T5, T6, T7, T8>::T4(t4),
            Self::T5(t5) => Or8::<T1, B, T3, T4, T5, T6, T7, T8>::T5(t5),
            Self::T6(t6) => Or8::<T1, B, T3, T4, T5, T6, T7, T8>::T6(t6),
            Self::T7(t7) => Or8::<T1, B, T3, T4, T5, T6, T7, T8>::T7(t7),
            Self::T8(t8) => Or8::<T1, B, T3, T4, T5, T6, T7, T8>::T8(t8),
        }
    }

    /// Transforms the T3 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t3<F, B>(self, f: F) -> Or8<T1, T2, B, T4, T5, T6, T7, T8>
    where
        F: FnOnce(T3) -> B,
    {
        match self {
            Self::T1(t1) => Or8::<T1, T2, B, T4, T5, T6, T7, T8>::T1(t1),
            Self::T2(t2) => Or8::<T1, T2, B, T4, T5, T6, T7, T8>::T2(t2),
            Self::T3(t3) => Or8::<T1, T2, B, T4, T5, T6, T7, T8>::T3(f(t3)),
            Self::T4(t4) => Or8::<T1, T2, B, T4, T5, T6, T7, T8>::T4(t4),
            Self::T5(t5) => Or8::<T1, T2, B, T4, T5, T6, T7, T8>::T5(t5),
            Self::T6(t6) => Or8::<T1, T2, B, T4, T5, T6, T7, T8>::T6(t6),
            Self::T7(t7) => Or8::<T1, T2, B, T4, T5, T6, T7, T8>::T7(t7),
            Self::T8(t8) => Or8::<T1, T2, B, T4, T5, T6, T7, T8>::T8(t8),
        }
    }

    /// Transforms the T4 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t4<F, B>(self, f: F) -> Or8<T1, T2, T3, B, T5, T6, T7, T8>
    where
        F: FnOnce(T4) -> B,
    {
        match self {
            Self::T1(t1) => Or8::<T1, T2, T3, B, T5, T6, T7, T8>::T1(t1),
            Self::T2(t2) => Or8::<T1, T2, T3, B, T5, T6, T7, T8>::T2(t2),
            Self::T3(t3) => Or8::<T1, T2, T3, B, T5, T6, T7, T8>::T3(t3),
            Self::T4(t4) => Or8::<T1, T2, T3, B, T5, T6, T7, T8>::T4(f(t4)),
            Self::T5(t5) => Or8::<T1, T2, T3, B, T5, T6, T7, T8>::T5(t5),
            Self::T6(t6) => Or8::<T1, T2, T3, B, T5, T6, T7, T8>::T6(t6),
            Self::T7(t7) => Or8::<T1, T2, T3, B, T5, T6, T7, T8>::T7(t7),
            Self::T8(t8) => Or8::<T1, T2, T3, B, T5, T6, T7, T8>::T8(t8),
        }
    }

    /// Transforms the T5 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t5<F, B>(self, f: F) -> Or8<T1, T2, T3, T4, B, T6, T7, T8>
    where
        F: FnOnce(T5) -> B,
    {
        match self {
            Self::T1(t1) => Or8::<T1, T2, T3, T4, B, T6, T7, T8>::T1(t1),
            Self::T2(t2) => Or8::<T1, T2, T3, T4, B, T6, T7, T8>::T2(t2),
            Self::T3(t3) => Or8::<T1, T2, T3, T4, B, T6, T7, T8>::T3(t3),
            Self::T4(t4) => Or8::<T1, T2, T3, T4, B, T6, T7, T8>::T4(t4),
            Self::T5(t5) => Or8::<T1, T2, T3, T4, B, T6, T7, T8>::T5(f(t5)),
            Self::T6(t6) => Or8::<T1, T2, T3, T4, B, T6, T7, T8>::T6(t6),
            Self::T7(t7) => Or8::<T1, T2, T3, T4, B, T6, T7, T8>::T7(t7),
            Self::T8(t8) => Or8::<T1, T2, T3, T4, B, T6, T7, T8>::T8(t8),
        }
    }

    /// Transforms the T6 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t6<F, B>(self, f: F) -> Or8<T1, T2, T3, T4, T5, B, T7, T8>
    where
        F: FnOnce(T6) -> B,
    {
        match self {
            Self::T1(t1) => Or8::<T1, T2, T3, T4, T5, B, T7, T8>::T1(t1),
            Self::T2(t2) => Or8::<T1, T2, T3, T4, T5, B, T7, T8>::T2(t2),
            Self::T3(t3) => Or8::<T1, T2, T3, T4, T5, B, T7, T8>::T3(t3),
            Self::T4(t4) => Or8::<T1, T2, T3, T4, T5, B, T7, T8>::T4(t4),
            Self::T5(t5) => Or8::<T1, T2, T3, T4, T5, B, T7, T8>::T5(t5),
            Self::T6(t6) => Or8::<T1, T2, T3, T4, T5, B, T7, T8>::T6(f(t6)),
            Self::T7(t7) => Or8::<T1, T2, T3, T4, T5, B, T7, T8>::T7(t7),
            Self::T8(t8) => Or8::<T1, T2, T3, T4, T5, B, T7, T8>::T8(t8),
        }
    }

    /// Transforms the T7 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t7<F, B>(self, f: F) -> Or8<T1, T2, T3, T4, T5, T6, B, T8>
    where
        F: FnOnce(T7) -> B,
    {
        match self {
            Self::T1(t1) => Or8::<T1, T2, T3, T4, T5, T6, B, T8>::T1(t1),
            Self::T2(t2) => Or8::<T1, T2, T3, T4, T5, T6, B, T8>::T2(t2),
            Self::T3(t3) => Or8::<T1, T2, T3, T4, T5, T6, B, T8>::T3(t3),
            Self::T4(t4) => Or8::<T1, T2, T3, T4, T5, T6, B, T8>::T4(t4),
            Self::T5(t5) => Or8::<T1, T2, T3, T4, T5, T6, B, T8>::T5(t5),
            Self::T6(t6) => Or8::<T1, T2, T3, T4, T5, T6, B, T8>::T6(t6),
            Self::T7(t7) => Or8::<T1, T2, T3, T4, T5, T6, B, T8>::T7(f(t7)),
            Self::T8(t8) => Or8::<T1, T2, T3, T4, T5, T6, B, T8>::T8(t8),
        }
    }

    /// Transforms the T8 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t8<F, B>(self, f: F) -> Or8<T1, T2, T3, T4, T5, T6, T7, B>
    where
        F: FnOnce(T8) -> B,
    {
        match self {
            Self::T1(t1) => Or8::<T1, T2, T3, T4, T5, T6, T7, B>::T1(t1),
            Self::T2(t2) => Or8::<T1, T2, T3, T4, T5, T6, T7, B>::T2(t2),
            Self::T3(t3) => Or8::<T1, T2, T3, T4, T5, T6, T7, B>::T3(t3),
            Self::T4(t4) => Or8::<T1, T2, T3, T4, T5, T6, T7, B>::T4(t4),
            Self::T5(t5) => Or8::<T1, T2, T3, T4, T5, T6, T7, B>::T5(t5),
            Self::T6(t6) => Or8::<T1, T2, T3, T4, T5, T6, T7, B>::T6(t6),
            Self::T7(t7) => Or8::<T1, T2, T3, T4, T5, T6, T7, B>::T7(t7),
            Self::T8(t8) => Or8::<T1, T2, T3, T4, T5, T6, T7, B>::T8(f(t8)),
        }
    }

    /// Consolidates the `Or8` enum into a single value of type `T`,
    /// by applying provided functions.
    pub fn fold<T, F0, F1, F2, F3, F4, F5, F6, F7, F8>(
        self,
        f1: F1,
        f2: F2,
        f3: F3,
        f4: F4,
        f5: F5,
        f6: F6,
        f7: F7,
        f8: F8,
    ) -> T
    where
        F1: FnOnce(T1) -> T,
        F2: FnOnce(T2) -> T,
        F3: FnOnce(T3) -> T,
        F4: FnOnce(T4) -> T,
        F5: FnOnce(T5) -> T,
        F6: FnOnce(T6) -> T,
        F7: FnOnce(T7) -> T,
        F8: FnOnce(T8) -> T,
    {
        match self {
            Self::T1(t1) => f1(t1),
            Self::T2(t2) => f2(t2),
            Self::T3(t3) => f3(t3),
            Self::T4(t4) => f4(t4),
            Self::T5(t5) => f5(t5),
            Self::T6(t6) => f6(t6),
            Self::T7(t7) => f7(t7),
            Self::T8(t8) => f8(t8),
        }
    }
}

/// Extension to `Or8` to check if the enum's type matches a arbitrary type.
/// Currently, these functions depend on the rustc intrinsics, and the constraints
/// of the intrinsics require that the type must satisfy `'static'`.
impl<T1, T2, T3, T4, T5, T6, T7, T8> Or8<T1, T2, T3, T4, T5, T6, T7, T8>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
{
    pub fn is_type<T: 'static>(&self) -> bool {
        match self {
            Self::T1(_) => TypeId::of::<T>() == TypeId::of::<T1>(),
            Self::T2(_) => TypeId::of::<T>() == TypeId::of::<T2>(),
            Self::T3(_) => TypeId::of::<T>() == TypeId::of::<T3>(),
            Self::T4(_) => TypeId::of::<T>() == TypeId::of::<T4>(),
            Self::T5(_) => TypeId::of::<T>() == TypeId::of::<T5>(),
            Self::T6(_) => TypeId::of::<T>() == TypeId::of::<T6>(),
            Self::T7(_) => TypeId::of::<T>() == TypeId::of::<T7>(),
            Self::T8(_) => TypeId::of::<T>() == TypeId::of::<T8>(),
        }
    }
}

/// `Or9` is an enum representing a value that can be either of 9 types, T1 ... T9.
pub enum Or9<T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    T1(T1),
    T2(T2),
    T3(T3),
    T4(T4),
    T5(T5),
    T6(T6),
    T7(T7),
    T8(T8),
    T9(T9),
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> Or9<T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    /// Returns true if the enum is of type T1.
    pub fn is_t1(&self) -> bool {
        match self {
            Self::T1(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T2.
    pub fn is_t2(&self) -> bool {
        match self {
            Self::T2(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T3.
    pub fn is_t3(&self) -> bool {
        match self {
            Self::T3(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T4.
    pub fn is_t4(&self) -> bool {
        match self {
            Self::T4(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T5.
    pub fn is_t5(&self) -> bool {
        match self {
            Self::T5(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T6.
    pub fn is_t6(&self) -> bool {
        match self {
            Self::T6(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T7.
    pub fn is_t7(&self) -> bool {
        match self {
            Self::T7(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T8.
    pub fn is_t8(&self) -> bool {
        match self {
            Self::T8(_) => true,
            _ => false,
        }
    }

    /// Returns true if the enum is of type T9.
    pub fn is_t9(&self) -> bool {
        match self {
            Self::T9(_) => true,
            _ => false,
        }
    }

    /// Converts the enum to an Option containing the T1 value, if it is of type T1.
    pub fn as_t1(self) -> Option<T1> {
        match self {
            Self::T1(t1) => Some(t1),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T2 value, if it is of type T2.
    pub fn as_t2(self) -> Option<T2> {
        match self {
            Self::T2(t2) => Some(t2),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T3 value, if it is of type T3.
    pub fn as_t3(self) -> Option<T3> {
        match self {
            Self::T3(t3) => Some(t3),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T4 value, if it is of type T4.
    pub fn as_t4(self) -> Option<T4> {
        match self {
            Self::T4(t4) => Some(t4),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T5 value, if it is of type T5.
    pub fn as_t5(self) -> Option<T5> {
        match self {
            Self::T5(t5) => Some(t5),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T6 value, if it is of type T6.
    pub fn as_t6(self) -> Option<T6> {
        match self {
            Self::T6(t6) => Some(t6),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T7 value, if it is of type T7.
    pub fn as_t7(self) -> Option<T7> {
        match self {
            Self::T7(t7) => Some(t7),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T8 value, if it is of type T8.
    pub fn as_t8(self) -> Option<T8> {
        match self {
            Self::T8(t8) => Some(t8),
            _ => None,
        }
    }

    /// Converts the enum to an Option containing the T9 value, if it is of type T9.
    pub fn as_t9(self) -> Option<T9> {
        match self {
            Self::T9(t9) => Some(t9),
            _ => None,
        }
    }

    /// Transforms the T1 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t1<F, B>(self, f: F) -> Or9<B, T2, T3, T4, T5, T6, T7, T8, T9>
    where
        F: FnOnce(T1) -> B,
    {
        match self {
            Self::T1(t1) => Or9::<B, T2, T3, T4, T5, T6, T7, T8, T9>::T1(f(t1)),
            Self::T2(t2) => Or9::<B, T2, T3, T4, T5, T6, T7, T8, T9>::T2(t2),
            Self::T3(t3) => Or9::<B, T2, T3, T4, T5, T6, T7, T8, T9>::T3(t3),
            Self::T4(t4) => Or9::<B, T2, T3, T4, T5, T6, T7, T8, T9>::T4(t4),
            Self::T5(t5) => Or9::<B, T2, T3, T4, T5, T6, T7, T8, T9>::T5(t5),
            Self::T6(t6) => Or9::<B, T2, T3, T4, T5, T6, T7, T8, T9>::T6(t6),
            Self::T7(t7) => Or9::<B, T2, T3, T4, T5, T6, T7, T8, T9>::T7(t7),
            Self::T8(t8) => Or9::<B, T2, T3, T4, T5, T6, T7, T8, T9>::T8(t8),
            Self::T9(t9) => Or9::<B, T2, T3, T4, T5, T6, T7, T8, T9>::T9(t9),
        }
    }

    /// Transforms the T2 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t2<F, B>(self, f: F) -> Or9<T1, B, T3, T4, T5, T6, T7, T8, T9>
    where
        F: FnOnce(T2) -> B,
    {
        match self {
            Self::T1(t1) => Or9::<T1, B, T3, T4, T5, T6, T7, T8, T9>::T1(t1),
            Self::T2(t2) => Or9::<T1, B, T3, T4, T5, T6, T7, T8, T9>::T2(f(t2)),
            Self::T3(t3) => Or9::<T1, B, T3, T4, T5, T6, T7, T8, T9>::T3(t3),
            Self::T4(t4) => Or9::<T1, B, T3, T4, T5, T6, T7, T8, T9>::T4(t4),
            Self::T5(t5) => Or9::<T1, B, T3, T4, T5, T6, T7, T8, T9>::T5(t5),
            Self::T6(t6) => Or9::<T1, B, T3, T4, T5, T6, T7, T8, T9>::T6(t6),
            Self::T7(t7) => Or9::<T1, B, T3, T4, T5, T6, T7, T8, T9>::T7(t7),
            Self::T8(t8) => Or9::<T1, B, T3, T4, T5, T6, T7, T8, T9>::T8(t8),
            Self::T9(t9) => Or9::<T1, B, T3, T4, T5, T6, T7, T8, T9>::T9(t9),
        }
    }

    /// Transforms the T3 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t3<F, B>(self, f: F) -> Or9<T1, T2, B, T4, T5, T6, T7, T8, T9>
    where
        F: FnOnce(T3) -> B,
    {
        match self {
            Self::T1(t1) => Or9::<T1, T2, B, T4, T5, T6, T7, T8, T9>::T1(t1),
            Self::T2(t2) => Or9::<T1, T2, B, T4, T5, T6, T7, T8, T9>::T2(t2),
            Self::T3(t3) => Or9::<T1, T2, B, T4, T5, T6, T7, T8, T9>::T3(f(t3)),
            Self::T4(t4) => Or9::<T1, T2, B, T4, T5, T6, T7, T8, T9>::T4(t4),
            Self::T5(t5) => Or9::<T1, T2, B, T4, T5, T6, T7, T8, T9>::T5(t5),
            Self::T6(t6) => Or9::<T1, T2, B, T4, T5, T6, T7, T8, T9>::T6(t6),
            Self::T7(t7) => Or9::<T1, T2, B, T4, T5, T6, T7, T8, T9>::T7(t7),
            Self::T8(t8) => Or9::<T1, T2, B, T4, T5, T6, T7, T8, T9>::T8(t8),
            Self::T9(t9) => Or9::<T1, T2, B, T4, T5, T6, T7, T8, T9>::T9(t9),
        }
    }

    /// Transforms the T4 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t4<F, B>(self, f: F) -> Or9<T1, T2, T3, B, T5, T6, T7, T8, T9>
    where
        F: FnOnce(T4) -> B,
    {
        match self {
            Self::T1(t1) => Or9::<T1, T2, T3, B, T5, T6, T7, T8, T9>::T1(t1),
            Self::T2(t2) => Or9::<T1, T2, T3, B, T5, T6, T7, T8, T9>::T2(t2),
            Self::T3(t3) => Or9::<T1, T2, T3, B, T5, T6, T7, T8, T9>::T3(t3),
            Self::T4(t4) => Or9::<T1, T2, T3, B, T5, T6, T7, T8, T9>::T4(f(t4)),
            Self::T5(t5) => Or9::<T1, T2, T3, B, T5, T6, T7, T8, T9>::T5(t5),
            Self::T6(t6) => Or9::<T1, T2, T3, B, T5, T6, T7, T8, T9>::T6(t6),
            Self::T7(t7) => Or9::<T1, T2, T3, B, T5, T6, T7, T8, T9>::T7(t7),
            Self::T8(t8) => Or9::<T1, T2, T3, B, T5, T6, T7, T8, T9>::T8(t8),
            Self::T9(t9) => Or9::<T1, T2, T3, B, T5, T6, T7, T8, T9>::T9(t9),
        }
    }

    /// Transforms the T5 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t5<F, B>(self, f: F) -> Or9<T1, T2, T3, T4, B, T6, T7, T8, T9>
    where
        F: FnOnce(T5) -> B,
    {
        match self {
            Self::T1(t1) => Or9::<T1, T2, T3, T4, B, T6, T7, T8, T9>::T1(t1),
            Self::T2(t2) => Or9::<T1, T2, T3, T4, B, T6, T7, T8, T9>::T2(t2),
            Self::T3(t3) => Or9::<T1, T2, T3, T4, B, T6, T7, T8, T9>::T3(t3),
            Self::T4(t4) => Or9::<T1, T2, T3, T4, B, T6, T7, T8, T9>::T4(t4),
            Self::T5(t5) => Or9::<T1, T2, T3, T4, B, T6, T7, T8, T9>::T5(f(t5)),
            Self::T6(t6) => Or9::<T1, T2, T3, T4, B, T6, T7, T8, T9>::T6(t6),
            Self::T7(t7) => Or9::<T1, T2, T3, T4, B, T6, T7, T8, T9>::T7(t7),
            Self::T8(t8) => Or9::<T1, T2, T3, T4, B, T6, T7, T8, T9>::T8(t8),
            Self::T9(t9) => Or9::<T1, T2, T3, T4, B, T6, T7, T8, T9>::T9(t9),
        }
    }

    /// Transforms the T6 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t6<F, B>(self, f: F) -> Or9<T1, T2, T3, T4, T5, B, T7, T8, T9>
    where
        F: FnOnce(T6) -> B,
    {
        match self {
            Self::T1(t1) => Or9::<T1, T2, T3, T4, T5, B, T7, T8, T9>::T1(t1),
            Self::T2(t2) => Or9::<T1, T2, T3, T4, T5, B, T7, T8, T9>::T2(t2),
            Self::T3(t3) => Or9::<T1, T2, T3, T4, T5, B, T7, T8, T9>::T3(t3),
            Self::T4(t4) => Or9::<T1, T2, T3, T4, T5, B, T7, T8, T9>::T4(t4),
            Self::T5(t5) => Or9::<T1, T2, T3, T4, T5, B, T7, T8, T9>::T5(t5),
            Self::T6(t6) => Or9::<T1, T2, T3, T4, T5, B, T7, T8, T9>::T6(f(t6)),
            Self::T7(t7) => Or9::<T1, T2, T3, T4, T5, B, T7, T8, T9>::T7(t7),
            Self::T8(t8) => Or9::<T1, T2, T3, T4, T5, B, T7, T8, T9>::T8(t8),
            Self::T9(t9) => Or9::<T1, T2, T3, T4, T5, B, T7, T8, T9>::T9(t9),
        }
    }

    /// Transforms the T7 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t7<F, B>(self, f: F) -> Or9<T1, T2, T3, T4, T5, T6, B, T8, T9>
    where
        F: FnOnce(T7) -> B,
    {
        match self {
            Self::T1(t1) => Or9::<T1, T2, T3, T4, T5, T6, B, T8, T9>::T1(t1),
            Self::T2(t2) => Or9::<T1, T2, T3, T4, T5, T6, B, T8, T9>::T2(t2),
            Self::T3(t3) => Or9::<T1, T2, T3, T4, T5, T6, B, T8, T9>::T3(t3),
            Self::T4(t4) => Or9::<T1, T2, T3, T4, T5, T6, B, T8, T9>::T4(t4),
            Self::T5(t5) => Or9::<T1, T2, T3, T4, T5, T6, B, T8, T9>::T5(t5),
            Self::T6(t6) => Or9::<T1, T2, T3, T4, T5, T6, B, T8, T9>::T6(t6),
            Self::T7(t7) => Or9::<T1, T2, T3, T4, T5, T6, B, T8, T9>::T7(f(t7)),
            Self::T8(t8) => Or9::<T1, T2, T3, T4, T5, T6, B, T8, T9>::T8(t8),
            Self::T9(t9) => Or9::<T1, T2, T3, T4, T5, T6, B, T8, T9>::T9(t9),
        }
    }

    /// Transforms the T8 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t8<F, B>(self, f: F) -> Or9<T1, T2, T3, T4, T5, T6, T7, B, T9>
    where
        F: FnOnce(T8) -> B,
    {
        match self {
            Self::T1(t1) => Or9::<T1, T2, T3, T4, T5, T6, T7, B, T9>::T1(t1),
            Self::T2(t2) => Or9::<T1, T2, T3, T4, T5, T6, T7, B, T9>::T2(t2),
            Self::T3(t3) => Or9::<T1, T2, T3, T4, T5, T6, T7, B, T9>::T3(t3),
            Self::T4(t4) => Or9::<T1, T2, T3, T4, T5, T6, T7, B, T9>::T4(t4),
            Self::T5(t5) => Or9::<T1, T2, T3, T4, T5, T6, T7, B, T9>::T5(t5),
            Self::T6(t6) => Or9::<T1, T2, T3, T4, T5, T6, T7, B, T9>::T6(t6),
            Self::T7(t7) => Or9::<T1, T2, T3, T4, T5, T6, T7, B, T9>::T7(t7),
            Self::T8(t8) => Or9::<T1, T2, T3, T4, T5, T6, T7, B, T9>::T8(f(t8)),
            Self::T9(t9) => Or9::<T1, T2, T3, T4, T5, T6, T7, B, T9>::T9(t9),
        }
    }

    /// Transforms the T9 value of the enum using a provided function,
    /// maintaining other types as is.
    pub fn map_t9<F, B>(self, f: F) -> Or9<T1, T2, T3, T4, T5, T6, T7, T8, B>
    where
        F: FnOnce(T9) -> B,
    {
        match self {
            Self::T1(t1) => Or9::<T1, T2, T3, T4, T5, T6, T7, T8, B>::T1(t1),
            Self::T2(t2) => Or9::<T1, T2, T3, T4, T5, T6, T7, T8, B>::T2(t2),
            Self::T3(t3) => Or9::<T1, T2, T3, T4, T5, T6, T7, T8, B>::T3(t3),
            Self::T4(t4) => Or9::<T1, T2, T3, T4, T5, T6, T7, T8, B>::T4(t4),
            Self::T5(t5) => Or9::<T1, T2, T3, T4, T5, T6, T7, T8, B>::T5(t5),
            Self::T6(t6) => Or9::<T1, T2, T3, T4, T5, T6, T7, T8, B>::T6(t6),
            Self::T7(t7) => Or9::<T1, T2, T3, T4, T5, T6, T7, T8, B>::T7(t7),
            Self::T8(t8) => Or9::<T1, T2, T3, T4, T5, T6, T7, T8, B>::T8(t8),
            Self::T9(t9) => Or9::<T1, T2, T3, T4, T5, T6, T7, T8, B>::T9(f(t9)),
        }
    }

    /// Consolidates the `Or9` enum into a single value of type `T`,
    /// by applying provided functions.
    pub fn fold<T, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9>(
        self,
        f1: F1,
        f2: F2,
        f3: F3,
        f4: F4,
        f5: F5,
        f6: F6,
        f7: F7,
        f8: F8,
        f9: F9,
    ) -> T
    where
        F1: FnOnce(T1) -> T,
        F2: FnOnce(T2) -> T,
        F3: FnOnce(T3) -> T,
        F4: FnOnce(T4) -> T,
        F5: FnOnce(T5) -> T,
        F6: FnOnce(T6) -> T,
        F7: FnOnce(T7) -> T,
        F8: FnOnce(T8) -> T,
        F9: FnOnce(T9) -> T,
    {
        match self {
            Self::T1(t1) => f1(t1),
            Self::T2(t2) => f2(t2),
            Self::T3(t3) => f3(t3),
            Self::T4(t4) => f4(t4),
            Self::T5(t5) => f5(t5),
            Self::T6(t6) => f6(t6),
            Self::T7(t7) => f7(t7),
            Self::T8(t8) => f8(t8),
            Self::T9(t9) => f9(t9),
        }
    }
}

/// Extension to `Or9` to check if the enum's type matches a arbitrary type.
/// Currently, these functions depend on the rustc intrinsics, and the constraints
/// of the intrinsics require that the type must satisfy `'static'`.
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> Or9<T1, T2, T3, T4, T5, T6, T7, T8, T9>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
    T9: 'static,
{
    pub fn is_type<T: 'static>(&self) -> bool {
        match self {
            Self::T1(_) => TypeId::of::<T>() == TypeId::of::<T1>(),
            Self::T2(_) => TypeId::of::<T>() == TypeId::of::<T2>(),
            Self::T3(_) => TypeId::of::<T>() == TypeId::of::<T3>(),
            Self::T4(_) => TypeId::of::<T>() == TypeId::of::<T4>(),
            Self::T5(_) => TypeId::of::<T>() == TypeId::of::<T5>(),
            Self::T6(_) => TypeId::of::<T>() == TypeId::of::<T6>(),
            Self::T7(_) => TypeId::of::<T>() == TypeId::of::<T7>(),
            Self::T8(_) => TypeId::of::<T>() == TypeId::of::<T8>(),
            Self::T9(_) => TypeId::of::<T>() == TypeId::of::<T9>(),
        }
    }
}
