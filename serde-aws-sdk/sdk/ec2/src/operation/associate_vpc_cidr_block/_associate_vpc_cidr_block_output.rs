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
pub struct AssociateVpcCidrBlockOutput {
    /// <p>Information about the IPv6 CIDR block association.</p>
    #[doc(hidden)]
    pub ipv6_cidr_block_association: std::option::Option<crate::types::VpcIpv6CidrBlockAssociation>,
    /// <p>Information about the IPv4 CIDR block association.</p>
    #[doc(hidden)]
    pub cidr_block_association: std::option::Option<crate::types::VpcCidrBlockAssociation>,
    /// <p>The ID of the VPC.</p>
    #[doc(hidden)]
    pub vpc_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl AssociateVpcCidrBlockOutput {
    /// <p>Information about the IPv6 CIDR block association.</p>
    pub fn ipv6_cidr_block_association(
        &self,
    ) -> std::option::Option<&crate::types::VpcIpv6CidrBlockAssociation> {
        self.ipv6_cidr_block_association.as_ref()
    }
    /// <p>Information about the IPv4 CIDR block association.</p>
    pub fn cidr_block_association(
        &self,
    ) -> std::option::Option<&crate::types::VpcCidrBlockAssociation> {
        self.cidr_block_association.as_ref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for AssociateVpcCidrBlockOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AssociateVpcCidrBlockOutput {
    /// Creates a new builder-style object to manufacture [`AssociateVpcCidrBlockOutput`](crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockOutput).
    pub fn builder(
    ) -> crate::operation::associate_vpc_cidr_block::builders::AssociateVpcCidrBlockOutputBuilder
    {
        crate::operation::associate_vpc_cidr_block::builders::AssociateVpcCidrBlockOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockOutput;
/// A builder for [`AssociateVpcCidrBlockOutput`](crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockOutput).
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
pub struct AssociateVpcCidrBlockOutputBuilder {
    pub(crate) ipv6_cidr_block_association:
        std::option::Option<crate::types::VpcIpv6CidrBlockAssociation>,
    pub(crate) cidr_block_association: std::option::Option<crate::types::VpcCidrBlockAssociation>,
    pub(crate) vpc_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl AssociateVpcCidrBlockOutputBuilder {
    /// <p>Information about the IPv6 CIDR block association.</p>
    pub fn ipv6_cidr_block_association(
        mut self,
        input: crate::types::VpcIpv6CidrBlockAssociation,
    ) -> Self {
        self.ipv6_cidr_block_association = Some(input);
        self
    }
    /// <p>Information about the IPv6 CIDR block association.</p>
    pub fn set_ipv6_cidr_block_association(
        mut self,
        input: std::option::Option<crate::types::VpcIpv6CidrBlockAssociation>,
    ) -> Self {
        self.ipv6_cidr_block_association = input;
        self
    }
    /// <p>Information about the IPv4 CIDR block association.</p>
    pub fn cidr_block_association(mut self, input: crate::types::VpcCidrBlockAssociation) -> Self {
        self.cidr_block_association = Some(input);
        self
    }
    /// <p>Information about the IPv4 CIDR block association.</p>
    pub fn set_cidr_block_association(
        mut self,
        input: std::option::Option<crate::types::VpcCidrBlockAssociation>,
    ) -> Self {
        self.cidr_block_association = input;
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpc_id = Some(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`AssociateVpcCidrBlockOutput`](crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockOutput).
    pub fn build(self) -> crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockOutput {
        crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockOutput {
            ipv6_cidr_block_association: self.ipv6_cidr_block_association,
            cidr_block_association: self.cidr_block_association,
            vpc_id: self.vpc_id,
            _request_id: self._request_id,
        }
    }
}