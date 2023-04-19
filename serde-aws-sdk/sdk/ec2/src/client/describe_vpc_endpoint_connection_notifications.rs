// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeVpcEndpointConnectionNotifications`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`connection_notification_id(impl Into<String>)`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::connection_notification_id) / [`set_connection_notification_id(Option<String>)`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::set_connection_notification_id): <p>The ID of the notification.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::set_filters): <p>The filters.</p>  <ul>   <li> <p> <code>connection-notification-arn</code> - The ARN of the SNS topic for the notification.</p> </li>   <li> <p> <code>connection-notification-id</code> - The ID of the notification.</p> </li>   <li> <p> <code>connection-notification-state</code> - The state of the notification (<code>Enabled</code> | <code>Disabled</code>).</p> </li>   <li> <p> <code>connection-notification-type</code> - The type of notification (<code>Topic</code>).</p> </li>   <li> <p> <code>service-id</code> - The ID of the endpoint service.</p> </li>   <li> <p> <code>vpc-endpoint-id</code> - The ID of the VPC endpoint.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::set_max_results): <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::set_next_token): <p>The token to request the next page of results.</p>
    /// - On success, responds with [`DescribeVpcEndpointConnectionNotificationsOutput`](crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsOutput) with field(s):
    ///   - [`connection_notification_set(Option<Vec<ConnectionNotification>>)`](crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsOutput::connection_notification_set): <p>The notifications.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeVpcEndpointConnectionNotificationsError>`](crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsError)
    pub fn describe_vpc_endpoint_connection_notifications(&self) -> crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder{
        crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsFluentBuilder::new(self.handle.clone())
    }
}