// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>In the response to a <code>ListHostedZonesByVPC</code> request, the <code>HostedZoneSummaries</code> element contains one <code>HostedZoneSummary</code> element for each hosted zone that the specified Amazon VPC is associated with. Each <code>HostedZoneSummary</code> element contains the hosted zone name and ID, and information about who owns the hosted zone.</p>
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
pub struct HostedZoneSummary {
    /// <p>The Route 53 hosted zone ID of a private hosted zone that the specified VPC is associated with.</p>
    #[doc(hidden)]
    pub hosted_zone_id: std::option::Option<std::string::String>,
    /// <p>The name of the private hosted zone, such as <code>example.com</code>.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The owner of a private hosted zone that the specified VPC is associated with. The owner can be either an Amazon Web Services account or an Amazon Web Services service.</p>
    #[doc(hidden)]
    pub owner: std::option::Option<crate::types::HostedZoneOwner>,
}
impl HostedZoneSummary {
    /// <p>The Route 53 hosted zone ID of a private hosted zone that the specified VPC is associated with.</p>
    pub fn hosted_zone_id(&self) -> std::option::Option<&str> {
        self.hosted_zone_id.as_deref()
    }
    /// <p>The name of the private hosted zone, such as <code>example.com</code>.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The owner of a private hosted zone that the specified VPC is associated with. The owner can be either an Amazon Web Services account or an Amazon Web Services service.</p>
    pub fn owner(&self) -> std::option::Option<&crate::types::HostedZoneOwner> {
        self.owner.as_ref()
    }
}
impl HostedZoneSummary {
    /// Creates a new builder-style object to manufacture [`HostedZoneSummary`](crate::types::HostedZoneSummary).
    pub fn builder() -> crate::types::builders::HostedZoneSummaryBuilder {
        crate::types::builders::HostedZoneSummaryBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::HostedZoneSummary;
/// A builder for [`HostedZoneSummary`](crate::types::HostedZoneSummary).
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
pub struct HostedZoneSummaryBuilder {
    pub(crate) hosted_zone_id: std::option::Option<std::string::String>,
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) owner: std::option::Option<crate::types::HostedZoneOwner>,
}
impl HostedZoneSummaryBuilder {
    /// <p>The Route 53 hosted zone ID of a private hosted zone that the specified VPC is associated with.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.hosted_zone_id = Some(input.into());
        self
    }
    /// <p>The Route 53 hosted zone ID of a private hosted zone that the specified VPC is associated with.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.hosted_zone_id = input;
        self
    }
    /// <p>The name of the private hosted zone, such as <code>example.com</code>.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the private hosted zone, such as <code>example.com</code>.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The owner of a private hosted zone that the specified VPC is associated with. The owner can be either an Amazon Web Services account or an Amazon Web Services service.</p>
    pub fn owner(mut self, input: crate::types::HostedZoneOwner) -> Self {
        self.owner = Some(input);
        self
    }
    /// <p>The owner of a private hosted zone that the specified VPC is associated with. The owner can be either an Amazon Web Services account or an Amazon Web Services service.</p>
    pub fn set_owner(mut self, input: std::option::Option<crate::types::HostedZoneOwner>) -> Self {
        self.owner = input;
        self
    }
    /// Consumes the builder and constructs a [`HostedZoneSummary`](crate::types::HostedZoneSummary).
    pub fn build(self) -> crate::types::HostedZoneSummary {
        crate::types::HostedZoneSummary {
            hosted_zone_id: self.hosted_zone_id,
            name: self.name,
            owner: self.owner,
        }
    }
}
