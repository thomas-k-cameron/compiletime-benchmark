// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeFlowLogs`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`filter(Vec<Filter>)`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::filter) / [`set_filter(Option<Vec<Filter>>)`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::set_filter): <p>One or more filters.</p>  <ul>   <li> <p> <code>deliver-log-status</code> - The status of the logs delivery (<code>SUCCESS</code> | <code>FAILED</code>).</p> </li>   <li> <p> <code>log-destination-type</code> - The type of destination for the flow log data (<code>cloud-watch-logs</code> | <code>s3</code> | <code>kinesis-data-firehose</code>).</p> </li>   <li> <p> <code>flow-log-id</code> - The ID of the flow log.</p> </li>   <li> <p> <code>log-group-name</code> - The name of the log group.</p> </li>   <li> <p> <code>resource-id</code> - The ID of the VPC, subnet, or network interface.</p> </li>   <li> <p> <code>traffic-type</code> - The type of traffic (<code>ACCEPT</code> | <code>REJECT</code> | <code>ALL</code>).</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>  </ul>
    ///   - [`flow_log_ids(Vec<String>)`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::flow_log_ids) / [`set_flow_log_ids(Option<Vec<String>>)`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::set_flow_log_ids): <p>One or more flow log IDs.</p>  <p>Constraint: Maximum of 1000 flow log IDs.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::set_max_results): <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::set_next_token): <p>The token to request the next page of items. Pagination continues from the end of the items returned by the previous request.</p>
    /// - On success, responds with [`DescribeFlowLogsOutput`](crate::operation::describe_flow_logs::DescribeFlowLogsOutput) with field(s):
    ///   - [`flow_logs(Option<Vec<FlowLog>>)`](crate::operation::describe_flow_logs::DescribeFlowLogsOutput::flow_logs): <p>Information about the flow logs.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_flow_logs::DescribeFlowLogsOutput::next_token): <p>The token to request the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<DescribeFlowLogsError>`](crate::operation::describe_flow_logs::DescribeFlowLogsError)
    pub fn describe_flow_logs(
        &self,
    ) -> crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder {
        crate::operation::describe_flow_logs::builders::DescribeFlowLogsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
