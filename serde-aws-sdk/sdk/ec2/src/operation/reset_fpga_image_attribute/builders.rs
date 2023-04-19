// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::reset_fpga_image_attribute::_reset_fpga_image_attribute_output::ResetFpgaImageAttributeOutputBuilder;

pub use crate::operation::reset_fpga_image_attribute::_reset_fpga_image_attribute_input::ResetFpgaImageAttributeInputBuilder;

/// Fluent builder constructing a request to `ResetFpgaImageAttribute`.
///
/// <p>Resets the specified attribute of the specified Amazon FPGA Image (AFI) to its default value. You can only reset the load permission attribute.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ResetFpgaImageAttributeFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeInputBuilder,
}
impl ResetFpgaImageAttributeFluentBuilder {
    /// Creates a new `ResetFpgaImageAttribute`.
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
            crate::operation::reset_fpga_image_attribute::ResetFpgaImageAttribute,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::reset_fpga_image_attribute::ResetFpgaImageAttributeError,
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
        crate::operation::reset_fpga_image_attribute::ResetFpgaImageAttributeOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::reset_fpga_image_attribute::ResetFpgaImageAttributeError,
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
    ///     let deserialized_parameters: crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.reset_fpga_image_attribute().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeInputBuilder,
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
    /// <p>The ID of the AFI.</p>
    pub fn fpga_image_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.fpga_image_id(input.into());
        self
    }
    /// <p>The ID of the AFI.</p>
    pub fn set_fpga_image_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_fpga_image_id(input);
        self
    }
    /// <p>The attribute.</p>
    pub fn attribute(mut self, input: crate::types::ResetFpgaImageAttributeName) -> Self {
        self.inner = self.inner.attribute(input);
        self
    }
    /// <p>The attribute.</p>
    pub fn set_attribute(
        mut self,
        input: std::option::Option<crate::types::ResetFpgaImageAttributeName>,
    ) -> Self {
        self.inner = self.inner.set_attribute(input);
        self
    }
}
