// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTransitGatewayPeeringAttachments`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_attachment_ids(Vec<String>)`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::transit_gateway_attachment_ids) / [`set_transit_gateway_attachment_ids(Option<Vec<String>>)`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::set_transit_gateway_attachment_ids): <p>One or more IDs of the transit gateway peering attachments.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::set_filters): <p>One or more filters. The possible values are:</p>  <ul>   <li> <p> <code>transit-gateway-attachment-id</code> - The ID of the transit gateway attachment.</p> </li>   <li> <p> <code>local-owner-id</code> - The ID of your Amazon Web Services account.</p> </li>   <li> <p> <code>remote-owner-id</code> - The ID of the Amazon Web Services account in the remote Region that owns the transit gateway.</p> </li>   <li> <p> <code>state</code> - The state of the peering attachment. Valid values are <code>available</code> | <code>deleted</code> | <code>deleting</code> | <code>failed</code> | <code>failing</code> | <code>initiatingRequest</code> | <code>modifying</code> | <code>pendingAcceptance</code> | <code>pending</code> | <code>rollingBack</code> | <code>rejected</code> | <code>rejecting</code>).</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources that have a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>transit-gateway-id</code> - The ID of the transit gateway.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DescribeTransitGatewayPeeringAttachmentsOutput`](crate::operation::describe_transit_gateway_peering_attachments::DescribeTransitGatewayPeeringAttachmentsOutput) with field(s):
    ///   - [`transit_gateway_peering_attachments(Option<Vec<TransitGatewayPeeringAttachment>>)`](crate::operation::describe_transit_gateway_peering_attachments::DescribeTransitGatewayPeeringAttachmentsOutput::transit_gateway_peering_attachments): <p>The transit gateway peering attachments.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_transit_gateway_peering_attachments::DescribeTransitGatewayPeeringAttachmentsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeTransitGatewayPeeringAttachmentsError>`](crate::operation::describe_transit_gateway_peering_attachments::DescribeTransitGatewayPeeringAttachmentsError)
    pub fn describe_transit_gateway_peering_attachments(&self) -> crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder{
        crate::operation::describe_transit_gateway_peering_attachments::builders::DescribeTransitGatewayPeeringAttachmentsFluentBuilder::new(self.handle.clone())
    }
}