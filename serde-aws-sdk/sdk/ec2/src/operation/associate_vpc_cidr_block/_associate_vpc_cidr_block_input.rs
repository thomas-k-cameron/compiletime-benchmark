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
pub struct AssociateVpcCidrBlockInput {
    /// <p>Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IPv6 addresses, or the size of the CIDR block.</p>
    #[doc(hidden)]
    pub amazon_provided_ipv6_cidr_block: std::option::Option<bool>,
    /// <p>An IPv4 CIDR block to associate with the VPC.</p>
    #[doc(hidden)]
    pub cidr_block: std::option::Option<std::string::String>,
    /// <p>The ID of the VPC.</p>
    #[doc(hidden)]
    pub vpc_id: std::option::Option<std::string::String>,
    /// <p>The name of the location from which we advertise the IPV6 CIDR block. Use this parameter to limit the CIDR block to this location.</p>
    /// <p> You must set <code>AmazonProvidedIpv6CidrBlock</code> to <code>true</code> to use this parameter.</p>
    /// <p> You can have one IPv6 CIDR block association per network border group.</p>
    #[doc(hidden)]
    pub ipv6_cidr_block_network_border_group: std::option::Option<std::string::String>,
    /// <p>The ID of an IPv6 address pool from which to allocate the IPv6 CIDR block.</p>
    #[doc(hidden)]
    pub ipv6_pool: std::option::Option<std::string::String>,
    /// <p>An IPv6 CIDR block from the IPv6 address pool. You must also specify <code>Ipv6Pool</code> in the request.</p>
    /// <p>To let Amazon choose the IPv6 CIDR block for you, omit this parameter.</p>
    #[doc(hidden)]
    pub ipv6_cidr_block: std::option::Option<std::string::String>,
    /// <p>Associate a CIDR allocated from an IPv4 IPAM pool to a VPC. For more information about Amazon VPC IP Address Manager (IPAM), see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    #[doc(hidden)]
    pub ipv4_ipam_pool_id: std::option::Option<std::string::String>,
    /// <p>The netmask length of the IPv4 CIDR you would like to associate from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>. </p>
    #[doc(hidden)]
    pub ipv4_netmask_length: std::option::Option<i32>,
    /// <p>Associates a CIDR allocated from an IPv6 IPAM pool to a VPC. For more information about Amazon VPC IP Address Manager (IPAM), see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    #[doc(hidden)]
    pub ipv6_ipam_pool_id: std::option::Option<std::string::String>,
    /// <p>The netmask length of the IPv6 CIDR you would like to associate from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>. </p>
    #[doc(hidden)]
    pub ipv6_netmask_length: std::option::Option<i32>,
}
impl AssociateVpcCidrBlockInput {
    /// <p>Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IPv6 addresses, or the size of the CIDR block.</p>
    pub fn amazon_provided_ipv6_cidr_block(&self) -> std::option::Option<bool> {
        self.amazon_provided_ipv6_cidr_block
    }
    /// <p>An IPv4 CIDR block to associate with the VPC.</p>
    pub fn cidr_block(&self) -> std::option::Option<&str> {
        self.cidr_block.as_deref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The name of the location from which we advertise the IPV6 CIDR block. Use this parameter to limit the CIDR block to this location.</p>
    /// <p> You must set <code>AmazonProvidedIpv6CidrBlock</code> to <code>true</code> to use this parameter.</p>
    /// <p> You can have one IPv6 CIDR block association per network border group.</p>
    pub fn ipv6_cidr_block_network_border_group(&self) -> std::option::Option<&str> {
        self.ipv6_cidr_block_network_border_group.as_deref()
    }
    /// <p>The ID of an IPv6 address pool from which to allocate the IPv6 CIDR block.</p>
    pub fn ipv6_pool(&self) -> std::option::Option<&str> {
        self.ipv6_pool.as_deref()
    }
    /// <p>An IPv6 CIDR block from the IPv6 address pool. You must also specify <code>Ipv6Pool</code> in the request.</p>
    /// <p>To let Amazon choose the IPv6 CIDR block for you, omit this parameter.</p>
    pub fn ipv6_cidr_block(&self) -> std::option::Option<&str> {
        self.ipv6_cidr_block.as_deref()
    }
    /// <p>Associate a CIDR allocated from an IPv4 IPAM pool to a VPC. For more information about Amazon VPC IP Address Manager (IPAM), see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    pub fn ipv4_ipam_pool_id(&self) -> std::option::Option<&str> {
        self.ipv4_ipam_pool_id.as_deref()
    }
    /// <p>The netmask length of the IPv4 CIDR you would like to associate from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>. </p>
    pub fn ipv4_netmask_length(&self) -> std::option::Option<i32> {
        self.ipv4_netmask_length
    }
    /// <p>Associates a CIDR allocated from an IPv6 IPAM pool to a VPC. For more information about Amazon VPC IP Address Manager (IPAM), see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    pub fn ipv6_ipam_pool_id(&self) -> std::option::Option<&str> {
        self.ipv6_ipam_pool_id.as_deref()
    }
    /// <p>The netmask length of the IPv6 CIDR you would like to associate from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>. </p>
    pub fn ipv6_netmask_length(&self) -> std::option::Option<i32> {
        self.ipv6_netmask_length
    }
}
impl AssociateVpcCidrBlockInput {
    /// Creates a new builder-style object to manufacture [`AssociateVpcCidrBlockInput`](crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockInput).
    pub fn builder(
    ) -> crate::operation::associate_vpc_cidr_block::builders::AssociateVpcCidrBlockInputBuilder
    {
        crate::operation::associate_vpc_cidr_block::builders::AssociateVpcCidrBlockInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockInput;
/// A builder for [`AssociateVpcCidrBlockInput`](crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockInput).
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
pub struct AssociateVpcCidrBlockInputBuilder {
    pub(crate) amazon_provided_ipv6_cidr_block: std::option::Option<bool>,
    pub(crate) cidr_block: std::option::Option<std::string::String>,
    pub(crate) vpc_id: std::option::Option<std::string::String>,
    pub(crate) ipv6_cidr_block_network_border_group: std::option::Option<std::string::String>,
    pub(crate) ipv6_pool: std::option::Option<std::string::String>,
    pub(crate) ipv6_cidr_block: std::option::Option<std::string::String>,
    pub(crate) ipv4_ipam_pool_id: std::option::Option<std::string::String>,
    pub(crate) ipv4_netmask_length: std::option::Option<i32>,
    pub(crate) ipv6_ipam_pool_id: std::option::Option<std::string::String>,
    pub(crate) ipv6_netmask_length: std::option::Option<i32>,
}
impl AssociateVpcCidrBlockInputBuilder {
    /// <p>Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IPv6 addresses, or the size of the CIDR block.</p>
    pub fn amazon_provided_ipv6_cidr_block(mut self, input: bool) -> Self {
        self.amazon_provided_ipv6_cidr_block = Some(input);
        self
    }
    /// <p>Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IPv6 addresses, or the size of the CIDR block.</p>
    pub fn set_amazon_provided_ipv6_cidr_block(mut self, input: std::option::Option<bool>) -> Self {
        self.amazon_provided_ipv6_cidr_block = input;
        self
    }
    /// <p>An IPv4 CIDR block to associate with the VPC.</p>
    pub fn cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.cidr_block = Some(input.into());
        self
    }
    /// <p>An IPv4 CIDR block to associate with the VPC.</p>
    pub fn set_cidr_block(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cidr_block = input;
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
    /// <p>The name of the location from which we advertise the IPV6 CIDR block. Use this parameter to limit the CIDR block to this location.</p>
    /// <p> You must set <code>AmazonProvidedIpv6CidrBlock</code> to <code>true</code> to use this parameter.</p>
    /// <p> You can have one IPv6 CIDR block association per network border group.</p>
    pub fn ipv6_cidr_block_network_border_group(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.ipv6_cidr_block_network_border_group = Some(input.into());
        self
    }
    /// <p>The name of the location from which we advertise the IPV6 CIDR block. Use this parameter to limit the CIDR block to this location.</p>
    /// <p> You must set <code>AmazonProvidedIpv6CidrBlock</code> to <code>true</code> to use this parameter.</p>
    /// <p> You can have one IPv6 CIDR block association per network border group.</p>
    pub fn set_ipv6_cidr_block_network_border_group(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.ipv6_cidr_block_network_border_group = input;
        self
    }
    /// <p>The ID of an IPv6 address pool from which to allocate the IPv6 CIDR block.</p>
    pub fn ipv6_pool(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipv6_pool = Some(input.into());
        self
    }
    /// <p>The ID of an IPv6 address pool from which to allocate the IPv6 CIDR block.</p>
    pub fn set_ipv6_pool(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ipv6_pool = input;
        self
    }
    /// <p>An IPv6 CIDR block from the IPv6 address pool. You must also specify <code>Ipv6Pool</code> in the request.</p>
    /// <p>To let Amazon choose the IPv6 CIDR block for you, omit this parameter.</p>
    pub fn ipv6_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipv6_cidr_block = Some(input.into());
        self
    }
    /// <p>An IPv6 CIDR block from the IPv6 address pool. You must also specify <code>Ipv6Pool</code> in the request.</p>
    /// <p>To let Amazon choose the IPv6 CIDR block for you, omit this parameter.</p>
    pub fn set_ipv6_cidr_block(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ipv6_cidr_block = input;
        self
    }
    /// <p>Associate a CIDR allocated from an IPv4 IPAM pool to a VPC. For more information about Amazon VPC IP Address Manager (IPAM), see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    pub fn ipv4_ipam_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipv4_ipam_pool_id = Some(input.into());
        self
    }
    /// <p>Associate a CIDR allocated from an IPv4 IPAM pool to a VPC. For more information about Amazon VPC IP Address Manager (IPAM), see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    pub fn set_ipv4_ipam_pool_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.ipv4_ipam_pool_id = input;
        self
    }
    /// <p>The netmask length of the IPv4 CIDR you would like to associate from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>. </p>
    pub fn ipv4_netmask_length(mut self, input: i32) -> Self {
        self.ipv4_netmask_length = Some(input);
        self
    }
    /// <p>The netmask length of the IPv4 CIDR you would like to associate from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>. </p>
    pub fn set_ipv4_netmask_length(mut self, input: std::option::Option<i32>) -> Self {
        self.ipv4_netmask_length = input;
        self
    }
    /// <p>Associates a CIDR allocated from an IPv6 IPAM pool to a VPC. For more information about Amazon VPC IP Address Manager (IPAM), see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    pub fn ipv6_ipam_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipv6_ipam_pool_id = Some(input.into());
        self
    }
    /// <p>Associates a CIDR allocated from an IPv6 IPAM pool to a VPC. For more information about Amazon VPC IP Address Manager (IPAM), see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    pub fn set_ipv6_ipam_pool_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.ipv6_ipam_pool_id = input;
        self
    }
    /// <p>The netmask length of the IPv6 CIDR you would like to associate from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>. </p>
    pub fn ipv6_netmask_length(mut self, input: i32) -> Self {
        self.ipv6_netmask_length = Some(input);
        self
    }
    /// <p>The netmask length of the IPv6 CIDR you would like to associate from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>. </p>
    pub fn set_ipv6_netmask_length(mut self, input: std::option::Option<i32>) -> Self {
        self.ipv6_netmask_length = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateVpcCidrBlockInput`](crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::associate_vpc_cidr_block::AssociateVpcCidrBlockInput {
                amazon_provided_ipv6_cidr_block: self.amazon_provided_ipv6_cidr_block,
                cidr_block: self.cidr_block,
                vpc_id: self.vpc_id,
                ipv6_cidr_block_network_border_group: self.ipv6_cidr_block_network_border_group,
                ipv6_pool: self.ipv6_pool,
                ipv6_cidr_block: self.ipv6_cidr_block,
                ipv4_ipam_pool_id: self.ipv4_ipam_pool_id,
                ipv4_netmask_length: self.ipv4_netmask_length,
                ipv6_ipam_pool_id: self.ipv6_ipam_pool_id,
                ipv6_netmask_length: self.ipv6_netmask_length,
            },
        )
    }
}
