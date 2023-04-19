// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeVpcPeeringConnections`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>accepter-vpc-info.cidr-block</code> - The IPv4 CIDR block of the accepter VPC.</p> </li>   <li> <p> <code>accepter-vpc-info.owner-id</code> - The ID of the Amazon Web Services account that owns the accepter VPC.</p> </li>   <li> <p> <code>accepter-vpc-info.vpc-id</code> - The ID of the accepter VPC.</p> </li>   <li> <p> <code>expiration-time</code> - The expiration date and time for the VPC peering connection.</p> </li>   <li> <p> <code>requester-vpc-info.cidr-block</code> - The IPv4 CIDR block of the requester's VPC.</p> </li>   <li> <p> <code>requester-vpc-info.owner-id</code> - The ID of the Amazon Web Services account that owns the requester VPC.</p> </li>   <li> <p> <code>requester-vpc-info.vpc-id</code> - The ID of the requester VPC.</p> </li>   <li> <p> <code>status-code</code> - The status of the VPC peering connection (<code>pending-acceptance</code> | <code>failed</code> | <code>expired</code> | <code>provisioning</code> | <code>active</code> | <code>deleting</code> | <code>deleted</code> | <code>rejected</code>).</p> </li>   <li> <p> <code>status-message</code> - A message that provides more information about the status of the VPC peering connection, if applicable.</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>vpc-peering-connection-id</code> - The ID of the VPC peering connection.</p> </li>  </ul>
    ///   - [`dry_run(bool)`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`vpc_peering_connection_ids(Vec<String>)`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::vpc_peering_connection_ids) / [`set_vpc_peering_connection_ids(Option<Vec<String>>)`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::set_vpc_peering_connection_ids): <p>One or more VPC peering connection IDs.</p>  <p>Default: Describes all your VPC peering connections.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::set_next_token): <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::set_max_results): <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    /// - On success, responds with [`DescribeVpcPeeringConnectionsOutput`](crate::operation::describe_vpc_peering_connections::DescribeVpcPeeringConnectionsOutput) with field(s):
    ///   - [`vpc_peering_connections(Option<Vec<VpcPeeringConnection>>)`](crate::operation::describe_vpc_peering_connections::DescribeVpcPeeringConnectionsOutput::vpc_peering_connections): <p>Information about the VPC peering connections.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_vpc_peering_connections::DescribeVpcPeeringConnectionsOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<DescribeVpcPeeringConnectionsError>`](crate::operation::describe_vpc_peering_connections::DescribeVpcPeeringConnectionsError)
    pub fn describe_vpc_peering_connections(&self) -> crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder{
        crate::operation::describe_vpc_peering_connections::builders::DescribeVpcPeeringConnectionsFluentBuilder::new(self.handle.clone())
    }
}