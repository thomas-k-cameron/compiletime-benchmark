// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an association between a local gateway route table and a virtual interface group.</p>
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
pub struct LocalGatewayRouteTableVirtualInterfaceGroupAssociation {
    /// <p>The ID of the association.</p>
    #[doc(hidden)]
    pub local_gateway_route_table_virtual_interface_group_association_id:
        std::option::Option<std::string::String>,
    /// <p>The ID of the virtual interface group.</p>
    #[doc(hidden)]
    pub local_gateway_virtual_interface_group_id: std::option::Option<std::string::String>,
    /// <p>The ID of the local gateway.</p>
    #[doc(hidden)]
    pub local_gateway_id: std::option::Option<std::string::String>,
    /// <p>The ID of the local gateway route table.</p>
    #[doc(hidden)]
    pub local_gateway_route_table_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the local gateway route table for the virtual interface group.</p>
    #[doc(hidden)]
    pub local_gateway_route_table_arn: std::option::Option<std::string::String>,
    /// <p>The ID of the Amazon Web Services account that owns the local gateway virtual interface group association.</p>
    #[doc(hidden)]
    pub owner_id: std::option::Option<std::string::String>,
    /// <p>The state of the association.</p>
    #[doc(hidden)]
    pub state: std::option::Option<std::string::String>,
    /// <p>The tags assigned to the association.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl LocalGatewayRouteTableVirtualInterfaceGroupAssociation {
    /// <p>The ID of the association.</p>
    pub fn local_gateway_route_table_virtual_interface_group_association_id(
        &self,
    ) -> std::option::Option<&str> {
        self.local_gateway_route_table_virtual_interface_group_association_id
            .as_deref()
    }
    /// <p>The ID of the virtual interface group.</p>
    pub fn local_gateway_virtual_interface_group_id(&self) -> std::option::Option<&str> {
        self.local_gateway_virtual_interface_group_id.as_deref()
    }
    /// <p>The ID of the local gateway.</p>
    pub fn local_gateway_id(&self) -> std::option::Option<&str> {
        self.local_gateway_id.as_deref()
    }
    /// <p>The ID of the local gateway route table.</p>
    pub fn local_gateway_route_table_id(&self) -> std::option::Option<&str> {
        self.local_gateway_route_table_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the local gateway route table for the virtual interface group.</p>
    pub fn local_gateway_route_table_arn(&self) -> std::option::Option<&str> {
        self.local_gateway_route_table_arn.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the local gateway virtual interface group association.</p>
    pub fn owner_id(&self) -> std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The state of the association.</p>
    pub fn state(&self) -> std::option::Option<&str> {
        self.state.as_deref()
    }
    /// <p>The tags assigned to the association.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl LocalGatewayRouteTableVirtualInterfaceGroupAssociation {
    /// Creates a new builder-style object to manufacture [`LocalGatewayRouteTableVirtualInterfaceGroupAssociation`](crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation).
    pub fn builder(
    ) -> crate::types::builders::LocalGatewayRouteTableVirtualInterfaceGroupAssociationBuilder {
        crate::types::builders::LocalGatewayRouteTableVirtualInterfaceGroupAssociationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation;
/// A builder for [`LocalGatewayRouteTableVirtualInterfaceGroupAssociation`](crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation).
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
pub struct LocalGatewayRouteTableVirtualInterfaceGroupAssociationBuilder {
    pub(crate) local_gateway_route_table_virtual_interface_group_association_id:
        std::option::Option<std::string::String>,
    pub(crate) local_gateway_virtual_interface_group_id: std::option::Option<std::string::String>,
    pub(crate) local_gateway_id: std::option::Option<std::string::String>,
    pub(crate) local_gateway_route_table_id: std::option::Option<std::string::String>,
    pub(crate) local_gateway_route_table_arn: std::option::Option<std::string::String>,
    pub(crate) owner_id: std::option::Option<std::string::String>,
    pub(crate) state: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl LocalGatewayRouteTableVirtualInterfaceGroupAssociationBuilder {
    /// <p>The ID of the association.</p>
    pub fn local_gateway_route_table_virtual_interface_group_association_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.local_gateway_route_table_virtual_interface_group_association_id = Some(input.into());
        self
    }
    /// <p>The ID of the association.</p>
    pub fn set_local_gateway_route_table_virtual_interface_group_association_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.local_gateway_route_table_virtual_interface_group_association_id = input;
        self
    }
    /// <p>The ID of the virtual interface group.</p>
    pub fn local_gateway_virtual_interface_group_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.local_gateway_virtual_interface_group_id = Some(input.into());
        self
    }
    /// <p>The ID of the virtual interface group.</p>
    pub fn set_local_gateway_virtual_interface_group_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.local_gateway_virtual_interface_group_id = input;
        self
    }
    /// <p>The ID of the local gateway.</p>
    pub fn local_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_gateway_id = Some(input.into());
        self
    }
    /// <p>The ID of the local gateway.</p>
    pub fn set_local_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.local_gateway_id = input;
        self
    }
    /// <p>The ID of the local gateway route table.</p>
    pub fn local_gateway_route_table_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_gateway_route_table_id = Some(input.into());
        self
    }
    /// <p>The ID of the local gateway route table.</p>
    pub fn set_local_gateway_route_table_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.local_gateway_route_table_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the local gateway route table for the virtual interface group.</p>
    pub fn local_gateway_route_table_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_gateway_route_table_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the local gateway route table for the virtual interface group.</p>
    pub fn set_local_gateway_route_table_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.local_gateway_route_table_arn = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the local gateway virtual interface group association.</p>
    pub fn owner_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.owner_id = Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the local gateway virtual interface group association.</p>
    pub fn set_owner_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The state of the association.</p>
    pub fn state(mut self, input: impl Into<std::string::String>) -> Self {
        self.state = Some(input.into());
        self
    }
    /// <p>The state of the association.</p>
    pub fn set_state(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.state = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags assigned to the association.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags assigned to the association.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`LocalGatewayRouteTableVirtualInterfaceGroupAssociation`](crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation).
    pub fn build(self) -> crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation {
        crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation {
            local_gateway_route_table_virtual_interface_group_association_id: self
                .local_gateway_route_table_virtual_interface_group_association_id,
            local_gateway_virtual_interface_group_id: self.local_gateway_virtual_interface_group_id,
            local_gateway_id: self.local_gateway_id,
            local_gateway_route_table_id: self.local_gateway_route_table_id,
            local_gateway_route_table_arn: self.local_gateway_route_table_arn,
            owner_id: self.owner_id,
            state: self.state,
            tags: self.tags,
        }
    }
}
