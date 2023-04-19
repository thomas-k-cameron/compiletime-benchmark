// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_compliance_summary_by_resource_type::_get_compliance_summary_by_resource_type_output::GetComplianceSummaryByResourceTypeOutputBuilder;

pub use crate::operation::get_compliance_summary_by_resource_type::_get_compliance_summary_by_resource_type_input::GetComplianceSummaryByResourceTypeInputBuilder;

/// Fluent builder constructing a request to `GetComplianceSummaryByResourceType`.
///
/// <p>Returns the number of resources that are compliant and the number that are noncompliant. You can specify one or more resource types to get these numbers for each resource type. The maximum number returned is 100.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetComplianceSummaryByResourceTypeFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_compliance_summary_by_resource_type::builders::GetComplianceSummaryByResourceTypeInputBuilder
            }
impl GetComplianceSummaryByResourceTypeFluentBuilder {
    /// Creates a new `GetComplianceSummaryByResourceType`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::get_compliance_summary_by_resource_type::GetComplianceSummaryByResourceType, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::get_compliance_summary_by_resource_type::GetComplianceSummaryByResourceTypeError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::get_compliance_summary_by_resource_type::GetComplianceSummaryByResourceTypeOutput, aws_smithy_http::result::SdkError<crate::operation::get_compliance_summary_by_resource_type::GetComplianceSummaryByResourceTypeError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::get_compliance_summary_by_resource_type::builders::GetComplianceSummaryByResourceTypeInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_compliance_summary_by_resource_type().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_compliance_summary_by_resource_type::builders::GetComplianceSummaryByResourceTypeInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Appends an item to `ResourceTypes`.
    ///
    /// To override the contents of this collection use [`set_resource_types`](Self::set_resource_types).
    ///
    /// <p>Specify one or more resource types to get the number of resources that are compliant and the number that are noncompliant for each resource type.</p>
    /// <p>For this request, you can specify an Amazon Web Services resource type such as <code>AWS::EC2::Instance</code>. You can specify that the resource type is an Amazon Web Services account by specifying <code>AWS::::Account</code>.</p>
    pub fn resource_types(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_types(input.into());
        self
    }
    /// <p>Specify one or more resource types to get the number of resources that are compliant and the number that are noncompliant for each resource type.</p>
    /// <p>For this request, you can specify an Amazon Web Services resource type such as <code>AWS::EC2::Instance</code>. You can specify that the resource type is an Amazon Web Services account by specifying <code>AWS::::Account</code>.</p>
    pub fn set_resource_types(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_resource_types(input);
        self
    }
}
