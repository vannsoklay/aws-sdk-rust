// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_tags_to_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddTagsToResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.resource_arn {
        object.key("ResourceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tag_list {
        let mut array_3 = object.key("TagList").start_array();
        for item_4 in var_2 {
            {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_hapg_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateHapgInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.label {
        object.key("Label").string(var_6.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_hsm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateHsmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.subnet_id {
        object.key("SubnetId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.ssh_key {
        object.key("SshKey").string(var_8.as_str());
    }
    if let Some(var_9) = &input.eni_ip {
        object.key("EniIp").string(var_9.as_str());
    }
    if let Some(var_10) = &input.iam_role_arn {
        object.key("IamRoleArn").string(var_10.as_str());
    }
    if let Some(var_11) = &input.external_id {
        object.key("ExternalId").string(var_11.as_str());
    }
    if let Some(var_12) = &input.subscription_type {
        object.key("SubscriptionType").string(var_12.as_str());
    }
    if let Some(var_13) = &input.client_token {
        object.key("ClientToken").string(var_13.as_str());
    }
    if let Some(var_14) = &input.syslog_ip {
        object.key("SyslogIp").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_luna_client_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLunaClientInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_15) = &input.label {
        object.key("Label").string(var_15.as_str());
    }
    if let Some(var_16) = &input.certificate {
        object.key("Certificate").string(var_16.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_hapg_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteHapgInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.hapg_arn {
        object.key("HapgArn").string(var_17.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_hsm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteHsmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.hsm_arn {
        object.key("HsmArn").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_luna_client_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteLunaClientInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_19) = &input.client_arn {
        object.key("ClientArn").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_hapg_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeHapgInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.hapg_arn {
        object.key("HapgArn").string(var_20.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_hsm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeHsmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.hsm_arn {
        object.key("HsmArn").string(var_21.as_str());
    }
    if let Some(var_22) = &input.hsm_serial_number {
        object.key("HsmSerialNumber").string(var_22.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_luna_client_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeLunaClientInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.client_arn {
        object.key("ClientArn").string(var_23.as_str());
    }
    if let Some(var_24) = &input.certificate_fingerprint {
        object.key("CertificateFingerprint").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.client_arn {
        object.key("ClientArn").string(var_25.as_str());
    }
    if let Some(var_26) = &input.client_version {
        object.key("ClientVersion").string(var_26.as_str());
    }
    if let Some(var_27) = &input.hapg_list {
        let mut array_28 = object.key("HapgList").start_array();
        for item_29 in var_27 {
            {
                array_28.value().string(item_29.as_str());
            }
        }
        array_28.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_hapgs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListHapgsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.next_token {
        object.key("NextToken").string(var_30.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_hsms_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListHsmsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.next_token {
        object.key("NextToken").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_luna_clients_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListLunaClientsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_32) = &input.next_token {
        object.key("NextToken").string(var_32.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.resource_arn {
        object.key("ResourceArn").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_modify_hapg_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ModifyHapgInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.hapg_arn {
        object.key("HapgArn").string(var_34.as_str());
    }
    if let Some(var_35) = &input.label {
        object.key("Label").string(var_35.as_str());
    }
    if let Some(var_36) = &input.partition_serial_list {
        let mut array_37 = object.key("PartitionSerialList").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38.as_str());
            }
        }
        array_37.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_modify_hsm_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ModifyHsmInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.hsm_arn {
        object.key("HsmArn").string(var_39.as_str());
    }
    if let Some(var_40) = &input.subnet_id {
        object.key("SubnetId").string(var_40.as_str());
    }
    if let Some(var_41) = &input.eni_ip {
        object.key("EniIp").string(var_41.as_str());
    }
    if let Some(var_42) = &input.iam_role_arn {
        object.key("IamRoleArn").string(var_42.as_str());
    }
    if let Some(var_43) = &input.external_id {
        object.key("ExternalId").string(var_43.as_str());
    }
    if let Some(var_44) = &input.syslog_ip {
        object.key("SyslogIp").string(var_44.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_modify_luna_client_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ModifyLunaClientInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.client_arn {
        object.key("ClientArn").string(var_45.as_str());
    }
    if let Some(var_46) = &input.certificate {
        object.key("Certificate").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_tags_from_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveTagsFromResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.resource_arn {
        object.key("ResourceArn").string(var_47.as_str());
    }
    if let Some(var_48) = &input.tag_key_list {
        let mut array_49 = object.key("TagKeyList").start_array();
        for item_50 in var_48 {
            {
                array_49.value().string(item_50.as_str());
            }
        }
        array_49.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.key {
        object.key("Key").string(var_51.as_str());
    }
    if let Some(var_52) = &input.value {
        object.key("Value").string(var_52.as_str());
    }
    Ok(())
}
