// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.api_gateway_proxy {
        let mut object_2 = object.key("ApiGatewayProxy").start_object();
        crate::json_ser::serialize_structure_crate_model_api_gateway_proxy_input(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    if let Some(var_3) = &input.client_token {
        object.key("ClientToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.name {
        object.key("Name").string(var_4.as_str());
    }
    if let Some(var_5) = &input.proxy_type {
        object.key("ProxyType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        let mut object_7 = object.key("Tags").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    if let Some(var_10) = &input.vpc_id {
        object.key("VpcId").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_environment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.client_token {
        object.key("ClientToken").string(var_11.as_str());
    }
    if let Some(var_12) = &input.description {
        object.key("Description").string(var_12.as_str());
    }
    if let Some(var_13) = &input.name {
        object.key("Name").string(var_13.as_str());
    }
    if let Some(var_14) = &input.network_fabric_type {
        object.key("NetworkFabricType").string(var_14.as_str());
    }
    if let Some(var_15) = &input.tags {
        let mut object_16 = object.key("Tags").start_object();
        for (key_17, value_18) in var_15 {
            {
                object_16.key(key_17).string(value_18.as_str());
            }
        }
        object_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_route_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRouteInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_19) = &input.client_token {
        object.key("ClientToken").string(var_19.as_str());
    }
    if let Some(var_20) = &input.route_type {
        object.key("RouteType").string(var_20.as_str());
    }
    if let Some(var_21) = &input.service_identifier {
        object.key("ServiceIdentifier").string(var_21.as_str());
    }
    if let Some(var_22) = &input.tags {
        let mut object_23 = object.key("Tags").start_object();
        for (key_24, value_25) in var_22 {
            {
                object_23.key(key_24).string(value_25.as_str());
            }
        }
        object_23.finish();
    }
    if let Some(var_26) = &input.uri_path_route {
        let mut object_27 = object.key("UriPathRoute").start_object();
        crate::json_ser::serialize_structure_crate_model_uri_path_route_input(
            &mut object_27,
            var_26,
        )?;
        object_27.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_service_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateServiceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.client_token {
        object.key("ClientToken").string(var_28.as_str());
    }
    if let Some(var_29) = &input.description {
        object.key("Description").string(var_29.as_str());
    }
    if let Some(var_30) = &input.endpoint_type {
        object.key("EndpointType").string(var_30.as_str());
    }
    if let Some(var_31) = &input.lambda_endpoint {
        let mut object_32 = object.key("LambdaEndpoint").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_endpoint_input(
            &mut object_32,
            var_31,
        )?;
        object_32.finish();
    }
    if let Some(var_33) = &input.name {
        object.key("Name").string(var_33.as_str());
    }
    if let Some(var_34) = &input.tags {
        let mut object_35 = object.key("Tags").start_object();
        for (key_36, value_37) in var_34 {
            {
                object_35.key(key_36).string(value_37.as_str());
            }
        }
        object_35.finish();
    }
    if let Some(var_38) = &input.url_endpoint {
        let mut object_39 = object.key("UrlEndpoint").start_object();
        crate::json_ser::serialize_structure_crate_model_url_endpoint_input(
            &mut object_39,
            var_38,
        )?;
        object_39.finish();
    }
    if let Some(var_40) = &input.vpc_id {
        object.key("VpcId").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_41) = &input.policy {
        object.key("Policy").string(var_41.as_str());
    }
    if let Some(var_42) = &input.resource_arn {
        object.key("ResourceArn").string(var_42.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.tags {
        let mut object_44 = object.key("Tags").start_object();
        for (key_45, value_46) in var_43 {
            {
                object_44.key(key_45).string(value_46.as_str());
            }
        }
        object_44.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_api_gateway_proxy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ApiGatewayProxyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.endpoint_type {
        object.key("EndpointType").string(var_47.as_str());
    }
    if let Some(var_48) = &input.stage_name {
        object.key("StageName").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_uri_path_route_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UriPathRouteInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.source_path {
        object.key("SourcePath").string(var_49.as_str());
    }
    if let Some(var_50) = &input.activation_state {
        object.key("ActivationState").string(var_50.as_str());
    }
    if let Some(var_51) = &input.methods {
        let mut array_52 = object.key("Methods").start_array();
        for item_53 in var_51 {
            {
                array_52.value().string(item_53.as_str());
            }
        }
        array_52.finish();
    }
    if let Some(var_54) = &input.include_child_paths {
        object.key("IncludeChildPaths").boolean(*var_54);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_endpoint_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaEndpointInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_55) = &input.arn {
        object.key("Arn").string(var_55.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_url_endpoint_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UrlEndpointInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.url {
        object.key("Url").string(var_56.as_str());
    }
    if let Some(var_57) = &input.health_url {
        object.key("HealthUrl").string(var_57.as_str());
    }
    Ok(())
}
