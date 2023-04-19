// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_role_from_instance_profile::_remove_role_from_instance_profile_output::RemoveRoleFromInstanceProfileOutputBuilder;

pub use crate::operation::remove_role_from_instance_profile::_remove_role_from_instance_profile_input::RemoveRoleFromInstanceProfileInputBuilder;

/// Fluent builder constructing a request to `RemoveRoleFromInstanceProfile`.
///
/// <p>Removes the specified IAM role from the specified EC2 instance profile.</p> <important>
/// <p>Make sure that you do not have any Amazon EC2 instances running with the role you are about to remove from the instance profile. Removing a role from an instance profile that is associated with a running instance might break any applications running on the instance.</p>
/// </important>
/// <p> For more information about IAM roles, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/WorkingWithRoles.html">Working with roles</a>. For more information about instance profiles, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/AboutInstanceProfiles.html">About instance profiles</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RemoveRoleFromInstanceProfileFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileInputBuilder
            }
impl RemoveRoleFromInstanceProfileFluentBuilder {
    /// Creates a new `RemoveRoleFromInstanceProfile`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfile,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::remove_role_from_instance_profile::RemoveRoleFromInstanceProfileError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.remove_role_from_instance_profile().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::remove_role_from_instance_profile::builders::RemoveRoleFromInstanceProfileInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the instance profile to update.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn instance_profile_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_profile_name(input.into());
        self
    }
    /// <p>The name of the instance profile to update.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_instance_profile_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_instance_profile_name(input);
        self
    }
    /// <p>The name of the role to remove.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_name(input.into());
        self
    }
    /// <p>The name of the role to remove.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_name(input);
        self
    }
}
