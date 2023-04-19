// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct AssociateAddressInput {
    /// <p>[EC2-VPC] The allocation ID. This is required for EC2-VPC.</p>
    #[doc(hidden)]
    pub allocation_id: std::option::Option<std::string::String>,
    /// <p>The ID of the instance. The instance must have exactly one attached network interface. For EC2-VPC, you can specify either the instance ID or the network interface ID, but not both. For EC2-Classic, you must specify an instance ID and the instance must be in the running state.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>[EC2-Classic] The Elastic IP address to associate with the instance. This is required for EC2-Classic.</p>
    #[doc(hidden)]
    pub public_ip: std::option::Option<std::string::String>,
    /// <p>[EC2-VPC] For a VPC in an EC2-Classic account, specify true to allow an Elastic IP address that is already associated with an instance or network interface to be reassociated with the specified instance or network interface. Otherwise, the operation fails. In a VPC in an EC2-VPC-only account, reassociation is automatic, therefore you can specify false to ensure the operation fails if the Elastic IP address is already associated with another resource.</p>
    #[doc(hidden)]
    pub allow_reassociation: std::option::Option<bool>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>[EC2-VPC] The ID of the network interface. If the instance has more than one network interface, you must specify a network interface ID.</p>
    /// <p>For EC2-VPC, you can specify either the instance ID or the network interface ID, but not both. </p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
    /// <p>[EC2-VPC] The primary or secondary private IP address to associate with the Elastic IP address. If no private IP address is specified, the Elastic IP address is associated with the primary private IP address.</p>
    #[doc(hidden)]
    pub private_ip_address: std::option::Option<std::string::String>,
}
impl AssociateAddressInput {
    /// <p>[EC2-VPC] The allocation ID. This is required for EC2-VPC.</p>
    pub fn allocation_id(&self) -> std::option::Option<&str> {
        self.allocation_id.as_deref()
    }
    /// <p>The ID of the instance. The instance must have exactly one attached network interface. For EC2-VPC, you can specify either the instance ID or the network interface ID, but not both. For EC2-Classic, you must specify an instance ID and the instance must be in the running state.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>[EC2-Classic] The Elastic IP address to associate with the instance. This is required for EC2-Classic.</p>
    pub fn public_ip(&self) -> std::option::Option<&str> {
        self.public_ip.as_deref()
    }
    /// <p>[EC2-VPC] For a VPC in an EC2-Classic account, specify true to allow an Elastic IP address that is already associated with an instance or network interface to be reassociated with the specified instance or network interface. Otherwise, the operation fails. In a VPC in an EC2-VPC-only account, reassociation is automatic, therefore you can specify false to ensure the operation fails if the Elastic IP address is already associated with another resource.</p>
    pub fn allow_reassociation(&self) -> std::option::Option<bool> {
        self.allow_reassociation
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>[EC2-VPC] The ID of the network interface. If the instance has more than one network interface, you must specify a network interface ID.</p>
    /// <p>For EC2-VPC, you can specify either the instance ID or the network interface ID, but not both. </p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>[EC2-VPC] The primary or secondary private IP address to associate with the Elastic IP address. If no private IP address is specified, the Elastic IP address is associated with the primary private IP address.</p>
    pub fn private_ip_address(&self) -> std::option::Option<&str> {
        self.private_ip_address.as_deref()
    }
}
impl AssociateAddressInput {
    /// Creates a new builder-style object to manufacture [`AssociateAddressInput`](crate::operation::associate_address::AssociateAddressInput).
    pub fn builder() -> crate::operation::associate_address::builders::AssociateAddressInputBuilder
    {
        crate::operation::associate_address::builders::AssociateAddressInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::associate_address::AssociateAddressInput;
/// A builder for [`AssociateAddressInput`](crate::operation::associate_address::AssociateAddressInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct AssociateAddressInputBuilder {
    pub(crate) allocation_id: std::option::Option<std::string::String>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) public_ip: std::option::Option<std::string::String>,
    pub(crate) allow_reassociation: std::option::Option<bool>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
    pub(crate) private_ip_address: std::option::Option<std::string::String>,
}
impl AssociateAddressInputBuilder {
    /// <p>[EC2-VPC] The allocation ID. This is required for EC2-VPC.</p>
    pub fn allocation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.allocation_id = Some(input.into());
        self
    }
    /// <p>[EC2-VPC] The allocation ID. This is required for EC2-VPC.</p>
    pub fn set_allocation_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.allocation_id = input;
        self
    }
    /// <p>The ID of the instance. The instance must have exactly one attached network interface. For EC2-VPC, you can specify either the instance ID or the network interface ID, but not both. For EC2-Classic, you must specify an instance ID and the instance must be in the running state.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance. The instance must have exactly one attached network interface. For EC2-VPC, you can specify either the instance ID or the network interface ID, but not both. For EC2-Classic, you must specify an instance ID and the instance must be in the running state.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>[EC2-Classic] The Elastic IP address to associate with the instance. This is required for EC2-Classic.</p>
    pub fn public_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.public_ip = Some(input.into());
        self
    }
    /// <p>[EC2-Classic] The Elastic IP address to associate with the instance. This is required for EC2-Classic.</p>
    pub fn set_public_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.public_ip = input;
        self
    }
    /// <p>[EC2-VPC] For a VPC in an EC2-Classic account, specify true to allow an Elastic IP address that is already associated with an instance or network interface to be reassociated with the specified instance or network interface. Otherwise, the operation fails. In a VPC in an EC2-VPC-only account, reassociation is automatic, therefore you can specify false to ensure the operation fails if the Elastic IP address is already associated with another resource.</p>
    pub fn allow_reassociation(mut self, input: bool) -> Self {
        self.allow_reassociation = Some(input);
        self
    }
    /// <p>[EC2-VPC] For a VPC in an EC2-Classic account, specify true to allow an Elastic IP address that is already associated with an instance or network interface to be reassociated with the specified instance or network interface. Otherwise, the operation fails. In a VPC in an EC2-VPC-only account, reassociation is automatic, therefore you can specify false to ensure the operation fails if the Elastic IP address is already associated with another resource.</p>
    pub fn set_allow_reassociation(mut self, input: std::option::Option<bool>) -> Self {
        self.allow_reassociation = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>[EC2-VPC] The ID of the network interface. If the instance has more than one network interface, you must specify a network interface ID.</p>
    /// <p>For EC2-VPC, you can specify either the instance ID or the network interface ID, but not both. </p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_interface_id = Some(input.into());
        self
    }
    /// <p>[EC2-VPC] The ID of the network interface. If the instance has more than one network interface, you must specify a network interface ID.</p>
    /// <p>For EC2-VPC, you can specify either the instance ID or the network interface ID, but not both. </p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>[EC2-VPC] The primary or secondary private IP address to associate with the Elastic IP address. If no private IP address is specified, the Elastic IP address is associated with the primary private IP address.</p>
    pub fn private_ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.private_ip_address = Some(input.into());
        self
    }
    /// <p>[EC2-VPC] The primary or secondary private IP address to associate with the Elastic IP address. If no private IP address is specified, the Elastic IP address is associated with the primary private IP address.</p>
    pub fn set_private_ip_address(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.private_ip_address = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateAddressInput`](crate::operation::associate_address::AssociateAddressInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::associate_address::AssociateAddressInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::associate_address::AssociateAddressInput {
            allocation_id: self.allocation_id,
            instance_id: self.instance_id,
            public_ip: self.public_ip,
            allow_reassociation: self.allow_reassociation,
            dry_run: self.dry_run,
            network_interface_id: self.network_interface_id,
            private_ip_address: self.private_ip_address,
        })
    }
}
