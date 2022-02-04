// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_environment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.data_bundles {
        let mut array_2 = object.key("dataBundles").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.federation_mode {
        object.key("federationMode").string(var_5.as_str());
    }
    if let Some(var_6) = &input.federation_parameters {
        let mut object_7 = object.key("federationParameters").start_object();
        crate::json_ser::serialize_structure_crate_model_federation_parameters(
            &mut object_7,
            var_6,
        )?;
        object_7.finish();
    }
    if let Some(var_8) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.name {
        object.key("name").string(var_9.as_str());
    }
    if let Some(var_10) = &input.superuser_parameters {
        let mut object_11 = object.key("superuserParameters").start_object();
        crate::json_ser::serialize_structure_crate_model_superuser_parameters(
            &mut object_11,
            var_10,
        )?;
        object_11.finish();
    }
    if let Some(var_12) = &input.tags {
        let mut object_13 = object.key("tags").start_object();
        for (key_14, value_15) in var_12 {
            {
                object_13.key(key_14).string(value_15.as_str());
            }
        }
        object_13.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_16) = &input.tags {
        let mut object_17 = object.key("tags").start_object();
        for (key_18, value_19) in var_16 {
            {
                object_17.key(key_18).string(value_19.as_str());
            }
        }
        object_17.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_environment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEnvironmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.description {
        object.key("description").string(var_20.as_str());
    }
    if let Some(var_21) = &input.federation_mode {
        object.key("federationMode").string(var_21.as_str());
    }
    if let Some(var_22) = &input.federation_parameters {
        let mut object_23 = object.key("federationParameters").start_object();
        crate::json_ser::serialize_structure_crate_model_federation_parameters(
            &mut object_23,
            var_22,
        )?;
        object_23.finish();
    }
    if let Some(var_24) = &input.name {
        object.key("name").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_federation_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FederationParameters,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.saml_metadata_document {
        object.key("samlMetadataDocument").string(var_25.as_str());
    }
    if let Some(var_26) = &input.saml_metadata_url {
        object.key("samlMetadataURL").string(var_26.as_str());
    }
    if let Some(var_27) = &input.application_call_back_url {
        object.key("applicationCallBackURL").string(var_27.as_str());
    }
    if let Some(var_28) = &input.federation_urn {
        object.key("federationURN").string(var_28.as_str());
    }
    if let Some(var_29) = &input.federation_provider_name {
        object.key("federationProviderName").string(var_29.as_str());
    }
    if let Some(var_30) = &input.attribute_map {
        let mut object_31 = object.key("attributeMap").start_object();
        for (key_32, value_33) in var_30 {
            {
                object_31.key(key_32).string(value_33.as_str());
            }
        }
        object_31.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_superuser_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SuperuserParameters,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.email_address {
        object.key("emailAddress").string(var_34.as_str());
    }
    if let Some(var_35) = &input.first_name {
        object.key("firstName").string(var_35.as_str());
    }
    if let Some(var_36) = &input.last_name {
        object.key("lastName").string(var_36.as_str());
    }
    Ok(())
}
