// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_scheduling_policies::_describe_scheduling_policies_output::DescribeSchedulingPoliciesOutputBuilder;

pub use crate::operation::describe_scheduling_policies::_describe_scheduling_policies_input::DescribeSchedulingPoliciesInputBuilder;

/// Fluent builder constructing a request to `DescribeSchedulingPolicies`.
///
/// <p>Describes one or more of your scheduling policies.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSchedulingPoliciesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_scheduling_policies::builders::DescribeSchedulingPoliciesInputBuilder
            }
impl DescribeSchedulingPoliciesFluentBuilder {
    /// Creates a new `DescribeSchedulingPolicies`.
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
            crate::operation::describe_scheduling_policies::DescribeSchedulingPolicies,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_scheduling_policies::DescribeSchedulingPoliciesError,
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
        crate::operation::describe_scheduling_policies::DescribeSchedulingPoliciesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_scheduling_policies::DescribeSchedulingPoliciesError,
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
    ///     let deserialized_parameters: crate::operation::describe_scheduling_policies::builders::DescribeSchedulingPoliciesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_scheduling_policies().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_scheduling_policies::builders::DescribeSchedulingPoliciesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Appends an item to `arns`.
    ///
    /// To override the contents of this collection use [`set_arns`](Self::set_arns).
    ///
    /// <p>A list of up to 100 scheduling policy Amazon Resource Name (ARN) entries.</p>
    pub fn arns(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.arns(input.into());
        self
    }
    /// <p>A list of up to 100 scheduling policy Amazon Resource Name (ARN) entries.</p>
    pub fn set_arns(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_arns(input);
        self
    }
}
