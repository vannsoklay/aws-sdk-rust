// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateClusterInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.cluster_name {
        object.key("ClusterName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.node_type {
        object.key("NodeType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    {
        object.key("ReplicationFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.replication_factor).into()),
        );
    }
    if let Some(var_4) = &input.availability_zones {
        let mut array_5 = object.key("AvailabilityZones").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.subnet_group_name {
        object.key("SubnetGroupName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.security_group_ids {
        let mut array_9 = object.key("SecurityGroupIds").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.preferred_maintenance_window {
        object
            .key("PreferredMaintenanceWindow")
            .string(var_11.as_str());
    }
    if let Some(var_12) = &input.notification_topic_arn {
        object.key("NotificationTopicArn").string(var_12.as_str());
    }
    if let Some(var_13) = &input.iam_role_arn {
        object.key("IamRoleArn").string(var_13.as_str());
    }
    if let Some(var_14) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_14.as_str());
    }
    if let Some(var_15) = &input.tags {
        let mut array_16 = object.key("Tags").start_array();
        for item_17 in var_15 {
            {
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    if let Some(var_19) = &input.sse_specification {
        let mut object_20 = object.key("SSESpecification").start_object();
        crate::json_ser::serialize_structure_crate_model_sse_specification(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.cluster_endpoint_encryption_type {
        object
            .key("ClusterEndpointEncryptionType")
            .string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_parameter_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateParameterGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_22.as_str());
    }
    if let Some(var_23) = &input.description {
        object.key("Description").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_subnet_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSubnetGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.subnet_group_name {
        object.key("SubnetGroupName").string(var_24.as_str());
    }
    if let Some(var_25) = &input.description {
        object.key("Description").string(var_25.as_str());
    }
    if let Some(var_26) = &input.subnet_ids {
        let mut array_27 = object.key("SubnetIds").start_array();
        for item_28 in var_26 {
            {
                array_27.value().string(item_28.as_str());
            }
        }
        array_27.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_decrease_replication_factor_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DecreaseReplicationFactorInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.cluster_name {
        object.key("ClusterName").string(var_29.as_str());
    }
    {
        object.key("NewReplicationFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.new_replication_factor).into()),
        );
    }
    if let Some(var_30) = &input.availability_zones {
        let mut array_31 = object.key("AvailabilityZones").start_array();
        for item_32 in var_30 {
            {
                array_31.value().string(item_32.as_str());
            }
        }
        array_31.finish();
    }
    if let Some(var_33) = &input.node_ids_to_remove {
        let mut array_34 = object.key("NodeIdsToRemove").start_array();
        for item_35 in var_33 {
            {
                array_34.value().string(item_35.as_str());
            }
        }
        array_34.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteClusterInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.cluster_name {
        object.key("ClusterName").string(var_36.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_parameter_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteParameterGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_subnet_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSubnetGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.subnet_group_name {
        object.key("SubnetGroupName").string(var_38.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_clusters_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeClustersInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.cluster_names {
        let mut array_40 = object.key("ClusterNames").start_array();
        for item_41 in var_39 {
            {
                array_40.value().string(item_41.as_str());
            }
        }
        array_40.finish();
    }
    if let Some(var_42) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_42).into()),
        );
    }
    if let Some(var_43) = &input.next_token {
        object.key("NextToken").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_default_parameters_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDefaultParametersInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_44).into()),
        );
    }
    if let Some(var_45) = &input.next_token {
        object.key("NextToken").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_events_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeEventsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.source_name {
        object.key("SourceName").string(var_46.as_str());
    }
    if let Some(var_47) = &input.source_type {
        object.key("SourceType").string(var_47.as_str());
    }
    if let Some(var_48) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_48, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_49) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_49, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_50) = &input.duration {
        object.key("Duration").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_50).into()),
        );
    }
    if let Some(var_51) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_51).into()),
        );
    }
    if let Some(var_52) = &input.next_token {
        object.key("NextToken").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_parameter_groups_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeParameterGroupsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.parameter_group_names {
        let mut array_54 = object.key("ParameterGroupNames").start_array();
        for item_55 in var_53 {
            {
                array_54.value().string(item_55.as_str());
            }
        }
        array_54.finish();
    }
    if let Some(var_56) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_56).into()),
        );
    }
    if let Some(var_57) = &input.next_token {
        object.key("NextToken").string(var_57.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_parameters_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeParametersInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_58.as_str());
    }
    if let Some(var_59) = &input.source {
        object.key("Source").string(var_59.as_str());
    }
    if let Some(var_60) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_60).into()),
        );
    }
    if let Some(var_61) = &input.next_token {
        object.key("NextToken").string(var_61.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_subnet_groups_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSubnetGroupsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.subnet_group_names {
        let mut array_63 = object.key("SubnetGroupNames").start_array();
        for item_64 in var_62 {
            {
                array_63.value().string(item_64.as_str());
            }
        }
        array_63.finish();
    }
    if let Some(var_65) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_65).into()),
        );
    }
    if let Some(var_66) = &input.next_token {
        object.key("NextToken").string(var_66.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_increase_replication_factor_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::IncreaseReplicationFactorInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.cluster_name {
        object.key("ClusterName").string(var_67.as_str());
    }
    {
        object.key("NewReplicationFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.new_replication_factor).into()),
        );
    }
    if let Some(var_68) = &input.availability_zones {
        let mut array_69 = object.key("AvailabilityZones").start_array();
        for item_70 in var_68 {
            {
                array_69.value().string(item_70.as_str());
            }
        }
        array_69.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_71) = &input.resource_name {
        object.key("ResourceName").string(var_71.as_str());
    }
    if let Some(var_72) = &input.next_token {
        object.key("NextToken").string(var_72.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_reboot_node_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RebootNodeInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_73) = &input.cluster_name {
        object.key("ClusterName").string(var_73.as_str());
    }
    if let Some(var_74) = &input.node_id {
        object.key("NodeId").string(var_74.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_75) = &input.resource_name {
        object.key("ResourceName").string(var_75.as_str());
    }
    if let Some(var_76) = &input.tags {
        let mut array_77 = object.key("Tags").start_array();
        for item_78 in var_76 {
            {
                let mut object_79 = array_77.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_79, item_78)?;
                object_79.finish();
            }
        }
        array_77.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_80) = &input.resource_name {
        object.key("ResourceName").string(var_80.as_str());
    }
    if let Some(var_81) = &input.tag_keys {
        let mut array_82 = object.key("TagKeys").start_array();
        for item_83 in var_81 {
            {
                array_82.value().string(item_83.as_str());
            }
        }
        array_82.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateClusterInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_84) = &input.cluster_name {
        object.key("ClusterName").string(var_84.as_str());
    }
    if let Some(var_85) = &input.description {
        object.key("Description").string(var_85.as_str());
    }
    if let Some(var_86) = &input.preferred_maintenance_window {
        object
            .key("PreferredMaintenanceWindow")
            .string(var_86.as_str());
    }
    if let Some(var_87) = &input.notification_topic_arn {
        object.key("NotificationTopicArn").string(var_87.as_str());
    }
    if let Some(var_88) = &input.notification_topic_status {
        object
            .key("NotificationTopicStatus")
            .string(var_88.as_str());
    }
    if let Some(var_89) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_89.as_str());
    }
    if let Some(var_90) = &input.security_group_ids {
        let mut array_91 = object.key("SecurityGroupIds").start_array();
        for item_92 in var_90 {
            {
                array_91.value().string(item_92.as_str());
            }
        }
        array_91.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_parameter_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateParameterGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_93) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_93.as_str());
    }
    if let Some(var_94) = &input.parameter_name_values {
        let mut array_95 = object.key("ParameterNameValues").start_array();
        for item_96 in var_94 {
            {
                let mut object_97 = array_95.value().start_object();
                crate::json_ser::serialize_structure_crate_model_parameter_name_value(
                    &mut object_97,
                    item_96,
                )?;
                object_97.finish();
            }
        }
        array_95.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_subnet_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSubnetGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_98) = &input.subnet_group_name {
        object.key("SubnetGroupName").string(var_98.as_str());
    }
    if let Some(var_99) = &input.description {
        object.key("Description").string(var_99.as_str());
    }
    if let Some(var_100) = &input.subnet_ids {
        let mut array_101 = object.key("SubnetIds").start_array();
        for item_102 in var_100 {
            {
                array_101.value().string(item_102.as_str());
            }
        }
        array_101.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_103) = &input.key {
        object.key("Key").string(var_103.as_str());
    }
    if let Some(var_104) = &input.value {
        object.key("Value").string(var_104.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sse_specification(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SseSpecification,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_105) = &input.enabled {
        object.key("Enabled").boolean(*var_105);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_parameter_name_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ParameterNameValue,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_106) = &input.parameter_name {
        object.key("ParameterName").string(var_106.as_str());
    }
    if let Some(var_107) = &input.parameter_value {
        object.key("ParameterValue").string(var_107.as_str());
    }
    Ok(())
}
