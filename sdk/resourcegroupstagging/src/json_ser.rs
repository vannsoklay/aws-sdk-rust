// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_get_compliance_summary_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetComplianceSummaryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.target_id_filters {
        let mut array_2 = object.key("TargetIdFilters").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.region_filters {
        let mut array_5 = object.key("RegionFilters").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.resource_type_filters {
        let mut array_8 = object.key("ResourceTypeFilters").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.tag_key_filters {
        let mut array_11 = object.key("TagKeyFilters").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.group_by {
        let mut array_14 = object.key("GroupBy").start_array();
        for item_15 in var_13 {
            {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    if let Some(var_16) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    if let Some(var_17) = &input.pagination_token {
        object.key("PaginationToken").string(var_17.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_resources_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourcesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.pagination_token {
        object.key("PaginationToken").string(var_18.as_str());
    }
    if let Some(var_19) = &input.tag_filters {
        let mut array_20 = object.key("TagFilters").start_array();
        for item_21 in var_19 {
            {
                let mut object_22 = array_20.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag_filter(
                    &mut object_22,
                    item_21,
                )?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    if let Some(var_23) = &input.resources_per_page {
        object.key("ResourcesPerPage").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_23).into()),
        );
    }
    if let Some(var_24) = &input.tags_per_page {
        object.key("TagsPerPage").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_24).into()),
        );
    }
    if let Some(var_25) = &input.resource_type_filters {
        let mut array_26 = object.key("ResourceTypeFilters").start_array();
        for item_27 in var_25 {
            {
                array_26.value().string(item_27.as_str());
            }
        }
        array_26.finish();
    }
    if let Some(var_28) = &input.include_compliance_details {
        object.key("IncludeComplianceDetails").boolean(*var_28);
    }
    if let Some(var_29) = &input.exclude_compliant_resources {
        object.key("ExcludeCompliantResources").boolean(*var_29);
    }
    if let Some(var_30) = &input.resource_arn_list {
        let mut array_31 = object.key("ResourceARNList").start_array();
        for item_32 in var_30 {
            {
                array_31.value().string(item_32.as_str());
            }
        }
        array_31.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_tag_keys_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTagKeysInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.pagination_token {
        object.key("PaginationToken").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_tag_values_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTagValuesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.pagination_token {
        object.key("PaginationToken").string(var_34.as_str());
    }
    if let Some(var_35) = &input.key {
        object.key("Key").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_report_creation_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartReportCreationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.s3_bucket {
        object.key("S3Bucket").string(var_36.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resources_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourcesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.resource_arn_list {
        let mut array_38 = object.key("ResourceARNList").start_array();
        for item_39 in var_37 {
            {
                array_38.value().string(item_39.as_str());
            }
        }
        array_38.finish();
    }
    if let Some(var_40) = &input.tags {
        let mut object_41 = object.key("Tags").start_object();
        for (key_42, value_43) in var_40 {
            {
                object_41.key(key_42).string(value_43.as_str());
            }
        }
        object_41.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resources_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourcesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.resource_arn_list {
        let mut array_45 = object.key("ResourceARNList").start_array();
        for item_46 in var_44 {
            {
                array_45.value().string(item_46.as_str());
            }
        }
        array_45.finish();
    }
    if let Some(var_47) = &input.tag_keys {
        let mut array_48 = object.key("TagKeys").start_array();
        for item_49 in var_47 {
            {
                array_48.value().string(item_49.as_str());
            }
        }
        array_48.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TagFilter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.key {
        object.key("Key").string(var_50.as_str());
    }
    if let Some(var_51) = &input.values {
        let mut array_52 = object.key("Values").start_array();
        for item_53 in var_51 {
            {
                array_52.value().string(item_53.as_str());
            }
        }
        array_52.finish();
    }
    Ok(())
}
