// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the VPC attachment options.</p>
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
pub struct TransitGatewayVpcAttachmentOptions {
    /// <p>Indicates whether DNS support is enabled.</p>
    #[doc(hidden)]
    pub dns_support: std::option::Option<crate::types::DnsSupportValue>,
    /// <p>Indicates whether IPv6 support is disabled.</p>
    #[doc(hidden)]
    pub ipv6_support: std::option::Option<crate::types::Ipv6SupportValue>,
    /// <p>Indicates whether appliance mode support is enabled.</p>
    #[doc(hidden)]
    pub appliance_mode_support: std::option::Option<crate::types::ApplianceModeSupportValue>,
}
impl TransitGatewayVpcAttachmentOptions {
    /// <p>Indicates whether DNS support is enabled.</p>
    pub fn dns_support(&self) -> std::option::Option<&crate::types::DnsSupportValue> {
        self.dns_support.as_ref()
    }
    /// <p>Indicates whether IPv6 support is disabled.</p>
    pub fn ipv6_support(&self) -> std::option::Option<&crate::types::Ipv6SupportValue> {
        self.ipv6_support.as_ref()
    }
    /// <p>Indicates whether appliance mode support is enabled.</p>
    pub fn appliance_mode_support(
        &self,
    ) -> std::option::Option<&crate::types::ApplianceModeSupportValue> {
        self.appliance_mode_support.as_ref()
    }
}
impl TransitGatewayVpcAttachmentOptions {
    /// Creates a new builder-style object to manufacture [`TransitGatewayVpcAttachmentOptions`](crate::types::TransitGatewayVpcAttachmentOptions).
    pub fn builder() -> crate::types::builders::TransitGatewayVpcAttachmentOptionsBuilder {
        crate::types::builders::TransitGatewayVpcAttachmentOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TransitGatewayVpcAttachmentOptions;
/// A builder for [`TransitGatewayVpcAttachmentOptions`](crate::types::TransitGatewayVpcAttachmentOptions).
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
pub struct TransitGatewayVpcAttachmentOptionsBuilder {
    pub(crate) dns_support: std::option::Option<crate::types::DnsSupportValue>,
    pub(crate) ipv6_support: std::option::Option<crate::types::Ipv6SupportValue>,
    pub(crate) appliance_mode_support: std::option::Option<crate::types::ApplianceModeSupportValue>,
}
impl TransitGatewayVpcAttachmentOptionsBuilder {
    /// <p>Indicates whether DNS support is enabled.</p>
    pub fn dns_support(mut self, input: crate::types::DnsSupportValue) -> Self {
        self.dns_support = Some(input);
        self
    }
    /// <p>Indicates whether DNS support is enabled.</p>
    pub fn set_dns_support(
        mut self,
        input: std::option::Option<crate::types::DnsSupportValue>,
    ) -> Self {
        self.dns_support = input;
        self
    }
    /// <p>Indicates whether IPv6 support is disabled.</p>
    pub fn ipv6_support(mut self, input: crate::types::Ipv6SupportValue) -> Self {
        self.ipv6_support = Some(input);
        self
    }
    /// <p>Indicates whether IPv6 support is disabled.</p>
    pub fn set_ipv6_support(
        mut self,
        input: std::option::Option<crate::types::Ipv6SupportValue>,
    ) -> Self {
        self.ipv6_support = input;
        self
    }
    /// <p>Indicates whether appliance mode support is enabled.</p>
    pub fn appliance_mode_support(
        mut self,
        input: crate::types::ApplianceModeSupportValue,
    ) -> Self {
        self.appliance_mode_support = Some(input);
        self
    }
    /// <p>Indicates whether appliance mode support is enabled.</p>
    pub fn set_appliance_mode_support(
        mut self,
        input: std::option::Option<crate::types::ApplianceModeSupportValue>,
    ) -> Self {
        self.appliance_mode_support = input;
        self
    }
    /// Consumes the builder and constructs a [`TransitGatewayVpcAttachmentOptions`](crate::types::TransitGatewayVpcAttachmentOptions).
    pub fn build(self) -> crate::types::TransitGatewayVpcAttachmentOptions {
        crate::types::TransitGatewayVpcAttachmentOptions {
            dns_support: self.dns_support,
            ipv6_support: self.ipv6_support,
            appliance_mode_support: self.appliance_mode_support,
        }
    }
}
