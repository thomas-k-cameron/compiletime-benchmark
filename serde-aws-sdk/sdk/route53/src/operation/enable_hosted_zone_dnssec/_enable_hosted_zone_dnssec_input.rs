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
pub struct EnableHostedZoneDnssecInput {
    /// <p>A unique string used to identify a hosted zone.</p>
    #[doc(hidden)]
    pub hosted_zone_id: std::option::Option<std::string::String>,
}
impl EnableHostedZoneDnssecInput {
    /// <p>A unique string used to identify a hosted zone.</p>
    pub fn hosted_zone_id(&self) -> std::option::Option<&str> {
        self.hosted_zone_id.as_deref()
    }
}
impl EnableHostedZoneDnssecInput {
    /// Creates a new builder-style object to manufacture [`EnableHostedZoneDnssecInput`](crate::operation::enable_hosted_zone_dnssec::EnableHostedZoneDnssecInput).
    pub fn builder(
    ) -> crate::operation::enable_hosted_zone_dnssec::builders::EnableHostedZoneDnssecInputBuilder
    {
        crate::operation::enable_hosted_zone_dnssec::builders::EnableHostedZoneDnssecInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::enable_hosted_zone_dnssec::EnableHostedZoneDnssecInput;
/// A builder for [`EnableHostedZoneDnssecInput`](crate::operation::enable_hosted_zone_dnssec::EnableHostedZoneDnssecInput).
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
pub struct EnableHostedZoneDnssecInputBuilder {
    pub(crate) hosted_zone_id: std::option::Option<std::string::String>,
}
impl EnableHostedZoneDnssecInputBuilder {
    /// <p>A unique string used to identify a hosted zone.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.hosted_zone_id = Some(input.into());
        self
    }
    /// <p>A unique string used to identify a hosted zone.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.hosted_zone_id = input;
        self
    }
    /// Consumes the builder and constructs a [`EnableHostedZoneDnssecInput`](crate::operation::enable_hosted_zone_dnssec::EnableHostedZoneDnssecInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::enable_hosted_zone_dnssec::EnableHostedZoneDnssecInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::enable_hosted_zone_dnssec::EnableHostedZoneDnssecInput {
                hosted_zone_id: self.hosted_zone_id,
            },
        )
    }
}
