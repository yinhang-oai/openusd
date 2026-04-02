use std::collections::HashMap;

use half::f16;
use strum::{EnumIs, EnumTryAs};

use super::*;

/// `Value` is a type that can hold data type loaded from USD file.
///
/// Suffixes:
/// - d: double
/// - f: float
/// - h: half
/// - i: int
///
#[derive(Debug, Clone, PartialEq, EnumIs, EnumTryAs)]
pub enum Value {
    /// None value, only produced by expressions (not directly assignable).
    None,

    Bool(bool),
    BoolVec(Vec<bool>),

    Uchar(u8),
    UcharVec(Vec<u8>),

    Int(i32),
    IntVec(Vec<i32>),

    Uint(u32),
    UintVec(Vec<u32>),

    Int64(i64),
    Int64Vec(Vec<i64>),

    Uint64(u64),
    Uint64Vec(Vec<u64>),

    Half(f16),
    HalfVec(Vec<f16>),

    Float(f32),
    FloatVec(Vec<f32>),

    Double(f64),
    DoubleVec(Vec<f64>),

    String(String),
    StringVec(Vec<String>),

    Token(String),
    TokenVec(Vec<String>),

    AssetPath(String),

    Quath(Vec<f16>),
    Quatf(Vec<f32>),
    Quatd(Vec<f64>),

    Vec2h(Vec<f16>),
    Vec2f(Vec<f32>),
    Vec2d(Vec<f64>),
    Vec2i(Vec<i32>),

    Vec3h(Vec<f16>),
    Vec3f(Vec<f32>),
    Vec3d(Vec<f64>),
    Vec3i(Vec<i32>),

    Vec4h(Vec<f16>),
    Vec4f(Vec<f32>),
    Vec4d(Vec<f64>),
    Vec4i(Vec<i32>),

    Matrix2d(Vec<f64>),
    Matrix3d(Vec<f64>),
    Matrix4d(Vec<f64>),

    Specifier(Specifier),
    Permission(Permission),
    Variability(Variability),

    Dictionary(HashMap<String, Value>),

    TokenListOp(TokenListOp),
    StringListOp(StringListOp),
    PathListOp(PathListOp),
    ReferenceListOp(ReferenceListOp),
    IntListOp(IntListOp),
    Int64ListOp(Int64ListOp),
    UIntListOp(UintListOp),
    UInt64ListOp(Uint64ListOp),
    PayloadListOp(PayloadListOp),

    Payload(Payload),
    PathVec,
    VariantSelectionMap(HashMap<String, String>),
    TimeSamples(TimeSampleMap),

    LayerOffsetVec(Vec<LayerOffset>),

    ValueBlock,
    Value,

    UnregisteredValue,
    UnregisteredValueListOp,

    TimeCode(f64),
    PathExpression,
}

impl Value {
    /// Returns string-like scalar values without allocating.
    #[inline]
    pub fn as_str_like(&self) -> Option<&str> {
        match self {
            Self::String(value) | Self::Token(value) | Self::AssetPath(value) => Some(value),
            _ => None,
        }
    }

