// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the DNS options for an endpoint.</p>
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
pub struct DnsOptions {
    /// <p>The DNS records created for the endpoint.</p>
    #[doc(hidden)]
    pub dns_record_ip_type: std::option::Option<crate::types::DnsRecordIpType>,
    /// <p>Indicates whether to enable private DNS only for inbound endpoints.</p>
    #[doc(hidden)]
    pub private_dns_only_for_inbound_resolver_endpoint: std::option::Option<bool>,
}
impl DnsOptions {
    /// <p>The DNS records created for the endpoint.</p>
    pub fn dns_record_ip_type(&self) -> std::option::Option<&crate::types::DnsRecordIpType> {
        self.dns_record_ip_type.as_ref()
    }
    /// <p>Indicates whether to enable private DNS only for inbound endpoints.</p>
    pub fn private_dns_only_for_inbound_resolver_endpoint(&self) -> std::option::Option<bool> {
        self.private_dns_only_for_inbound_resolver_endpoint
    }
}
impl DnsOptions {
    /// Creates a new builder-style object to manufacture [`DnsOptions`](crate::types::DnsOptions).
    pub fn builder() -> crate::types::builders::DnsOptionsBuilder {
        crate::types::builders::DnsOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::DnsOptions;
/// A builder for [`DnsOptions`](crate::types::DnsOptions).
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
pub struct DnsOptionsBuilder {
    pub(crate) dns_record_ip_type: std::option::Option<crate::types::DnsRecordIpType>,
    pub(crate) private_dns_only_for_inbound_resolver_endpoint: std::option::Option<bool>,
}
impl DnsOptionsBuilder {
    /// <p>The DNS records created for the endpoint.</p>
    pub fn dns_record_ip_type(mut self, input: crate::types::DnsRecordIpType) -> Self {
        self.dns_record_ip_type = Some(input);
        self
    }
    /// <p>The DNS records created for the endpoint.</p>
    pub fn set_dns_record_ip_type(
        mut self,
        input: std::option::Option<crate::types::DnsRecordIpType>,
    ) -> Self {
        self.dns_record_ip_type = input;
        self
    }
    /// <p>Indicates whether to enable private DNS only for inbound endpoints.</p>
    pub fn private_dns_only_for_inbound_resolver_endpoint(mut self, input: bool) -> Self {
        self.private_dns_only_for_inbound_resolver_endpoint = Some(input);
        self
    }
    /// <p>Indicates whether to enable private DNS only for inbound endpoints.</p>
    pub fn set_private_dns_only_for_inbound_resolver_endpoint(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.private_dns_only_for_inbound_resolver_endpoint = input;
        self
    }
    /// Consumes the builder and constructs a [`DnsOptions`](crate::types::DnsOptions).
    pub fn build(self) -> crate::types::DnsOptions {
        crate::types::DnsOptions {
            dns_record_ip_type: self.dns_record_ip_type,
            private_dns_only_for_inbound_resolver_endpoint: self
                .private_dns_only_for_inbound_resolver_endpoint,
        }
    }
}
