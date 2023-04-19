// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_nat_gateway::_create_nat_gateway_output::CreateNatGatewayOutputBuilder;

pub use crate::operation::create_nat_gateway::_create_nat_gateway_input::CreateNatGatewayInputBuilder;

/// Fluent builder constructing a request to `CreateNatGateway`.
///
/// <p>Creates a NAT gateway in the specified subnet. This action creates a network interface in the specified subnet with a private IP address from the IP address range of the subnet. You can create either a public NAT gateway or a private NAT gateway.</p>
/// <p>With a public NAT gateway, internet-bound traffic from a private subnet can be routed to the NAT gateway, so that instances in a private subnet can connect to the internet.</p>
/// <p>With a private NAT gateway, private communication is routed across VPCs and on-premises networks through a transit gateway or virtual private gateway. Common use cases include running large workloads behind a small pool of allowlisted IPv4 addresses, preserving private IPv4 addresses, and communicating between overlapping networks.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html">NAT gateways</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateNatGatewayFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_nat_gateway::builders::CreateNatGatewayInputBuilder,
}
impl CreateNatGatewayFluentBuilder {
    /// Creates a new `CreateNatGateway`.
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
            crate::operation::create_nat_gateway::CreateNatGateway,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_nat_gateway::CreateNatGatewayError,
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
        crate::operation::create_nat_gateway::CreateNatGatewayOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_nat_gateway::CreateNatGatewayError,
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
    ///     let deserialized_parameters: crate::operation::create_nat_gateway::builders::CreateNatGatewayInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_nat_gateway().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_nat_gateway::builders::CreateNatGatewayInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>[Public NAT gateways only] The allocation ID of an Elastic IP address to associate with the NAT gateway. You cannot specify an Elastic IP address with a private NAT gateway. If the Elastic IP address is associated with another resource, you must first disassociate it.</p>
    pub fn allocation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.allocation_id(input.into());
        self
    }
    /// <p>[Public NAT gateways only] The allocation ID of an Elastic IP address to associate with the NAT gateway. You cannot specify an Elastic IP address with a private NAT gateway. If the Elastic IP address is associated with another resource, you must first disassociate it.</p>
    pub fn set_allocation_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_allocation_id(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    /// <p>Constraint: Maximum 64 ASCII characters.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    /// <p>Constraint: Maximum 64 ASCII characters.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
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
    /// <p>The ID of the subnet in which to create the NAT gateway.</p>
    pub fn subnet_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.subnet_id(input.into());
        self
    }
    /// <p>The ID of the subnet in which to create the NAT gateway.</p>
    pub fn set_subnet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_subnet_id(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the NAT gateway.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to assign to the NAT gateway.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>Indicates whether the NAT gateway supports public or private connectivity. The default is public connectivity.</p>
    pub fn connectivity_type(mut self, input: crate::types::ConnectivityType) -> Self {
        self.inner = self.inner.connectivity_type(input);
        self
    }
    /// <p>Indicates whether the NAT gateway supports public or private connectivity. The default is public connectivity.</p>
    pub fn set_connectivity_type(
        mut self,
        input: std::option::Option<crate::types::ConnectivityType>,
    ) -> Self {
        self.inner = self.inner.set_connectivity_type(input);
        self
    }
    /// <p>The private IPv4 address to assign to the NAT gateway. If you don't provide an address, a private IPv4 address will be automatically assigned.</p>
    pub fn private_ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.private_ip_address(input.into());
        self
    }
    /// <p>The private IPv4 address to assign to the NAT gateway. If you don't provide an address, a private IPv4 address will be automatically assigned.</p>
    pub fn set_private_ip_address(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_private_ip_address(input);
        self
    }
    /// Appends an item to `SecondaryAllocationIds`.
    ///
    /// To override the contents of this collection use [`set_secondary_allocation_ids`](Self::set_secondary_allocation_ids).
    ///
    /// <p>Secondary EIP allocation IDs. For more information about secondary addresses, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html#nat-gateway-creating">Create a NAT gateway</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    pub fn secondary_allocation_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.secondary_allocation_ids(input.into());
        self
    }
    /// <p>Secondary EIP allocation IDs. For more information about secondary addresses, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html#nat-gateway-creating">Create a NAT gateway</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    pub fn set_secondary_allocation_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_secondary_allocation_ids(input);
        self
    }
    /// Appends an item to `SecondaryPrivateIpAddresses`.
    ///
    /// To override the contents of this collection use [`set_secondary_private_ip_addresses`](Self::set_secondary_private_ip_addresses).
    ///
    /// <p>Secondary private IPv4 addresses. For more information about secondary addresses, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html#nat-gateway-creating">Create a NAT gateway</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    pub fn secondary_private_ip_addresses(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.secondary_private_ip_addresses(input.into());
        self
    }
    /// <p>Secondary private IPv4 addresses. For more information about secondary addresses, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html#nat-gateway-creating">Create a NAT gateway</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    pub fn set_secondary_private_ip_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_secondary_private_ip_addresses(input);
        self
    }
    /// <p>[Private NAT gateway only] The number of secondary private IPv4 addresses you want to assign to the NAT gateway. For more information about secondary addresses, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html#nat-gateway-creating">Create a NAT gateway</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    pub fn secondary_private_ip_address_count(mut self, input: i32) -> Self {
        self.inner = self.inner.secondary_private_ip_address_count(input);
        self
    }
    /// <p>[Private NAT gateway only] The number of secondary private IPv4 addresses you want to assign to the NAT gateway. For more information about secondary addresses, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html#nat-gateway-creating">Create a NAT gateway</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    pub fn set_secondary_private_ip_address_count(
        mut self,
        input: std::option::Option<i32>,
    ) -> Self {
        self.inner = self.inner.set_secondary_private_ip_address_count(input);
        self
    }
}