    /// Returns a mixed numeric scalar as `f64`.
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Half(value) => Some(value.to_f64()),
            Self::Float(value) => Some(*value as f64),
            Self::Double(value) | Self::TimeCode(value) => Some(*value),
            Self::Int(value) => Some(*value as f64),
            Self::Uint(value) => Some(*value as f64),
            Self::Int64(value) => Some(*value as f64),
            Self::Uint64(value) => Some(*value as f64),
            _ => None,
        }
    }

    /// Returns an integer scalar as `i32` when it fits.
    #[inline]
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Self::Int(value) => Some(*value),
            Self::Uint(value) => i32::try_from(*value).ok(),
            Self::Int64(value) => i32::try_from(*value).ok(),
            Self::Uint64(value) => i32::try_from(*value).ok(),
            _ => None,
        }
    }

    /// Returns a 3-vector as `f64` values.
    #[inline]
    pub fn as_vec3_f64(&self) -> Option<[f64; 3]> {
        match self {
            Self::Vec3h(values) if values.len() >= 3 => Some([
                values[0].to_f64(),
                values[1].to_f64(),
                values[2].to_f64(),
            ]),
            Self::Vec3f(values) if values.len() >= 3 => {
                Some([values[0] as f64, values[1] as f64, values[2] as f64])
            }
            Self::Vec3d(values) if values.len() >= 3 => Some([values[0], values[1], values[2]]),
            Self::Vec3i(values) if values.len() >= 3 => {
                Some([values[0] as f64, values[1] as f64, values[2] as f64])
            }
            _ => None,
        }
    }

    /// Returns a quaternion as `[w, x, y, z]` in `f64`.
    #[inline]
    pub fn as_quat_wxyz_f64(&self) -> Option<[f64; 4]> {
        match self {
            Self::Quath(values) if values.len() >= 4 => Some([
                values[0].to_f64(),
                values[1].to_f64(),
                values[2].to_f64(),
                values[3].to_f64(),
            ]),
            Self::Quatf(values) if values.len() >= 4 => Some([
                values[0] as f64,
                values[1] as f64,
                values[2] as f64,
                values[3] as f64,
            ]),
            Self::Quatd(values) if values.len() >= 4 => {
                Some([values[0], values[1], values[2], values[3]])
            }
            _ => None,
        }
    }

    /// Returns numeric arrays as `f64` values.
    #[inline]
    pub fn as_f64_vec(&self) -> Option<Vec<f64>> {
        match self {
            Self::HalfVec(values) => Some(values.iter().map(|value| value.to_f64()).collect()),
            Self::FloatVec(values) => Some(values.iter().map(|value| *value as f64).collect()),
            Self::DoubleVec(values) => Some(values.clone()),
            Self::IntVec(values) => Some(values.iter().map(|value| *value as f64).collect()),
            Self::UintVec(values) => Some(values.iter().map(|value| *value as f64).collect()),
            Self::Int64Vec(values) => Some(values.iter().map(|value| *value as f64).collect()),
            Self::Uint64Vec(values) => Some(values.iter().map(|value| *value as f64).collect()),
            _ => None,
        }
    }

    /// Returns token-list-like values in authored list-op order.
    #[inline]
    pub fn as_token_list(&self) -> Vec<String> {
        match self {
            Self::Token(value) => vec![value.clone()],
            Self::TokenVec(values) => values.clone(),
            Self::TokenListOp(op) => {
                list_op_items(&op.explicit_items, &op.prepended_items, &op.added_items)
            }
            _ => Vec::new(),
        }
    }

    /// Returns path-list-like values in authored list-op order.
    #[inline]
    pub fn as_path_list(&self) -> Vec<Path> {
        match self {
            Self::PathListOp(op) => {
                list_op_items(&op.explicit_items, &op.prepended_items, &op.added_items)
            }
            Self::PathVec => Vec::new(),
            _ => Vec::new(),
        }
    }
}

fn list_op_items<T: Clone>(
    explicit_items: &[T],
    prepended_items: &[T],
    added_items: &[T],
) -> Vec<T> {
    if !explicit_items.is_empty() {
        explicit_items.to_vec()
    } else if !prepended_items.is_empty() {
        prepended_items.to_vec()
    } else {
        added_items.to_vec()
    }
}

/// Convert from `&str` to `Value`.
///
/// Used a lot in text parser since all tokens are basically strings.
impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Value::String(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is() {
        // Basic sanity checks
        assert!(Value::Bool(true).is_bool());
        assert!(!Value::Bool(true).is_bool_vec());

        assert!(Value::Float(1.44).is_float());
        assert!(!Value::Float(1.44).is_bool());
        assert!(!Value::Float(1.44).is_float_vec());

        assert!(Value::PayloadListOp(Default::default()).is_payload_list_op());
        assert!(Value::UnregisteredValue.is_unregistered_value());
    }
}
