// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for AttachNetworkInterface.</p>
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
pub struct AttachNetworkInterfaceInput {
    /// <p>The index of the device for the network interface attachment.</p>
    #[doc(hidden)]
    pub device_index: std::option::Option<i32>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The ID of the network interface.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
    /// <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    #[doc(hidden)]
    pub network_card_index: std::option::Option<i32>,
    /// <p>Configures ENA Express for the network interface that this action attaches to the instance.</p>
    #[doc(hidden)]
    pub ena_srd_specification: std::option::Option<crate::types::EnaSrdSpecification>,
}
impl AttachNetworkInterfaceInput {
    /// <p>The index of the device for the network interface attachment.</p>
    pub fn device_index(&self) -> std::option::Option<i32> {
        self.device_index
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    pub fn network_card_index(&self) -> std::option::Option<i32> {
        self.network_card_index
    }
    /// <p>Configures ENA Express for the network interface that this action attaches to the instance.</p>
    pub fn ena_srd_specification(&self) -> std::option::Option<&crate::types::EnaSrdSpecification> {
        self.ena_srd_specification.as_ref()
    }
}
impl AttachNetworkInterfaceInput {
    /// Creates a new builder-style object to manufacture [`AttachNetworkInterfaceInput`](crate::operation::attach_network_interface::AttachNetworkInterfaceInput).
    pub fn builder(
    ) -> crate::operation::attach_network_interface::builders::AttachNetworkInterfaceInputBuilder
    {
        crate::operation::attach_network_interface::builders::AttachNetworkInterfaceInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::attach_network_interface::AttachNetworkInterfaceInput;
/// A builder for [`AttachNetworkInterfaceInput`](crate::operation::attach_network_interface::AttachNetworkInterfaceInput).
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
pub struct AttachNetworkInterfaceInputBuilder {
    pub(crate) device_index: std::option::Option<i32>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
    pub(crate) network_card_index: std::option::Option<i32>,
    pub(crate) ena_srd_specification: std::option::Option<crate::types::EnaSrdSpecification>,
}
impl AttachNetworkInterfaceInputBuilder {
    /// <p>The index of the device for the network interface attachment.</p>
    pub fn device_index(mut self, input: i32) -> Self {
        self.device_index = Some(input);
        self
    }
    /// <p>The index of the device for the network interface attachment.</p>
    pub fn set_device_index(mut self, input: std::option::Option<i32>) -> Self {
        self.device_index = input;
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
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_interface_id = Some(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    pub fn network_card_index(mut self, input: i32) -> Self {
        self.network_card_index = Some(input);
        self
    }
    /// <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    pub fn set_network_card_index(mut self, input: std::option::Option<i32>) -> Self {
        self.network_card_index = input;
        self
    }
    /// <p>Configures ENA Express for the network interface that this action attaches to the instance.</p>
    pub fn ena_srd_specification(mut self, input: crate::types::EnaSrdSpecification) -> Self {
        self.ena_srd_specification = Some(input);
        self
    }
    /// <p>Configures ENA Express for the network interface that this action attaches to the instance.</p>
    pub fn set_ena_srd_specification(
        mut self,
        input: std::option::Option<crate::types::EnaSrdSpecification>,
    ) -> Self {
        self.ena_srd_specification = input;
        self
    }
    /// Consumes the builder and constructs a [`AttachNetworkInterfaceInput`](crate::operation::attach_network_interface::AttachNetworkInterfaceInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::attach_network_interface::AttachNetworkInterfaceInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::attach_network_interface::AttachNetworkInterfaceInput {
                device_index: self.device_index,
                dry_run: self.dry_run,
                instance_id: self.instance_id,
                network_interface_id: self.network_interface_id,
                network_card_index: self.network_card_index,
                ena_srd_specification: self.ena_srd_specification,
            },
        )
    }
}
