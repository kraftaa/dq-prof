use polars::prelude::DataType;

use crate::types::LogicalType;

pub fn infer_logical_type(dtype: &DataType) -> LogicalType {
    match dtype {
        DataType::Int8
        | DataType::Int16
        | DataType::Int32
        | DataType::Int64
        | DataType::UInt8
        | DataType::UInt16
        | DataType::UInt32
        | DataType::UInt64
        | DataType::Float32
        | DataType::Float64 => LogicalType::Numeric,
        DataType::Boolean => LogicalType::Boolean,
        DataType::Date | DataType::Datetime(_, _) => LogicalType::Datetime,
        DataType::String | DataType::Categorical(_, _) | DataType::Enum(_, _) => {
            LogicalType::Categorical
        }
        _ => LogicalType::Other,
    }
}
