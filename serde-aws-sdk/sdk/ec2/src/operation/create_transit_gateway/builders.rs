// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_transit_gateway::_create_transit_gateway_output::CreateTransitGatewayOutputBuilder;

pub use crate::operation::create_transit_gateway::_create_transit_gateway_input::CreateTransitGatewayInputBuilder;

/// Fluent builder constructing a request to `CreateTransitGateway`.
///
/// <p>Creates a transit gateway.</p>
/// <p>You can use a transit gateway to interconnect your virtual private clouds (VPC) and on-premises networks. After the transit gateway enters the <code>available</code> state, you can attach your VPCs and VPN connections to the transit gateway.</p>
/// <p>To attach your VPCs, use <code>CreateTransitGatewayVpcAttachment</code>.</p>
/// <p>To attach a VPN connection, use <code>CreateCustomerGateway</code> to create a customer gateway and specify the ID of the customer gateway and the ID of the transit gateway in a call to <code>CreateVpnConnection</code>.</p>
/// <p>When you create a transit gateway, we create a default transit gateway route table and use it as the default association route table and the default propagation route table. You can use <code>CreateTransitGatewayRouteTable</code> to create additional transit gateway route tables. If you disable automatic route propagation, we do not create a default transit gateway route table. You can use <code>EnableTransitGatewayRouteTablePropagation</code> to propagate routes from a resource attachment to a transit gateway route table. If you disable automatic associations, you can use <code>AssociateTransitGatewayRouteTable</code> to associate a resource attachment with a transit gateway route table.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateTransitGatewayFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_transit_gateway::builders::CreateTransitGatewayInputBuilder,
}
impl CreateTransitGatewayFluentBuilder {
    /// Creates a new `CreateTransitGateway`.
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
            crate::operation::create_transit_gateway::CreateTransitGateway,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_transit_gateway::CreateTransitGatewayError,
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
        crate::operation::create_transit_gateway::CreateTransitGatewayOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_transit_gateway::CreateTransitGatewayError,
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
    ///     let deserialized_parameters: crate::operation::create_transit_gateway::builders::CreateTransitGatewayInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_transit_gateway().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_transit_gateway::builders::CreateTransitGatewayInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>A description of the transit gateway.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the transit gateway.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The transit gateway options.</p>
    pub fn options(mut self, input: crate::types::TransitGatewayRequestOptions) -> Self {
        self.inner = self.inner.options(input);
        self
    }
    /// <p>The transit gateway options.</p>
    pub fn set_options(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayRequestOptions>,
    ) -> Self {
        self.inner = self.inner.set_options(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the transit gateway.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the transit gateway.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
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
