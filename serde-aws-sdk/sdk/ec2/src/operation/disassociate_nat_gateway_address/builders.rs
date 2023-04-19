// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_nat_gateway_address::_disassociate_nat_gateway_address_output::DisassociateNatGatewayAddressOutputBuilder;

pub use crate::operation::disassociate_nat_gateway_address::_disassociate_nat_gateway_address_input::DisassociateNatGatewayAddressInputBuilder;

/// Fluent builder constructing a request to `DisassociateNatGatewayAddress`.
///
/// <p>Disassociates secondary Elastic IP addresses (EIPs) from a public NAT gateway. You cannot disassociate your primary EIP. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html#nat-gateway-edit-secondary">Edit secondary IP address associations</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
/// <p>While disassociating is in progress, you cannot associate/disassociate additional EIPs while the connections are being drained. You are, however, allowed to delete the NAT gateway.</p>
/// <p>An EIP will only be released at the end of MaxDrainDurationSeconds. The EIPs stay associated and support the existing connections but do not support any new connections (new connections are distributed across the remaining associated EIPs). As the existing connections drain out, the EIPs (and the corresponding private IPs mapped to them) get released.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateNatGatewayAddressFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::disassociate_nat_gateway_address::builders::DisassociateNatGatewayAddressInputBuilder
            }
impl DisassociateNatGatewayAddressFluentBuilder {
    /// Creates a new `DisassociateNatGatewayAddress`.
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
            crate::operation::disassociate_nat_gateway_address::DisassociateNatGatewayAddress,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::disassociate_nat_gateway_address::DisassociateNatGatewayAddressError,
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
        crate::operation::disassociate_nat_gateway_address::DisassociateNatGatewayAddressOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::disassociate_nat_gateway_address::DisassociateNatGatewayAddressError,
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
    ///     let deserialized_parameters: crate::operation::disassociate_nat_gateway_address::builders::DisassociateNatGatewayAddressInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.disassociate_nat_gateway_address().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::disassociate_nat_gateway_address::builders::DisassociateNatGatewayAddressInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The NAT gateway ID.</p>
    pub fn nat_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.nat_gateway_id(input.into());
        self
    }
    /// <p>The NAT gateway ID.</p>
    pub fn set_nat_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_nat_gateway_id(input);
        self
    }
    /// Appends an item to `AssociationIds`.
    ///
    /// To override the contents of this collection use [`set_association_ids`](Self::set_association_ids).
    ///
    /// <p>The association IDs of EIPs that have been associated with the NAT gateway.</p>
    pub fn association_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.association_ids(input.into());
        self
    }
    /// <p>The association IDs of EIPs that have been associated with the NAT gateway.</p>
    pub fn set_association_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_association_ids(input);
        self
    }
    /// <p>The maximum amount of time to wait (in seconds) before forcibly releasing the IP addresses if connections are still in progress. Default value is 350 seconds.</p>
    pub fn max_drain_duration_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.max_drain_duration_seconds(input);
        self
    }
    /// <p>The maximum amount of time to wait (in seconds) before forcibly releasing the IP addresses if connections are still in progress. Default value is 350 seconds.</p>
    pub fn set_max_drain_duration_seconds(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_drain_duration_seconds(input);
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
