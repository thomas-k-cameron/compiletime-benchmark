// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a path component.</p>
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
pub struct PathComponent {
    /// <p>The sequence number.</p>
    #[doc(hidden)]
    pub sequence_number: std::option::Option<i32>,
    /// <p>The network ACL rule.</p>
    #[doc(hidden)]
    pub acl_rule: std::option::Option<crate::types::AnalysisAclRule>,
    /// <p>The resource to which the path component is attached.</p>
    #[doc(hidden)]
    pub attached_to: std::option::Option<crate::types::AnalysisComponent>,
    /// <p>The component.</p>
    #[doc(hidden)]
    pub component: std::option::Option<crate::types::AnalysisComponent>,
    /// <p>The destination VPC.</p>
    #[doc(hidden)]
    pub destination_vpc: std::option::Option<crate::types::AnalysisComponent>,
    /// <p>The outbound header.</p>
    #[doc(hidden)]
    pub outbound_header: std::option::Option<crate::types::AnalysisPacketHeader>,
    /// <p>The inbound header.</p>
    #[doc(hidden)]
    pub inbound_header: std::option::Option<crate::types::AnalysisPacketHeader>,
    /// <p>The route table route.</p>
    #[doc(hidden)]
    pub route_table_route: std::option::Option<crate::types::AnalysisRouteTableRoute>,
    /// <p>The security group rule.</p>
    #[doc(hidden)]
    pub security_group_rule: std::option::Option<crate::types::AnalysisSecurityGroupRule>,
    /// <p>The source VPC.</p>
    #[doc(hidden)]
    pub source_vpc: std::option::Option<crate::types::AnalysisComponent>,
    /// <p>The subnet.</p>
    #[doc(hidden)]
    pub subnet: std::option::Option<crate::types::AnalysisComponent>,
    /// <p>The component VPC.</p>
    #[doc(hidden)]
    pub vpc: std::option::Option<crate::types::AnalysisComponent>,
    /// <p>The additional details.</p>
    #[doc(hidden)]
    pub additional_details: std::option::Option<std::vec::Vec<crate::types::AdditionalDetail>>,
    /// <p>The transit gateway.</p>
    #[doc(hidden)]
    pub transit_gateway: std::option::Option<crate::types::AnalysisComponent>,
    /// <p>The route in a transit gateway route table.</p>
    #[doc(hidden)]
    pub transit_gateway_route_table_route:
        std::option::Option<crate::types::TransitGatewayRouteTableRoute>,
    /// <p>The explanation codes.</p>
    #[doc(hidden)]
    pub explanations: std::option::Option<std::vec::Vec<crate::types::Explanation>>,
    /// <p>The load balancer listener.</p>
    #[doc(hidden)]
    pub elastic_load_balancer_listener: std::option::Option<crate::types::AnalysisComponent>,
}
impl PathComponent {
    /// <p>The sequence number.</p>
    pub fn sequence_number(&self) -> std::option::Option<i32> {
        self.sequence_number
    }
    /// <p>The network ACL rule.</p>
    pub fn acl_rule(&self) -> std::option::Option<&crate::types::AnalysisAclRule> {
        self.acl_rule.as_ref()
    }
    /// <p>The resource to which the path component is attached.</p>
    pub fn attached_to(&self) -> std::option::Option<&crate::types::AnalysisComponent> {
        self.attached_to.as_ref()
    }
    /// <p>The component.</p>
    pub fn component(&self) -> std::option::Option<&crate::types::AnalysisComponent> {
        self.component.as_ref()
    }
    /// <p>The destination VPC.</p>
    pub fn destination_vpc(&self) -> std::option::Option<&crate::types::AnalysisComponent> {
        self.destination_vpc.as_ref()
    }
    /// <p>The outbound header.</p>
    pub fn outbound_header(&self) -> std::option::Option<&crate::types::AnalysisPacketHeader> {
        self.outbound_header.as_ref()
    }
    /// <p>The inbound header.</p>
    pub fn inbound_header(&self) -> std::option::Option<&crate::types::AnalysisPacketHeader> {
        self.inbound_header.as_ref()
    }
    /// <p>The route table route.</p>
    pub fn route_table_route(&self) -> std::option::Option<&crate::types::AnalysisRouteTableRoute> {
        self.route_table_route.as_ref()
    }
    /// <p>The security group rule.</p>
    pub fn security_group_rule(
        &self,
    ) -> std::option::Option<&crate::types::AnalysisSecurityGroupRule> {
        self.security_group_rule.as_ref()
    }
    /// <p>The source VPC.</p>
    pub fn source_vpc(&self) -> std::option::Option<&crate::types::AnalysisComponent> {
        self.source_vpc.as_ref()
    }
    /// <p>The subnet.</p>
    pub fn subnet(&self) -> std::option::Option<&crate::types::AnalysisComponent> {
        self.subnet.as_ref()
    }
    /// <p>The component VPC.</p>
    pub fn vpc(&self) -> std::option::Option<&crate::types::AnalysisComponent> {
        self.vpc.as_ref()
    }
    /// <p>The additional details.</p>
    pub fn additional_details(&self) -> std::option::Option<&[crate::types::AdditionalDetail]> {
        self.additional_details.as_deref()
    }
    /// <p>The transit gateway.</p>
    pub fn transit_gateway(&self) -> std::option::Option<&crate::types::AnalysisComponent> {
        self.transit_gateway.as_ref()
    }
    /// <p>The route in a transit gateway route table.</p>
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayRouteTableRoute> {
        self.transit_gateway_route_table_route.as_ref()
    }
    /// <p>The explanation codes.</p>
    pub fn explanations(&self) -> std::option::Option<&[crate::types::Explanation]> {
        self.explanations.as_deref()
    }
    /// <p>The load balancer listener.</p>
    pub fn elastic_load_balancer_listener(
        &self,
    ) -> std::option::Option<&crate::types::AnalysisComponent> {
        self.elastic_load_balancer_listener.as_ref()
    }
}
impl PathComponent {
    /// Creates a new builder-style object to manufacture [`PathComponent`](crate::types::PathComponent).
    pub fn builder() -> crate::types::builders::PathComponentBuilder {
        crate::types::builders::PathComponentBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::PathComponent;
/// A builder for [`PathComponent`](crate::types::PathComponent).
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
pub struct PathComponentBuilder {
    pub(crate) sequence_number: std::option::Option<i32>,
    pub(crate) acl_rule: std::option::Option<crate::types::AnalysisAclRule>,
    pub(crate) attached_to: std::option::Option<crate::types::AnalysisComponent>,
    pub(crate) component: std::option::Option<crate::types::AnalysisComponent>,
    pub(crate) destination_vpc: std::option::Option<crate::types::AnalysisComponent>,
    pub(crate) outbound_header: std::option::Option<crate::types::AnalysisPacketHeader>,
    pub(crate) inbound_header: std::option::Option<crate::types::AnalysisPacketHeader>,
    pub(crate) route_table_route: std::option::Option<crate::types::AnalysisRouteTableRoute>,
    pub(crate) security_group_rule: std::option::Option<crate::types::AnalysisSecurityGroupRule>,
    pub(crate) source_vpc: std::option::Option<crate::types::AnalysisComponent>,
    pub(crate) subnet: std::option::Option<crate::types::AnalysisComponent>,
    pub(crate) vpc: std::option::Option<crate::types::AnalysisComponent>,
    pub(crate) additional_details:
        std::option::Option<std::vec::Vec<crate::types::AdditionalDetail>>,
    pub(crate) transit_gateway: std::option::Option<crate::types::AnalysisComponent>,
    pub(crate) transit_gateway_route_table_route:
        std::option::Option<crate::types::TransitGatewayRouteTableRoute>,
    pub(crate) explanations: std::option::Option<std::vec::Vec<crate::types::Explanation>>,
    pub(crate) elastic_load_balancer_listener: std::option::Option<crate::types::AnalysisComponent>,
}
impl PathComponentBuilder {
    /// <p>The sequence number.</p>
    pub fn sequence_number(mut self, input: i32) -> Self {
        self.sequence_number = Some(input);
        self
    }
    /// <p>The sequence number.</p>
    pub fn set_sequence_number(mut self, input: std::option::Option<i32>) -> Self {
        self.sequence_number = input;
        self
    }
    /// <p>The network ACL rule.</p>
    pub fn acl_rule(mut self, input: crate::types::AnalysisAclRule) -> Self {
        self.acl_rule = Some(input);
        self
    }
    /// <p>The network ACL rule.</p>
    pub fn set_acl_rule(
        mut self,
        input: std::option::Option<crate::types::AnalysisAclRule>,
    ) -> Self {
        self.acl_rule = input;
        self
    }
    /// <p>The resource to which the path component is attached.</p>
    pub fn attached_to(mut self, input: crate::types::AnalysisComponent) -> Self {
        self.attached_to = Some(input);
        self
    }
    /// <p>The resource to which the path component is attached.</p>
    pub fn set_attached_to(
        mut self,
        input: std::option::Option<crate::types::AnalysisComponent>,
    ) -> Self {
        self.attached_to = input;
        self
    }
    /// <p>The component.</p>
    pub fn component(mut self, input: crate::types::AnalysisComponent) -> Self {
        self.component = Some(input);
        self
    }
    /// <p>The component.</p>
    pub fn set_component(
        mut self,
        input: std::option::Option<crate::types::AnalysisComponent>,
    ) -> Self {
        self.component = input;
        self
    }
    /// <p>The destination VPC.</p>
    pub fn destination_vpc(mut self, input: crate::types::AnalysisComponent) -> Self {
        self.destination_vpc = Some(input);
        self
    }
    /// <p>The destination VPC.</p>
    pub fn set_destination_vpc(
        mut self,
        input: std::option::Option<crate::types::AnalysisComponent>,
    ) -> Self {
        self.destination_vpc = input;
        self
    }
    /// <p>The outbound header.</p>
    pub fn outbound_header(mut self, input: crate::types::AnalysisPacketHeader) -> Self {
        self.outbound_header = Some(input);
        self
    }
    /// <p>The outbound header.</p>
    pub fn set_outbound_header(
        mut self,
        input: std::option::Option<crate::types::AnalysisPacketHeader>,
    ) -> Self {
        self.outbound_header = input;
        self
    }
    /// <p>The inbound header.</p>
    pub fn inbound_header(mut self, input: crate::types::AnalysisPacketHeader) -> Self {
        self.inbound_header = Some(input);
        self
    }
    /// <p>The inbound header.</p>
    pub fn set_inbound_header(
        mut self,
        input: std::option::Option<crate::types::AnalysisPacketHeader>,
    ) -> Self {
        self.inbound_header = input;
        self
    }
    /// <p>The route table route.</p>
    pub fn route_table_route(mut self, input: crate::types::AnalysisRouteTableRoute) -> Self {
        self.route_table_route = Some(input);
        self
    }
    /// <p>The route table route.</p>
    pub fn set_route_table_route(
        mut self,
        input: std::option::Option<crate::types::AnalysisRouteTableRoute>,
    ) -> Self {
        self.route_table_route = input;
        self
    }
    /// <p>The security group rule.</p>
    pub fn security_group_rule(mut self, input: crate::types::AnalysisSecurityGroupRule) -> Self {
        self.security_group_rule = Some(input);
        self
    }
    /// <p>The security group rule.</p>
    pub fn set_security_group_rule(
        mut self,
        input: std::option::Option<crate::types::AnalysisSecurityGroupRule>,
    ) -> Self {
        self.security_group_rule = input;
        self
    }
    /// <p>The source VPC.</p>
    pub fn source_vpc(mut self, input: crate::types::AnalysisComponent) -> Self {
        self.source_vpc = Some(input);
        self
    }
    /// <p>The source VPC.</p>
    pub fn set_source_vpc(
        mut self,
        input: std::option::Option<crate::types::AnalysisComponent>,
    ) -> Self {
        self.source_vpc = input;
        self
    }
    /// <p>The subnet.</p>
    pub fn subnet(mut self, input: crate::types::AnalysisComponent) -> Self {
        self.subnet = Some(input);
        self
    }
    /// <p>The subnet.</p>
    pub fn set_subnet(
        mut self,
        input: std::option::Option<crate::types::AnalysisComponent>,
    ) -> Self {
        self.subnet = input;
        self
    }
    /// <p>The component VPC.</p>
    pub fn vpc(mut self, input: crate::types::AnalysisComponent) -> Self {
        self.vpc = Some(input);
        self
    }
    /// <p>The component VPC.</p>
    pub fn set_vpc(mut self, input: std::option::Option<crate::types::AnalysisComponent>) -> Self {
        self.vpc = input;
        self
    }
    /// Appends an item to `additional_details`.
    ///
    /// To override the contents of this collection use [`set_additional_details`](Self::set_additional_details).
    ///
    /// <p>The additional details.</p>
    pub fn additional_details(mut self, input: crate::types::AdditionalDetail) -> Self {
        let mut v = self.additional_details.unwrap_or_default();
        v.push(input);
        self.additional_details = Some(v);
        self
    }
    /// <p>The additional details.</p>
    pub fn set_additional_details(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AdditionalDetail>>,
    ) -> Self {
        self.additional_details = input;
        self
    }
    /// <p>The transit gateway.</p>
    pub fn transit_gateway(mut self, input: crate::types::AnalysisComponent) -> Self {
        self.transit_gateway = Some(input);
        self
    }
    /// <p>The transit gateway.</p>
    pub fn set_transit_gateway(
        mut self,
        input: std::option::Option<crate::types::AnalysisComponent>,
    ) -> Self {
        self.transit_gateway = input;
        self
    }
    /// <p>The route in a transit gateway route table.</p>
    pub fn transit_gateway_route_table_route(
        mut self,
        input: crate::types::TransitGatewayRouteTableRoute,
    ) -> Self {
        self.transit_gateway_route_table_route = Some(input);
        self
    }
    /// <p>The route in a transit gateway route table.</p>
    pub fn set_transit_gateway_route_table_route(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayRouteTableRoute>,
    ) -> Self {
        self.transit_gateway_route_table_route = input;
        self
    }
    /// Appends an item to `explanations`.
    ///
    /// To override the contents of this collection use [`set_explanations`](Self::set_explanations).
    ///
    /// <p>The explanation codes.</p>
    pub fn explanations(mut self, input: crate::types::Explanation) -> Self {
        let mut v = self.explanations.unwrap_or_default();
        v.push(input);
        self.explanations = Some(v);
        self
    }
    /// <p>The explanation codes.</p>
    pub fn set_explanations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Explanation>>,
    ) -> Self {
        self.explanations = input;
        self
    }
    /// <p>The load balancer listener.</p>
    pub fn elastic_load_balancer_listener(
        mut self,
        input: crate::types::AnalysisComponent,
    ) -> Self {
        self.elastic_load_balancer_listener = Some(input);
        self
    }
    /// <p>The load balancer listener.</p>
    pub fn set_elastic_load_balancer_listener(
        mut self,
        input: std::option::Option<crate::types::AnalysisComponent>,
    ) -> Self {
        self.elastic_load_balancer_listener = input;
        self
    }
    /// Consumes the builder and constructs a [`PathComponent`](crate::types::PathComponent).
    pub fn build(self) -> crate::types::PathComponent {
        crate::types::PathComponent {
            sequence_number: self.sequence_number,
            acl_rule: self.acl_rule,
            attached_to: self.attached_to,
            component: self.component,
            destination_vpc: self.destination_vpc,
            outbound_header: self.outbound_header,
            inbound_header: self.inbound_header,
            route_table_route: self.route_table_route,
            security_group_rule: self.security_group_rule,
            source_vpc: self.source_vpc,
            subnet: self.subnet,
            vpc: self.vpc,
            additional_details: self.additional_details,
            transit_gateway: self.transit_gateway,
            transit_gateway_route_table_route: self.transit_gateway_route_table_route,
            explanations: self.explanations,
            elastic_load_balancer_listener: self.elastic_load_balancer_listener,
        }
    }
}
