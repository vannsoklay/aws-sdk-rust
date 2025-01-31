// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetResourcePolicyInput {
    /// <p>The Amazon Resource Name (ARN) of the function you want to retrieve the policy for. You can use either a qualified or an unqualified ARN, but the value you specify must be a complete ARN and wildcard characters are not accepted.</p>
    pub resource_arn: ::std::option::Option<::std::string::String>,
}
impl GetResourcePolicyInput {
    /// <p>The Amazon Resource Name (ARN) of the function you want to retrieve the policy for. You can use either a qualified or an unqualified ARN, but the value you specify must be a complete ARN and wildcard characters are not accepted.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
}
impl GetResourcePolicyInput {
    /// Creates a new builder-style object to manufacture [`GetResourcePolicyInput`](crate::operation::get_resource_policy::GetResourcePolicyInput).
    pub fn builder() -> crate::operation::get_resource_policy::builders::GetResourcePolicyInputBuilder {
        crate::operation::get_resource_policy::builders::GetResourcePolicyInputBuilder::default()
    }
}

/// A builder for [`GetResourcePolicyInput`](crate::operation::get_resource_policy::GetResourcePolicyInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetResourcePolicyInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
}
impl GetResourcePolicyInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the function you want to retrieve the policy for. You can use either a qualified or an unqualified ARN, but the value you specify must be a complete ARN and wildcard characters are not accepted.</p>
    /// This field is required.
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the function you want to retrieve the policy for. You can use either a qualified or an unqualified ARN, but the value you specify must be a complete ARN and wildcard characters are not accepted.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the function you want to retrieve the policy for. You can use either a qualified or an unqualified ARN, but the value you specify must be a complete ARN and wildcard characters are not accepted.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_arn
    }
    /// Consumes the builder and constructs a [`GetResourcePolicyInput`](crate::operation::get_resource_policy::GetResourcePolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_resource_policy::GetResourcePolicyInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_resource_policy::GetResourcePolicyInput {
            resource_arn: self.resource_arn,
        })
    }
}
