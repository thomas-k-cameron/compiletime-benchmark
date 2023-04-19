// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_reachability_analyzer_organization_sharing::_enable_reachability_analyzer_organization_sharing_output::EnableReachabilityAnalyzerOrganizationSharingOutputBuilder;

pub use crate::operation::enable_reachability_analyzer_organization_sharing::_enable_reachability_analyzer_organization_sharing_input::EnableReachabilityAnalyzerOrganizationSharingInputBuilder;

/// Fluent builder constructing a request to `EnableReachabilityAnalyzerOrganizationSharing`.
///
/// <p>Establishes a trust relationship between Reachability Analyzer and Organizations. This operation must be performed by the management account for the organization.</p>
/// <p>After you establish a trust relationship, a user in the management account or a delegated administrator account can run a cross-account analysis using resources from the member accounts.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct EnableReachabilityAnalyzerOrganizationSharingFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::enable_reachability_analyzer_organization_sharing::builders::EnableReachabilityAnalyzerOrganizationSharingInputBuilder
            }
impl EnableReachabilityAnalyzerOrganizationSharingFluentBuilder {
    /// Creates a new `EnableReachabilityAnalyzerOrganizationSharing`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::enable_reachability_analyzer_organization_sharing::EnableReachabilityAnalyzerOrganizationSharing, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::enable_reachability_analyzer_organization_sharing::EnableReachabilityAnalyzerOrganizationSharingError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::enable_reachability_analyzer_organization_sharing::EnableReachabilityAnalyzerOrganizationSharingOutput, aws_smithy_http::result::SdkError<crate::operation::enable_reachability_analyzer_organization_sharing::EnableReachabilityAnalyzerOrganizationSharingError>>
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
    ///     let deserialized_parameters: crate::operation::enable_reachability_analyzer_organization_sharing::builders::EnableReachabilityAnalyzerOrganizationSharingInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.enable_reachability_analyzer_organization_sharing().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::enable_reachability_analyzer_organization_sharing::builders::EnableReachabilityAnalyzerOrganizationSharingInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
