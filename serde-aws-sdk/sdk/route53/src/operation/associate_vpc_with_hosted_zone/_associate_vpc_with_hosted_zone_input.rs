// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains information about the request to associate a VPC with a private hosted zone.</p>
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
pub struct AssociateVpcWithHostedZoneInput {
    /// <p>The ID of the private hosted zone that you want to associate an Amazon VPC with.</p>
    /// <p>Note that you can't associate a VPC with a hosted zone that doesn't have an existing VPC association.</p>
    #[doc(hidden)]
    pub hosted_zone_id: std::option::Option<std::string::String>,
    /// <p>A complex type that contains information about the VPC that you want to associate with a private hosted zone.</p>
    #[doc(hidden)]
    pub vpc: std::option::Option<crate::types::Vpc>,
    /// <p> <i>Optional:</i> A comment about the association request.</p>
    #[doc(hidden)]
    pub comment: std::option::Option<std::string::String>,
}
impl AssociateVpcWithHostedZoneInput {
    /// <p>The ID of the private hosted zone that you want to associate an Amazon VPC with.</p>
    /// <p>Note that you can't associate a VPC with a hosted zone that doesn't have an existing VPC association.</p>
    pub fn hosted_zone_id(&self) -> std::option::Option<&str> {
        self.hosted_zone_id.as_deref()
    }
    /// <p>A complex type that contains information about the VPC that you want to associate with a private hosted zone.</p>
    pub fn vpc(&self) -> std::option::Option<&crate::types::Vpc> {
        self.vpc.as_ref()
    }
    /// <p> <i>Optional:</i> A comment about the association request.</p>
    pub fn comment(&self) -> std::option::Option<&str> {
        self.comment.as_deref()
    }
}
impl AssociateVpcWithHostedZoneInput {
    /// Creates a new builder-style object to manufacture [`AssociateVpcWithHostedZoneInput`](crate::operation::associate_vpc_with_hosted_zone::AssociateVpcWithHostedZoneInput).
    pub fn builder() -> crate::operation::associate_vpc_with_hosted_zone::builders::AssociateVpcWithHostedZoneInputBuilder{
        crate::operation::associate_vpc_with_hosted_zone::builders::AssociateVpcWithHostedZoneInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::associate_vpc_with_hosted_zone::AssociateVpcWithHostedZoneInput;
/// A builder for [`AssociateVpcWithHostedZoneInput`](crate::operation::associate_vpc_with_hosted_zone::AssociateVpcWithHostedZoneInput).
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
pub struct AssociateVpcWithHostedZoneInputBuilder {
    pub(crate) hosted_zone_id: std::option::Option<std::string::String>,
    pub(crate) vpc: std::option::Option<crate::types::Vpc>,
    pub(crate) comment: std::option::Option<std::string::String>,
}
impl AssociateVpcWithHostedZoneInputBuilder {
    /// <p>The ID of the private hosted zone that you want to associate an Amazon VPC with.</p>
    /// <p>Note that you can't associate a VPC with a hosted zone that doesn't have an existing VPC association.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.hosted_zone_id = Some(input.into());
        self
    }
    /// <p>The ID of the private hosted zone that you want to associate an Amazon VPC with.</p>
    /// <p>Note that you can't associate a VPC with a hosted zone that doesn't have an existing VPC association.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.hosted_zone_id = input;
        self
    }
    /// <p>A complex type that contains information about the VPC that you want to associate with a private hosted zone.</p>
    pub fn vpc(mut self, input: crate::types::Vpc) -> Self {
        self.vpc = Some(input);
        self
    }
    /// <p>A complex type that contains information about the VPC that you want to associate with a private hosted zone.</p>
    pub fn set_vpc(mut self, input: std::option::Option<crate::types::Vpc>) -> Self {
        self.vpc = input;
        self
    }
    /// <p> <i>Optional:</i> A comment about the association request.</p>
    pub fn comment(mut self, input: impl Into<std::string::String>) -> Self {
        self.comment = Some(input.into());
        self
    }
    /// <p> <i>Optional:</i> A comment about the association request.</p>
    pub fn set_comment(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.comment = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateVpcWithHostedZoneInput`](crate::operation::associate_vpc_with_hosted_zone::AssociateVpcWithHostedZoneInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::associate_vpc_with_hosted_zone::AssociateVpcWithHostedZoneInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::associate_vpc_with_hosted_zone::AssociateVpcWithHostedZoneInput {
                hosted_zone_id: self.hosted_zone_id,
                vpc: self.vpc,
                comment: self.comment,
            },
        )
    }
}