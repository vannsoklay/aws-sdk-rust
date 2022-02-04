// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_describe_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeStreamInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.stream_arn {
        object.key("StreamArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.exclusive_start_shard_id {
        object.key("ExclusiveStartShardId").string(var_3.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_records_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRecordsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.shard_iterator {
        object.key("ShardIterator").string(var_4.as_str());
    }
    if let Some(var_5) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_shard_iterator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetShardIteratorInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.stream_arn {
        object.key("StreamArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.shard_id {
        object.key("ShardId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.shard_iterator_type {
        object.key("ShardIteratorType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.sequence_number {
        object.key("SequenceNumber").string(var_9.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_streams_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListStreamsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.table_name {
        object.key("TableName").string(var_10.as_str());
    }
    if let Some(var_11) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.exclusive_start_stream_arn {
        object
            .key("ExclusiveStartStreamArn")
            .string(var_12.as_str());
    }
    Ok(())
}
