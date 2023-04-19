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
pub struct CreateVpcPeeringConnectionInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The Amazon Web Services account ID of the owner of the accepter VPC.</p>
    /// <p>Default: Your Amazon Web Services account ID</p>
    #[doc(hidden)]
    pub peer_owner_id: std::option::Option<std::string::String>,
    /// <p>The ID of the VPC with which you are creating the VPC peering connection. You must specify this parameter in the request.</p>
    #[doc(hidden)]
    pub peer_vpc_id: std::option::Option<std::string::String>,
    /// <p>The ID of the requester VPC. You must specify this parameter in the request.</p>
    #[doc(hidden)]
    pub vpc_id: std::option::Option<std::string::String>,
    /// <p>The Region code for the accepter VPC, if the accepter VPC is located in a Region other than the Region in which you make the request.</p>
    /// <p>Default: The Region in which you make the request.</p>
    #[doc(hidden)]
    pub peer_region: std::option::Option<std::string::String>,
    /// <p>The tags to assign to the peering connection.</p>
    #[doc(hidden)]
    pub tag_specifications: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
}
impl CreateVpcPeeringConnectionInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The Amazon Web Services account ID of the owner of the accepter VPC.</p>
    /// <p>Default: Your Amazon Web Services account ID</p>
    pub fn peer_owner_id(&self) -> std::option::Option<&str> {
        self.peer_owner_id.as_deref()
    }
    /// <p>The ID of the VPC with which you are creating the VPC peering connection. You must specify this parameter in the request.</p>
    pub fn peer_vpc_id(&self) -> std::option::Option<&str> {
        self.peer_vpc_id.as_deref()
    }
    /// <p>The ID of the requester VPC. You must specify this parameter in the request.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The Region code for the accepter VPC, if the accepter VPC is located in a Region other than the Region in which you make the request.</p>
    /// <p>Default: The Region in which you make the request.</p>
    pub fn peer_region(&self) -> std::option::Option<&str> {
        self.peer_region.as_deref()
    }
    /// <p>The tags to assign to the peering connection.</p>
    pub fn tag_specifications(&self) -> std::option::Option<&[crate::types::TagSpecification]> {
        self.tag_specifications.as_deref()
    }
}
impl CreateVpcPeeringConnectionInput {
    /// Creates a new builder-style object to manufacture [`CreateVpcPeeringConnectionInput`](crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput).
    pub fn builder() -> crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionInputBuilder{
        crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput;
/// A builder for [`CreateVpcPeeringConnectionInput`](crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput).
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
pub struct CreateVpcPeeringConnectionInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) peer_owner_id: std::option::Option<std::string::String>,
    pub(crate) peer_vpc_id: std::option::Option<std::string::String>,
    pub(crate) vpc_id: std::option::Option<std::string::String>,
    pub(crate) peer_region: std::option::Option<std::string::String>,
    pub(crate) tag_specifications:
        std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
}
impl CreateVpcPeeringConnectionInputBuilder {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The Amazon Web Services account ID of the owner of the accepter VPC.</p>
    /// <p>Default: Your Amazon Web Services account ID</p>
    pub fn peer_owner_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.peer_owner_id = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID of the owner of the accepter VPC.</p>
    /// <p>Default: Your Amazon Web Services account ID</p>
    pub fn set_peer_owner_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.peer_owner_id = input;
        self
    }
    /// <p>The ID of the VPC with which you are creating the VPC peering connection. You must specify this parameter in the request.</p>
    pub fn peer_vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.peer_vpc_id = Some(input.into());
        self
    }
    /// <p>The ID of the VPC with which you are creating the VPC peering connection. You must specify this parameter in the request.</p>
    pub fn set_peer_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.peer_vpc_id = input;
        self
    }
    /// <p>The ID of the requester VPC. You must specify this parameter in the request.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpc_id = Some(input.into());
        self
    }
    /// <p>The ID of the requester VPC. You must specify this parameter in the request.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>The Region code for the accepter VPC, if the accepter VPC is located in a Region other than the Region in which you make the request.</p>
    /// <p>Default: The Region in which you make the request.</p>
    pub fn peer_region(mut self, input: impl Into<std::string::String>) -> Self {
        self.peer_region = Some(input.into());
        self
    }
    /// <p>The Region code for the accepter VPC, if the accepter VPC is located in a Region other than the Region in which you make the request.</p>
    /// <p>Default: The Region in which you make the request.</p>
    pub fn set_peer_region(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.peer_region = input;
        self
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the peering connection.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = Some(v);
        self
    }
    /// <p>The tags to assign to the peering connection.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.tag_specifications = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateVpcPeeringConnectionInput`](crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionInput {
                dry_run: self.dry_run,
                peer_owner_id: self.peer_owner_id,
                peer_vpc_id: self.peer_vpc_id,
                vpc_id: self.vpc_id,
                peer_region: self.peer_region,
                tag_specifications: self.tag_specifications,
            },
        )
    }
}
