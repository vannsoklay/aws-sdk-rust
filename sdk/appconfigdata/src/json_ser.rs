// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_start_configuration_session_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartConfigurationSessionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.application_identifier {
        object.key("ApplicationIdentifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.configuration_profile_identifier {
        object
            .key("ConfigurationProfileIdentifier")
            .string(var_2.as_str());
    }
    if let Some(var_3) = &input.environment_identifier {
        object.key("EnvironmentIdentifier").string(var_3.as_str());
    }
    if let Some(var_4) = &input.required_minimum_poll_interval_in_seconds {
        object.key("RequiredMinimumPollIntervalInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    Ok(())
}
