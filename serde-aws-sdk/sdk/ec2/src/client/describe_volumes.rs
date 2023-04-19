// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeVolumes`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::set_filters): <p>The filters.</p>  <ul>   <li> <p> <code>attachment.attach-time</code> - The time stamp when the attachment initiated.</p> </li>   <li> <p> <code>attachment.delete-on-termination</code> - Whether the volume is deleted on instance termination.</p> </li>   <li> <p> <code>attachment.device</code> - The device name specified in the block device mapping (for example, <code>/dev/sda1</code>).</p> </li>   <li> <p> <code>attachment.instance-id</code> - The ID of the instance the volume is attached to.</p> </li>   <li> <p> <code>attachment.status</code> - The attachment state (<code>attaching</code> | <code>attached</code> | <code>detaching</code>).</p> </li>   <li> <p> <code>availability-zone</code> - The Availability Zone in which the volume was created.</p> </li>   <li> <p> <code>create-time</code> - The time stamp when the volume was created.</p> </li>   <li> <p> <code>encrypted</code> - Indicates whether the volume is encrypted (<code>true</code> | <code>false</code>)</p> </li>   <li> <p> <code>multi-attach-enabled</code> - Indicates whether the volume is enabled for Multi-Attach (<code>true</code> | <code>false</code>)</p> </li>   <li> <p> <code>fast-restored</code> - Indicates whether the volume was created from a snapshot that is enabled for fast snapshot restore (<code>true</code> | <code>false</code>).</p> </li>   <li> <p> <code>size</code> - The size of the volume, in GiB.</p> </li>   <li> <p> <code>snapshot-id</code> - The snapshot from which the volume was created.</p> </li>   <li> <p> <code>status</code> - The state of the volume (<code>creating</code> | <code>available</code> | <code>in-use</code> | <code>deleting</code> | <code>deleted</code> | <code>error</code>).</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>volume-id</code> - The volume ID.</p> </li>   <li> <p> <code>volume-type</code> - The Amazon EBS volume type (<code>gp2</code> | <code>gp3</code> | <code>io1</code> | <code>io2</code> | <code>st1</code> | <code>sc1</code>| <code>standard</code>)</p> </li>  </ul>
    ///   - [`volume_ids(Vec<String>)`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::volume_ids) / [`set_volume_ids(Option<Vec<String>>)`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::set_volume_ids): <p>The volume IDs.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::set_max_results): <p>The maximum number of volumes to return for this request. This value can be between 5 and 500; if you specify a value larger than 500, only 500 items are returned. If this parameter is not used, then all items are returned. You cannot specify this parameter and the volume IDs parameter in the same request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::set_next_token): <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned from the previous request.</p>
    /// - On success, responds with [`DescribeVolumesOutput`](crate::operation::describe_volumes::DescribeVolumesOutput) with field(s):
    ///   - [`volumes(Option<Vec<Volume>>)`](crate::operation::describe_volumes::DescribeVolumesOutput::volumes): <p>Information about the volumes.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_volumes::DescribeVolumesOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<DescribeVolumesError>`](crate::operation::describe_volumes::DescribeVolumesError)
    pub fn describe_volumes(
        &self,
    ) -> crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder {
        crate::operation::describe_volumes::builders::DescribeVolumesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
