// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a rule associated with a transit gateway policy.</p>
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
pub struct TransitGatewayPolicyRule {
    /// <p>The source CIDR block for the transit gateway policy rule.</p>
    #[doc(hidden)]
    pub source_cidr_block: std::option::Option<std::string::String>,
    /// <p>The port range for the transit gateway policy rule. Currently this is set to * (all).</p>
    #[doc(hidden)]
    pub source_port_range: std::option::Option<std::string::String>,
    /// <p>The destination CIDR block for the transit gateway policy rule.</p>
    #[doc(hidden)]
    pub destination_cidr_block: std::option::Option<std::string::String>,
    /// <p>The port range for the transit gateway policy rule. Currently this is set to * (all).</p>
    #[doc(hidden)]
    pub destination_port_range: std::option::Option<std::string::String>,
    /// <p>The protocol used by the transit gateway policy rule.</p>
    #[doc(hidden)]
    pub protocol: std::option::Option<std::string::String>,
    /// <p>The meta data tags used for the transit gateway policy rule.</p>
    #[doc(hidden)]
    pub meta_data: std::option::Option<crate::types::TransitGatewayPolicyRuleMetaData>,
}
impl TransitGatewayPolicyRule {
    /// <p>The source CIDR block for the transit gateway policy rule.</p>
    pub fn source_cidr_block(&self) -> std::option::Option<&str> {
        self.source_cidr_block.as_deref()
    }
    /// <p>The port range for the transit gateway policy rule. Currently this is set to * (all).</p>
    pub fn source_port_range(&self) -> std::option::Option<&str> {
        self.source_port_range.as_deref()
    }
    /// <p>The destination CIDR block for the transit gateway policy rule.</p>
    pub fn destination_cidr_block(&self) -> std::option::Option<&str> {
        self.destination_cidr_block.as_deref()
    }
    /// <p>The port range for the transit gateway policy rule. Currently this is set to * (all).</p>
    pub fn destination_port_range(&self) -> std::option::Option<&str> {
        self.destination_port_range.as_deref()
    }
    /// <p>The protocol used by the transit gateway policy rule.</p>
    pub fn protocol(&self) -> std::option::Option<&str> {
        self.protocol.as_deref()
    }
    /// <p>The meta data tags used for the transit gateway policy rule.</p>
    pub fn meta_data(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayPolicyRuleMetaData> {
        self.meta_data.as_ref()
    }
}
impl TransitGatewayPolicyRule {
    /// Creates a new builder-style object to manufacture [`TransitGatewayPolicyRule`](crate::types::TransitGatewayPolicyRule).
    pub fn builder() -> crate::types::builders::TransitGatewayPolicyRuleBuilder {
        crate::types::builders::TransitGatewayPolicyRuleBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TransitGatewayPolicyRule;
/// A builder for [`TransitGatewayPolicyRule`](crate::types::TransitGatewayPolicyRule).
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
pub struct TransitGatewayPolicyRuleBuilder {
    pub(crate) source_cidr_block: std::option::Option<std::string::String>,
    pub(crate) source_port_range: std::option::Option<std::string::String>,
    pub(crate) destination_cidr_block: std::option::Option<std::string::String>,
    pub(crate) destination_port_range: std::option::Option<std::string::String>,
    pub(crate) protocol: std::option::Option<std::string::String>,
    pub(crate) meta_data: std::option::Option<crate::types::TransitGatewayPolicyRuleMetaData>,
}
impl TransitGatewayPolicyRuleBuilder {
    /// <p>The source CIDR block for the transit gateway policy rule.</p>
    pub fn source_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.source_cidr_block = Some(input.into());
        self
    }
    /// <p>The source CIDR block for the transit gateway policy rule.</p>
    pub fn set_source_cidr_block(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.source_cidr_block = input;
        self
    }
    /// <p>The port range for the transit gateway policy rule. Currently this is set to * (all).</p>
    pub fn source_port_range(mut self, input: impl Into<std::string::String>) -> Self {
        self.source_port_range = Some(input.into());
        self
    }
    /// <p>The port range for the transit gateway policy rule. Currently this is set to * (all).</p>
    pub fn set_source_port_range(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.source_port_range = input;
        self
    }
    /// <p>The destination CIDR block for the transit gateway policy rule.</p>
    pub fn destination_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination_cidr_block = Some(input.into());
        self
    }
    /// <p>The destination CIDR block for the transit gateway policy rule.</p>
    pub fn set_destination_cidr_block(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.destination_cidr_block = input;
        self
    }
    /// <p>The port range for the transit gateway policy rule. Currently this is set to * (all).</p>
    pub fn destination_port_range(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination_port_range = Some(input.into());
        self
    }
    /// <p>The port range for the transit gateway policy rule. Currently this is set to * (all).</p>
    pub fn set_destination_port_range(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.destination_port_range = input;
        self
    }
    /// <p>The protocol used by the transit gateway policy rule.</p>
    pub fn protocol(mut self, input: impl Into<std::string::String>) -> Self {
        self.protocol = Some(input.into());
        self
    }
    /// <p>The protocol used by the transit gateway policy rule.</p>
    pub fn set_protocol(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.protocol = input;
        self
    }
    /// <p>The meta data tags used for the transit gateway policy rule.</p>
    pub fn meta_data(mut self, input: crate::types::TransitGatewayPolicyRuleMetaData) -> Self {
        self.meta_data = Some(input);
        self
    }
    /// <p>The meta data tags used for the transit gateway policy rule.</p>
    pub fn set_meta_data(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayPolicyRuleMetaData>,
    ) -> Self {
        self.meta_data = input;
        self
    }
    /// Consumes the builder and constructs a [`TransitGatewayPolicyRule`](crate::types::TransitGatewayPolicyRule).
    pub fn build(self) -> crate::types::TransitGatewayPolicyRule {
        crate::types::TransitGatewayPolicyRule {
            source_cidr_block: self.source_cidr_block,
            source_port_range: self.source_port_range,
            destination_cidr_block: self.destination_cidr_block,
            destination_port_range: self.destination_port_range,
            protocol: self.protocol,
            meta_data: self.meta_data,
        }
    }
}
