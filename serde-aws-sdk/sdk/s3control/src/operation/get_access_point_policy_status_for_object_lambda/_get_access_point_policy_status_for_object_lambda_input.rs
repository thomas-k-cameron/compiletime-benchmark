// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetAccessPointPolicyStatusForObjectLambdaInput {
    /// <p>The account ID for the account that owns the specified Object Lambda Access Point.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The name of the Object Lambda Access Point.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
}
impl GetAccessPointPolicyStatusForObjectLambdaInput {
    /// <p>The account ID for the account that owns the specified Object Lambda Access Point.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The name of the Object Lambda Access Point.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl GetAccessPointPolicyStatusForObjectLambdaInput {
    /// Creates a new builder-style object to manufacture [`GetAccessPointPolicyStatusForObjectLambdaInput`](crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaInput).
    pub fn builder() -> crate::operation::get_access_point_policy_status_for_object_lambda::builders::GetAccessPointPolicyStatusForObjectLambdaInputBuilder{
        crate::operation::get_access_point_policy_status_for_object_lambda::builders::GetAccessPointPolicyStatusForObjectLambdaInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaInput;
/// A builder for [`GetAccessPointPolicyStatusForObjectLambdaInput`](crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct GetAccessPointPolicyStatusForObjectLambdaInputBuilder {
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) name: std::option::Option<std::string::String>,
}
impl GetAccessPointPolicyStatusForObjectLambdaInputBuilder {
    /// <p>The account ID for the account that owns the specified Object Lambda Access Point.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The account ID for the account that owns the specified Object Lambda Access Point.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The name of the Object Lambda Access Point.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the Object Lambda Access Point.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetAccessPointPolicyStatusForObjectLambdaInput`](crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaInput).
    pub fn build(self) -> Result<crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaInput {
                account_id: self.account_id
                ,
                name: self.name
                ,
            }
        )
    }
}
