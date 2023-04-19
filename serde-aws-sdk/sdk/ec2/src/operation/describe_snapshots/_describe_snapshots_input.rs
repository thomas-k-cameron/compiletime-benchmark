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
pub struct DescribeSnapshotsInput {
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>description</code> - A description of the snapshot.</p> </li>
    /// <li> <p> <code>encrypted</code> - Indicates whether the snapshot is encrypted (<code>true</code> | <code>false</code>)</p> </li>
    /// <li> <p> <code>owner-alias</code> - The owner alias, from an Amazon-maintained list (<code>amazon</code>). This is not the user-configured Amazon Web Services account alias set using the IAM console. We recommend that you use the related parameter instead of this filter.</p> </li>
    /// <li> <p> <code>owner-id</code> - The Amazon Web Services account ID of the owner. We recommend that you use the related parameter instead of this filter.</p> </li>
    /// <li> <p> <code>progress</code> - The progress of the snapshot, as a percentage (for example, 80%).</p> </li>
    /// <li> <p> <code>snapshot-id</code> - The snapshot ID.</p> </li>
    /// <li> <p> <code>start-time</code> - The time stamp when the snapshot was initiated.</p> </li>
    /// <li> <p> <code>status</code> - The status of the snapshot (<code>pending</code> | <code>completed</code> | <code>error</code>).</p> </li>
    /// <li> <p> <code>storage-tier</code> - The storage tier of the snapshot (<code>archive</code> | <code>standard</code>).</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// <li> <p> <code>volume-id</code> - The ID of the volume the snapshot is for.</p> </li>
    /// <li> <p> <code>volume-size</code> - The size of the volume, in GiB.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of snapshots to return for this request. This value can be between 5 and 1,000; if this value is larger than 1,000, only 1,000 results are returned. If this parameter is not used, then the request returns all snapshots. You cannot specify this parameter and the snapshot IDs parameter in the same request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>Scopes the results to snapshots with the specified owners. You can specify a combination of Amazon Web Services account IDs, <code>self</code>, and <code>amazon</code>.</p>
    #[doc(hidden)]
    pub owner_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The IDs of the Amazon Web Services accounts that can create volumes from the snapshot.</p>
    #[doc(hidden)]
    pub restorable_by_user_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The snapshot IDs.</p>
    /// <p>Default: Describes the snapshots for which you have create volume permissions.</p>
    #[doc(hidden)]
    pub snapshot_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DescribeSnapshotsInput {
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>description</code> - A description of the snapshot.</p> </li>
    /// <li> <p> <code>encrypted</code> - Indicates whether the snapshot is encrypted (<code>true</code> | <code>false</code>)</p> </li>
    /// <li> <p> <code>owner-alias</code> - The owner alias, from an Amazon-maintained list (<code>amazon</code>). This is not the user-configured Amazon Web Services account alias set using the IAM console. We recommend that you use the related parameter instead of this filter.</p> </li>
    /// <li> <p> <code>owner-id</code> - The Amazon Web Services account ID of the owner. We recommend that you use the related parameter instead of this filter.</p> </li>
    /// <li> <p> <code>progress</code> - The progress of the snapshot, as a percentage (for example, 80%).</p> </li>
    /// <li> <p> <code>snapshot-id</code> - The snapshot ID.</p> </li>
    /// <li> <p> <code>start-time</code> - The time stamp when the snapshot was initiated.</p> </li>
    /// <li> <p> <code>status</code> - The status of the snapshot (<code>pending</code> | <code>completed</code> | <code>error</code>).</p> </li>
    /// <li> <p> <code>storage-tier</code> - The storage tier of the snapshot (<code>archive</code> | <code>standard</code>).</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// <li> <p> <code>volume-id</code> - The ID of the volume the snapshot is for.</p> </li>
    /// <li> <p> <code>volume-size</code> - The size of the volume, in GiB.</p> </li>
    /// </ul>
    pub fn filters(&self) -> std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The maximum number of snapshots to return for this request. This value can be between 5 and 1,000; if this value is larger than 1,000, only 1,000 results are returned. If this parameter is not used, then the request returns all snapshots. You cannot specify this parameter and the snapshot IDs parameter in the same request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Scopes the results to snapshots with the specified owners. You can specify a combination of Amazon Web Services account IDs, <code>self</code>, and <code>amazon</code>.</p>
    pub fn owner_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.owner_ids.as_deref()
    }
    /// <p>The IDs of the Amazon Web Services accounts that can create volumes from the snapshot.</p>
    pub fn restorable_by_user_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.restorable_by_user_ids.as_deref()
    }
    /// <p>The snapshot IDs.</p>
    /// <p>Default: Describes the snapshots for which you have create volume permissions.</p>
    pub fn snapshot_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.snapshot_ids.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeSnapshotsInput {
    /// Creates a new builder-style object to manufacture [`DescribeSnapshotsInput`](crate::operation::describe_snapshots::DescribeSnapshotsInput).
    pub fn builder() -> crate::operation::describe_snapshots::builders::DescribeSnapshotsInputBuilder
    {
        crate::operation::describe_snapshots::builders::DescribeSnapshotsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_snapshots::DescribeSnapshotsInput;
/// A builder for [`DescribeSnapshotsInput`](crate::operation::describe_snapshots::DescribeSnapshotsInput).
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
pub struct DescribeSnapshotsInputBuilder {
    pub(crate) filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) owner_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) restorable_by_user_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) snapshot_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DescribeSnapshotsInputBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>description</code> - A description of the snapshot.</p> </li>
    /// <li> <p> <code>encrypted</code> - Indicates whether the snapshot is encrypted (<code>true</code> | <code>false</code>)</p> </li>
    /// <li> <p> <code>owner-alias</code> - The owner alias, from an Amazon-maintained list (<code>amazon</code>). This is not the user-configured Amazon Web Services account alias set using the IAM console. We recommend that you use the related parameter instead of this filter.</p> </li>
    /// <li> <p> <code>owner-id</code> - The Amazon Web Services account ID of the owner. We recommend that you use the related parameter instead of this filter.</p> </li>
    /// <li> <p> <code>progress</code> - The progress of the snapshot, as a percentage (for example, 80%).</p> </li>
    /// <li> <p> <code>snapshot-id</code> - The snapshot ID.</p> </li>
    /// <li> <p> <code>start-time</code> - The time stamp when the snapshot was initiated.</p> </li>
    /// <li> <p> <code>status</code> - The status of the snapshot (<code>pending</code> | <code>completed</code> | <code>error</code>).</p> </li>
    /// <li> <p> <code>storage-tier</code> - The storage tier of the snapshot (<code>archive</code> | <code>standard</code>).</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// <li> <p> <code>volume-id</code> - The ID of the volume the snapshot is for.</p> </li>
    /// <li> <p> <code>volume-size</code> - The size of the volume, in GiB.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = Some(v);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>description</code> - A description of the snapshot.</p> </li>
    /// <li> <p> <code>encrypted</code> - Indicates whether the snapshot is encrypted (<code>true</code> | <code>false</code>)</p> </li>
    /// <li> <p> <code>owner-alias</code> - The owner alias, from an Amazon-maintained list (<code>amazon</code>). This is not the user-configured Amazon Web Services account alias set using the IAM console. We recommend that you use the related parameter instead of this filter.</p> </li>
    /// <li> <p> <code>owner-id</code> - The Amazon Web Services account ID of the owner. We recommend that you use the related parameter instead of this filter.</p> </li>
    /// <li> <p> <code>progress</code> - The progress of the snapshot, as a percentage (for example, 80%).</p> </li>
    /// <li> <p> <code>snapshot-id</code> - The snapshot ID.</p> </li>
    /// <li> <p> <code>start-time</code> - The time stamp when the snapshot was initiated.</p> </li>
    /// <li> <p> <code>status</code> - The status of the snapshot (<code>pending</code> | <code>completed</code> | <code>error</code>).</p> </li>
    /// <li> <p> <code>storage-tier</code> - The storage tier of the snapshot (<code>archive</code> | <code>standard</code>).</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// <li> <p> <code>volume-id</code> - The ID of the volume the snapshot is for.</p> </li>
    /// <li> <p> <code>volume-size</code> - The size of the volume, in GiB.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The maximum number of snapshots to return for this request. This value can be between 5 and 1,000; if this value is larger than 1,000, only 1,000 results are returned. If this parameter is not used, then the request returns all snapshots. You cannot specify this parameter and the snapshot IDs parameter in the same request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of snapshots to return for this request. This value can be between 5 and 1,000; if this value is larger than 1,000, only 1,000 results are returned. If this parameter is not used, then the request returns all snapshots. You cannot specify this parameter and the snapshot IDs parameter in the same request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `owner_ids`.
    ///
    /// To override the contents of this collection use [`set_owner_ids`](Self::set_owner_ids).
    ///
    /// <p>Scopes the results to snapshots with the specified owners. You can specify a combination of Amazon Web Services account IDs, <code>self</code>, and <code>amazon</code>.</p>
    pub fn owner_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.owner_ids.unwrap_or_default();
        v.push(input.into());
        self.owner_ids = Some(v);
        self
    }
    /// <p>Scopes the results to snapshots with the specified owners. You can specify a combination of Amazon Web Services account IDs, <code>self</code>, and <code>amazon</code>.</p>
    pub fn set_owner_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.owner_ids = input;
        self
    }
    /// Appends an item to `restorable_by_user_ids`.
    ///
    /// To override the contents of this collection use [`set_restorable_by_user_ids`](Self::set_restorable_by_user_ids).
    ///
    /// <p>The IDs of the Amazon Web Services accounts that can create volumes from the snapshot.</p>
    pub fn restorable_by_user_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.restorable_by_user_ids.unwrap_or_default();
        v.push(input.into());
        self.restorable_by_user_ids = Some(v);
        self
    }
    /// <p>The IDs of the Amazon Web Services accounts that can create volumes from the snapshot.</p>
    pub fn set_restorable_by_user_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.restorable_by_user_ids = input;
        self
    }
    /// Appends an item to `snapshot_ids`.
    ///
    /// To override the contents of this collection use [`set_snapshot_ids`](Self::set_snapshot_ids).
    ///
    /// <p>The snapshot IDs.</p>
    /// <p>Default: Describes the snapshots for which you have create volume permissions.</p>
    pub fn snapshot_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.snapshot_ids.unwrap_or_default();
        v.push(input.into());
        self.snapshot_ids = Some(v);
        self
    }
    /// <p>The snapshot IDs.</p>
    /// <p>Default: Describes the snapshots for which you have create volume permissions.</p>
    pub fn set_snapshot_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.snapshot_ids = input;
        self
    }
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
    /// Consumes the builder and constructs a [`DescribeSnapshotsInput`](crate::operation::describe_snapshots::DescribeSnapshotsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_snapshots::DescribeSnapshotsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_snapshots::DescribeSnapshotsInput {
                filters: self.filters,
                max_results: self.max_results,
                next_token: self.next_token,
                owner_ids: self.owner_ids,
                restorable_by_user_ids: self.restorable_by_user_ids,
                snapshot_ids: self.snapshot_ids,
                dry_run: self.dry_run,
            },
        )
    }
}
