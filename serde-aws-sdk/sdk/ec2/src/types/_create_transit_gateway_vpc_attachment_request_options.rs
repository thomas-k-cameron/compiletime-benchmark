// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the options for a VPC attachment.</p>
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
pub struct CreateTransitGatewayVpcAttachmentRequestOptions {
    /// <p>Enable or disable DNS support. The default is <code>enable</code>.</p>
    #[doc(hidden)]
    pub dns_support: std::option::Option<crate::types::DnsSupportValue>,
    /// <p>Enable or disable IPv6 support. The default is <code>disable</code>.</p>
    #[doc(hidden)]
    pub ipv6_support: std::option::Option<crate::types::Ipv6SupportValue>,
    /// <p>Enable or disable support for appliance mode. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. The default is <code>disable</code>.</p>
    #[doc(hidden)]
    pub appliance_mode_support: std::option::Option<crate::types::ApplianceModeSupportValue>,
}
impl CreateTransitGatewayVpcAttachmentRequestOptions {
    /// <p>Enable or disable DNS support. The default is <code>enable</code>.</p>
    pub fn dns_support(&self) -> std::option::Option<&crate::types::DnsSupportValue> {
        self.dns_support.as_ref()
    }
    /// <p>Enable or disable IPv6 support. The default is <code>disable</code>.</p>
    pub fn ipv6_support(&self) -> std::option::Option<&crate::types::Ipv6SupportValue> {
        self.ipv6_support.as_ref()
    }
    /// <p>Enable or disable support for appliance mode. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. The default is <code>disable</code>.</p>
    pub fn appliance_mode_support(
        &self,
    ) -> std::option::Option<&crate::types::ApplianceModeSupportValue> {
        self.appliance_mode_support.as_ref()
    }
}
impl CreateTransitGatewayVpcAttachmentRequestOptions {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayVpcAttachmentRequestOptions`](crate::types::CreateTransitGatewayVpcAttachmentRequestOptions).
    pub fn builder(
    ) -> crate::types::builders::CreateTransitGatewayVpcAttachmentRequestOptionsBuilder {
        crate::types::builders::CreateTransitGatewayVpcAttachmentRequestOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CreateTransitGatewayVpcAttachmentRequestOptions;
/// A builder for [`CreateTransitGatewayVpcAttachmentRequestOptions`](crate::types::CreateTransitGatewayVpcAttachmentRequestOptions).
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
pub struct CreateTransitGatewayVpcAttachmentRequestOptionsBuilder {
    pub(crate) dns_support: std::option::Option<crate::types::DnsSupportValue>,
    pub(crate) ipv6_support: std::option::Option<crate::types::Ipv6SupportValue>,
    pub(crate) appliance_mode_support: std::option::Option<crate::types::ApplianceModeSupportValue>,
}
impl CreateTransitGatewayVpcAttachmentRequestOptionsBuilder {
    /// <p>Enable or disable DNS support. The default is <code>enable</code>.</p>
    pub fn dns_support(mut self, input: crate::types::DnsSupportValue) -> Self {
        self.dns_support = Some(input);
        self
    }
    /// <p>Enable or disable DNS support. The default is <code>enable</code>.</p>
    pub fn set_dns_support(
        mut self,
        input: std::option::Option<crate::types::DnsSupportValue>,
    ) -> Self {
        self.dns_support = input;
        self
    }
    /// <p>Enable or disable IPv6 support. The default is <code>disable</code>.</p>
    pub fn ipv6_support(mut self, input: crate::types::Ipv6SupportValue) -> Self {
        self.ipv6_support = Some(input);
        self
    }
    /// <p>Enable or disable IPv6 support. The default is <code>disable</code>.</p>
    pub fn set_ipv6_support(
        mut self,
        input: std::option::Option<crate::types::Ipv6SupportValue>,
    ) -> Self {
        self.ipv6_support = input;
        self
    }
    /// <p>Enable or disable support for appliance mode. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. The default is <code>disable</code>.</p>
    pub fn appliance_mode_support(
        mut self,
        input: crate::types::ApplianceModeSupportValue,
    ) -> Self {
        self.appliance_mode_support = Some(input);
        self
    }
    /// <p>Enable or disable support for appliance mode. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. The default is <code>disable</code>.</p>
    pub fn set_appliance_mode_support(
        mut self,
        input: std::option::Option<crate::types::ApplianceModeSupportValue>,
    ) -> Self {
        self.appliance_mode_support = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateTransitGatewayVpcAttachmentRequestOptions`](crate::types::CreateTransitGatewayVpcAttachmentRequestOptions).
    pub fn build(self) -> crate::types::CreateTransitGatewayVpcAttachmentRequestOptions {
        crate::types::CreateTransitGatewayVpcAttachmentRequestOptions {
            dns_support: self.dns_support,
            ipv6_support: self.ipv6_support,
            appliance_mode_support: self.appliance_mode_support,
        }
    }
}
