// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_checker_ip_ranges::_get_checker_ip_ranges_output::GetCheckerIpRangesOutputBuilder;

pub use crate::operation::get_checker_ip_ranges::_get_checker_ip_ranges_input::GetCheckerIpRangesInputBuilder;

/// Fluent builder constructing a request to `GetCheckerIpRanges`.
///
/// <p>Route 53 does not perform authorization for this API because it retrieves information that is already available to the public.</p> <important>
/// <p> <code>GetCheckerIpRanges</code> still works, but we recommend that you download ip-ranges.json, which includes IP address ranges for all Amazon Web Services services. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/route-53-ip-addresses.html">IP Address Ranges of Amazon Route 53 Servers</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetCheckerIpRangesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_checker_ip_ranges::builders::GetCheckerIpRangesInputBuilder,
}
impl GetCheckerIpRangesFluentBuilder {
    /// Creates a new `GetCheckerIpRanges`.
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
            crate::operation::get_checker_ip_ranges::GetCheckerIpRanges,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_checker_ip_ranges::GetCheckerIpRangesError,
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
        crate::operation::get_checker_ip_ranges::GetCheckerIpRangesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_checker_ip_ranges::GetCheckerIpRangesError,
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
    ///     let deserialized_parameters: crate::operation::get_checker_ip_ranges::builders::GetCheckerIpRangesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_checker_ip_ranges().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_checker_ip_ranges::builders::GetCheckerIpRangesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
}