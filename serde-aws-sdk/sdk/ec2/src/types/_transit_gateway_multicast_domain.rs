// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the transit gateway multicast domain.</p>
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
pub struct TransitGatewayMulticastDomain {
    /// <p>The ID of the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub transit_gateway_multicast_domain_id: std::option::Option<std::string::String>,
    /// <p>The ID of the transit gateway.</p>
    #[doc(hidden)]
    pub transit_gateway_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub transit_gateway_multicast_domain_arn: std::option::Option<std::string::String>,
    /// <p> The ID of the Amazon Web Services account that owns the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub owner_id: std::option::Option<std::string::String>,
    /// <p>The options for the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub options: std::option::Option<crate::types::TransitGatewayMulticastDomainOptions>,
    /// <p>The state of the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::TransitGatewayMulticastDomainState>,
    /// <p>The time the transit gateway multicast domain was created.</p>
    #[doc(hidden)]
    pub creation_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The tags for the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl TransitGatewayMulticastDomain {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_multicast_domain_id.as_deref()
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_arn(&self) -> std::option::Option<&str> {
        self.transit_gateway_multicast_domain_arn.as_deref()
    }
    /// <p> The ID of the Amazon Web Services account that owns the transit gateway multicast domain.</p>
    pub fn owner_id(&self) -> std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The options for the transit gateway multicast domain.</p>
    pub fn options(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayMulticastDomainOptions> {
        self.options.as_ref()
    }
    /// <p>The state of the transit gateway multicast domain.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::TransitGatewayMulticastDomainState> {
        self.state.as_ref()
    }
    /// <p>The time the transit gateway multicast domain was created.</p>
    pub fn creation_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The tags for the transit gateway multicast domain.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl TransitGatewayMulticastDomain {
    /// Creates a new builder-style object to manufacture [`TransitGatewayMulticastDomain`](crate::types::TransitGatewayMulticastDomain).
    pub fn builder() -> crate::types::builders::TransitGatewayMulticastDomainBuilder {
        crate::types::builders::TransitGatewayMulticastDomainBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TransitGatewayMulticastDomain;
/// A builder for [`TransitGatewayMulticastDomain`](crate::types::TransitGatewayMulticastDomain).
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
pub struct TransitGatewayMulticastDomainBuilder {
    pub(crate) transit_gateway_multicast_domain_id: std::option::Option<std::string::String>,
    pub(crate) transit_gateway_id: std::option::Option<std::string::String>,
    pub(crate) transit_gateway_multicast_domain_arn: std::option::Option<std::string::String>,
    pub(crate) owner_id: std::option::Option<std::string::String>,
    pub(crate) options: std::option::Option<crate::types::TransitGatewayMulticastDomainOptions>,
    pub(crate) state: std::option::Option<crate::types::TransitGatewayMulticastDomainState>,
    pub(crate) creation_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl TransitGatewayMulticastDomainBuilder {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.transit_gateway_multicast_domain_id = Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_multicast_domain_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_multicast_domain_id = input;
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transit_gateway_id = Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn set_transit_gateway_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_arn(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.transit_gateway_multicast_domain_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_multicast_domain_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_multicast_domain_arn = input;
        self
    }
    /// <p> The ID of the Amazon Web Services account that owns the transit gateway multicast domain.</p>
    pub fn owner_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.owner_id = Some(input.into());
        self
    }
    /// <p> The ID of the Amazon Web Services account that owns the transit gateway multicast domain.</p>
    pub fn set_owner_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The options for the transit gateway multicast domain.</p>
    pub fn options(mut self, input: crate::types::TransitGatewayMulticastDomainOptions) -> Self {
        self.options = Some(input);
        self
    }
    /// <p>The options for the transit gateway multicast domain.</p>
    pub fn set_options(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayMulticastDomainOptions>,
    ) -> Self {
        self.options = input;
        self
    }
    /// <p>The state of the transit gateway multicast domain.</p>
    pub fn state(mut self, input: crate::types::TransitGatewayMulticastDomainState) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>The state of the transit gateway multicast domain.</p>
    pub fn set_state(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayMulticastDomainState>,
    ) -> Self {
        self.state = input;
        self
    }
    /// <p>The time the transit gateway multicast domain was created.</p>
    pub fn creation_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.creation_time = Some(input);
        self
    }
    /// <p>The time the transit gateway multicast domain was created.</p>
    pub fn set_creation_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the transit gateway multicast domain.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags for the transit gateway multicast domain.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`TransitGatewayMulticastDomain`](crate::types::TransitGatewayMulticastDomain).
    pub fn build(self) -> crate::types::TransitGatewayMulticastDomain {
        crate::types::TransitGatewayMulticastDomain {
            transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id,
            transit_gateway_id: self.transit_gateway_id,
            transit_gateway_multicast_domain_arn: self.transit_gateway_multicast_domain_arn,
            owner_id: self.owner_id,
            options: self.options,
            state: self.state,
            creation_time: self.creation_time,
            tags: self.tags,
        }
    }
}
