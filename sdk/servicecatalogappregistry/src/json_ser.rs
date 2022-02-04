// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        let mut object_5 = object.key("tags").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_attribute_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAttributeGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.attributes {
        object.key("attributes").string(var_8.as_str());
    }
    if let Some(var_9) = &input.client_token {
        object.key("clientToken").string(var_9.as_str());
    }
    if let Some(var_10) = &input.description {
        object.key("description").string(var_10.as_str());
    }
    if let Some(var_11) = &input.name {
        object.key("name").string(var_11.as_str());
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

pub fn serialize_structure_crate_input_update_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.description {
        object.key("description").string(var_20.as_str());
    }
    if let Some(var_21) = &input.name {
        object.key("name").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_attribute_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAttributeGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.attributes {
        object.key("attributes").string(var_22.as_str());
    }
    if let Some(var_23) = &input.description {
        object.key("description").string(var_23.as_str());
    }
    if let Some(var_24) = &input.name {
        object.key("name").string(var_24.as_str());
    }
    Ok(())
}
