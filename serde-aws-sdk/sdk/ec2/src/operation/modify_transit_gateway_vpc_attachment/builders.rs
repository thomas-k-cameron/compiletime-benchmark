// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_transit_gateway_vpc_attachment::_modify_transit_gateway_vpc_attachment_output::ModifyTransitGatewayVpcAttachmentOutputBuilder;

pub use crate::operation::modify_transit_gateway_vpc_attachment::_modify_transit_gateway_vpc_attachment_input::ModifyTransitGatewayVpcAttachmentInputBuilder;

/// Fluent builder constructing a request to `ModifyTransitGatewayVpcAttachment`.
///
/// <p>Modifies the specified VPC attachment.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ModifyTransitGatewayVpcAttachmentFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::modify_transit_gateway_vpc_attachment::builders::ModifyTransitGatewayVpcAttachmentInputBuilder
            }
impl ModifyTransitGatewayVpcAttachmentFluentBuilder {
    /// Creates a new `ModifyTransitGatewayVpcAttachment`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::modify_transit_gateway_vpc_attachment::ModifyTransitGatewayVpcAttachment, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::modify_transit_gateway_vpc_attachment::ModifyTransitGatewayVpcAttachmentError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::modify_transit_gateway_vpc_attachment::ModifyTransitGatewayVpcAttachmentOutput, aws_smithy_http::result::SdkError<crate::operation::modify_transit_gateway_vpc_attachment::ModifyTransitGatewayVpcAttachmentError>>
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
    ///     let deserialized_parameters: crate::operation::modify_transit_gateway_vpc_attachment::builders::ModifyTransitGatewayVpcAttachmentInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.modify_transit_gateway_vpc_attachment().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::modify_transit_gateway_vpc_attachment::builders::ModifyTransitGatewayVpcAttachmentInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_attachment_id(input.into());
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_attachment_id(input);
        self
    }
    /// Appends an item to `AddSubnetIds`.
    ///
    /// To override the contents of this collection use [`set_add_subnet_ids`](Self::set_add_subnet_ids).
    ///
    /// <p>The IDs of one or more subnets to add. You can specify at most one subnet per Availability Zone.</p>
    pub fn add_subnet_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.add_subnet_ids(input.into());
        self
    }
    /// <p>The IDs of one or more subnets to add. You can specify at most one subnet per Availability Zone.</p>
    pub fn set_add_subnet_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_add_subnet_ids(input);
        self
    }
    /// Appends an item to `RemoveSubnetIds`.
    ///
    /// To override the contents of this collection use [`set_remove_subnet_ids`](Self::set_remove_subnet_ids).
    ///
    /// <p>The IDs of one or more subnets to remove.</p>
    pub fn remove_subnet_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.remove_subnet_ids(input.into());
        self
    }
    /// <p>The IDs of one or more subnets to remove.</p>
    pub fn set_remove_subnet_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_remove_subnet_ids(input);
        self
    }
    /// <p>The new VPC attachment options.</p>
    pub fn options(
        mut self,
        input: crate::types::ModifyTransitGatewayVpcAttachmentRequestOptions,
    ) -> Self {
        self.inner = self.inner.options(input);
        self
    }
    /// <p>The new VPC attachment options.</p>
    pub fn set_options(
        mut self,
        input: std::option::Option<crate::types::ModifyTransitGatewayVpcAttachmentRequestOptions>,
    ) -> Self {
        self.inner = self.inner.set_options(input);
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
