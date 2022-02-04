// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_describe_dimension_keys_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDimensionKeysInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.service_type {
        object.key("ServiceType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identifier {
        object.key("Identifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_4, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_5) = &input.metric {
        object.key("Metric").string(var_5.as_str());
    }
    if let Some(var_6) = &input.period_in_seconds {
        object.key("PeriodInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.group_by {
        let mut object_8 = object.key("GroupBy").start_object();
        crate::json_ser::serialize_structure_crate_model_dimension_group(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.additional_metrics {
        let mut array_10 = object.key("AdditionalMetrics").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.partition_by {
        let mut object_13 = object.key("PartitionBy").start_object();
        crate::json_ser::serialize_structure_crate_model_dimension_group(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.filter {
        let mut object_15 = object.key("Filter").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    if let Some(var_18) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_18).into()),
        );
    }
    if let Some(var_19) = &input.next_token {
        object.key("NextToken").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_dimension_key_details_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDimensionKeyDetailsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.service_type {
        object.key("ServiceType").string(var_20.as_str());
    }
    if let Some(var_21) = &input.identifier {
        object.key("Identifier").string(var_21.as_str());
    }
    if let Some(var_22) = &input.group {
        object.key("Group").string(var_22.as_str());
    }
    if let Some(var_23) = &input.group_identifier {
        object.key("GroupIdentifier").string(var_23.as_str());
    }
    if let Some(var_24) = &input.requested_dimensions {
        let mut array_25 = object.key("RequestedDimensions").start_array();
        for item_26 in var_24 {
            {
                array_25.value().string(item_26.as_str());
            }
        }
        array_25.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_resource_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourceMetadataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.service_type {
        object.key("ServiceType").string(var_27.as_str());
    }
    if let Some(var_28) = &input.identifier {
        object.key("Identifier").string(var_28.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_resource_metrics_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourceMetricsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.service_type {
        object.key("ServiceType").string(var_29.as_str());
    }
    if let Some(var_30) = &input.identifier {
        object.key("Identifier").string(var_30.as_str());
    }
    if let Some(var_31) = &input.metric_queries {
        let mut array_32 = object.key("MetricQueries").start_array();
        for item_33 in var_31 {
            {
                let mut object_34 = array_32.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_query(
                    &mut object_34,
                    item_33,
                )?;
                object_34.finish();
            }
        }
        array_32.finish();
    }
    if let Some(var_35) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_35, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_36) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_36, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_37) = &input.period_in_seconds {
        object.key("PeriodInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_37).into()),
        );
    }
    if let Some(var_38) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_38).into()),
        );
    }
    if let Some(var_39) = &input.next_token {
        object.key("NextToken").string(var_39.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_available_resource_dimensions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAvailableResourceDimensionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.service_type {
        object.key("ServiceType").string(var_40.as_str());
    }
    if let Some(var_41) = &input.identifier {
        object.key("Identifier").string(var_41.as_str());
    }
    if let Some(var_42) = &input.metrics {
        let mut array_43 = object.key("Metrics").start_array();
        for item_44 in var_42 {
            {
                array_43.value().string(item_44.as_str());
            }
        }
        array_43.finish();
    }
    if let Some(var_45) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_45).into()),
        );
    }
    if let Some(var_46) = &input.next_token {
        object.key("NextToken").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_available_resource_metrics_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAvailableResourceMetricsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.service_type {
        object.key("ServiceType").string(var_47.as_str());
    }
    if let Some(var_48) = &input.identifier {
        object.key("Identifier").string(var_48.as_str());
    }
    if let Some(var_49) = &input.metric_types {
        let mut array_50 = object.key("MetricTypes").start_array();
        for item_51 in var_49 {
            {
                array_50.value().string(item_51.as_str());
            }
        }
        array_50.finish();
    }
    if let Some(var_52) = &input.next_token {
        object.key("NextToken").string(var_52.as_str());
    }
    if let Some(var_53) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_53).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dimension_group(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DimensionGroup,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.group {
        object.key("Group").string(var_54.as_str());
    }
    if let Some(var_55) = &input.dimensions {
        let mut array_56 = object.key("Dimensions").start_array();
        for item_57 in var_55 {
            {
                array_56.value().string(item_57.as_str());
            }
        }
        array_56.finish();
    }
    if let Some(var_58) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_58).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_metric_query(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MetricQuery,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.metric {
        object.key("Metric").string(var_59.as_str());
    }
    if let Some(var_60) = &input.group_by {
        let mut object_61 = object.key("GroupBy").start_object();
        crate::json_ser::serialize_structure_crate_model_dimension_group(&mut object_61, var_60)?;
        object_61.finish();
    }
    if let Some(var_62) = &input.filter {
        let mut object_63 = object.key("Filter").start_object();
        for (key_64, value_65) in var_62 {
            {
                object_63.key(key_64).string(value_65.as_str());
            }
        }
        object_63.finish();
    }
    Ok(())
}
