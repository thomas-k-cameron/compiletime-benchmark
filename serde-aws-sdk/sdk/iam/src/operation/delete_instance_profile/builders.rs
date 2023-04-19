// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_instance_profile::_delete_instance_profile_output::DeleteInstanceProfileOutputBuilder;

pub use crate::operation::delete_instance_profile::_delete_instance_profile_input::DeleteInstanceProfileInputBuilder;

/// Fluent builder constructing a request to `DeleteInstanceProfile`.
///
/// <p>Deletes the specified instance profile. The instance profile must not have an associated role.</p> <important>
/// <p>Make sure that you do not have any Amazon EC2 instances running with the instance profile you are about to delete. Deleting a role or instance profile that is associated with a running instance will break any applications running on the instance.</p>
/// </important>
/// <p>For more information about instance profiles, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/AboutInstanceProfiles.html">About instance profiles</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteInstanceProfileFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_instance_profile::builders::DeleteInstanceProfileInputBuilder,
}
impl DeleteInstanceProfileFluentBuilder {
    /// Creates a new `DeleteInstanceProfile`.
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
            crate::operation::delete_instance_profile::DeleteInstanceProfile,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_instance_profile::DeleteInstanceProfileError,
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
        crate::operation::delete_instance_profile::DeleteInstanceProfileOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_instance_profile::DeleteInstanceProfileError,
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
    ///     let deserialized_parameters: crate::operation::delete_instance_profile::builders::DeleteInstanceProfileInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_instance_profile().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_instance_profile::builders::DeleteInstanceProfileInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the instance profile to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn instance_profile_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_profile_name(input.into());
        self
    }
    /// <p>The name of the instance profile to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_instance_profile_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_instance_profile_name(input);
        self
    }
}
