// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeSnapshotTierStatus`](crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder::set_filters): <p>The filters.</p>  <ul>   <li> <p> <code>snapshot-id</code> - The snapshot ID.</p> </li>   <li> <p> <code>volume-id</code> - The ID of the volume the snapshot is for.</p> </li>   <li> <p> <code>last-tiering-operation</code> - The state of the last archive or restore action. (<code>archival-in-progress</code> | <code>archival-completed</code> | <code>archival-failed</code> | <code>permanent-restore-in-progress</code> | <code>permanent-restore-completed</code> | <code>permanent-restore-failed</code> | <code>temporary-restore-in-progress</code> | <code>temporary-restore-completed</code> | <code>temporary-restore-failed</code>)</p> </li>  </ul>
    ///   - [`dry_run(bool)`](crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder::set_next_token): <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder::set_max_results): <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    /// - On success, responds with [`DescribeSnapshotTierStatusOutput`](crate::operation::describe_snapshot_tier_status::DescribeSnapshotTierStatusOutput) with field(s):
    ///   - [`snapshot_tier_statuses(Option<Vec<SnapshotTierStatus>>)`](crate::operation::describe_snapshot_tier_status::DescribeSnapshotTierStatusOutput::snapshot_tier_statuses): <p>Information about the snapshot's storage tier.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_snapshot_tier_status::DescribeSnapshotTierStatusOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<DescribeSnapshotTierStatusError>`](crate::operation::describe_snapshot_tier_status::DescribeSnapshotTierStatusError)
    pub fn describe_snapshot_tier_status(&self) -> crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder{
        crate::operation::describe_snapshot_tier_status::builders::DescribeSnapshotTierStatusFluentBuilder::new(self.handle.clone())
    }
}
