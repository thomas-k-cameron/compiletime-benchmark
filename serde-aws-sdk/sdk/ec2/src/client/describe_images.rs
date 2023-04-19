// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeImages`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`executable_users(Vec<String>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::executable_users) / [`set_executable_users(Option<Vec<String>>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::set_executable_users): <p>Scopes the images by users with explicit launch permissions. Specify an Amazon Web Services account ID, <code>self</code> (the sender of the request), or <code>all</code> (public AMIs).</p>  <ul>   <li> <p>If you specify an Amazon Web Services account ID that is not your own, only AMIs shared with that specific Amazon Web Services account ID are returned. However, AMIs that are shared with the account’s organization or organizational unit (OU) are not returned.</p> </li>   <li> <p>If you specify <code>self</code> or your own Amazon Web Services account ID, AMIs shared with your account are returned. In addition, AMIs that are shared with the organization or OU of which you are member are also returned. </p> </li>   <li> <p>If you specify <code>all</code>, all public AMIs are returned.</p> </li>  </ul>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::set_filters): <p>The filters.</p>  <ul>   <li> <p> <code>architecture</code> - The image architecture (<code>i386</code> | <code>x86_64</code> | <code>arm64</code>).</p> </li>   <li> <p> <code>block-device-mapping.delete-on-termination</code> - A Boolean value that indicates whether the Amazon EBS volume is deleted on instance termination.</p> </li>   <li> <p> <code>block-device-mapping.device-name</code> - The device name specified in the block device mapping (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p> </li>   <li> <p> <code>block-device-mapping.snapshot-id</code> - The ID of the snapshot used for the Amazon EBS volume.</p> </li>   <li> <p> <code>block-device-mapping.volume-size</code> - The volume size of the Amazon EBS volume, in GiB.</p> </li>   <li> <p> <code>block-device-mapping.volume-type</code> - The volume type of the Amazon EBS volume (<code>io1</code> | <code>io2</code> | <code>gp2</code> | <code>gp3</code> | <code>sc1 </code>| <code>st1</code> | <code>standard</code>).</p> </li>   <li> <p> <code>block-device-mapping.encrypted</code> - A Boolean that indicates whether the Amazon EBS volume is encrypted.</p> </li>   <li> <p> <code>creation-date</code> - The time when the image was created, in the ISO 8601 format in the UTC time zone (YYYY-MM-DDThh:mm:ss.sssZ), for example, <code>2021-09-29T11:04:43.305Z</code>. You can use a wildcard (<code>*</code>), for example, <code>2021-09-29T*</code>, which matches an entire day.</p> </li>   <li> <p> <code>description</code> - The description of the image (provided during image creation).</p> </li>   <li> <p> <code>ena-support</code> - A Boolean that indicates whether enhanced networking with ENA is enabled.</p> </li>   <li> <p> <code>hypervisor</code> - The hypervisor type (<code>ovm</code> | <code>xen</code>).</p> </li>   <li> <p> <code>image-id</code> - The ID of the image.</p> </li>   <li> <p> <code>image-type</code> - The image type (<code>machine</code> | <code>kernel</code> | <code>ramdisk</code>).</p> </li>   <li> <p> <code>is-public</code> - A Boolean that indicates whether the image is public.</p> </li>   <li> <p> <code>kernel-id</code> - The kernel ID.</p> </li>   <li> <p> <code>manifest-location</code> - The location of the image manifest.</p> </li>   <li> <p> <code>name</code> - The name of the AMI (provided during image creation).</p> </li>   <li> <p> <code>owner-alias</code> - The owner alias (<code>amazon</code> | <code>aws-marketplace</code>). The valid aliases are defined in an Amazon-maintained list. This is not the Amazon Web Services account alias that can be set using the IAM console. We recommend that you use the <b>Owner</b> request parameter instead of this filter.</p> </li>   <li> <p> <code>owner-id</code> - The Amazon Web Services account ID of the owner. We recommend that you use the <b>Owner</b> request parameter instead of this filter.</p> </li>   <li> <p> <code>platform</code> - The platform. The only supported value is <code>windows</code>.</p> </li>   <li> <p> <code>product-code</code> - The product code.</p> </li>   <li> <p> <code>product-code.type</code> - The type of the product code (<code>marketplace</code>).</p> </li>   <li> <p> <code>ramdisk-id</code> - The RAM disk ID.</p> </li>   <li> <p> <code>root-device-name</code> - The device name of the root device volume (for example, <code>/dev/sda1</code>).</p> </li>   <li> <p> <code>root-device-type</code> - The type of the root device volume (<code>ebs</code> | <code>instance-store</code>).</p> </li>   <li> <p> <code>state</code> - The state of the image (<code>available</code> | <code>pending</code> | <code>failed</code>).</p> </li>   <li> <p> <code>state-reason-code</code> - The reason code for the state change.</p> </li>   <li> <p> <code>state-reason-message</code> - The message for the state change.</p> </li>   <li> <p> <code>sriov-net-support</code> - A value of <code>simple</code> indicates that enhanced networking with the Intel 82599 VF interface is enabled.</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>virtualization-type</code> - The virtualization type (<code>paravirtual</code> | <code>hvm</code>).</p> </li>  </ul>
    ///   - [`image_ids(Vec<String>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::image_ids) / [`set_image_ids(Option<Vec<String>>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::set_image_ids): <p>The image IDs.</p>  <p>Default: Describes all images available to you.</p>
    ///   - [`owners(Vec<String>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::owners) / [`set_owners(Option<Vec<String>>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::set_owners): <p>Scopes the results to images with the specified owners. You can specify a combination of Amazon Web Services account IDs, <code>self</code>, <code>amazon</code>, and <code>aws-marketplace</code>. If you omit this parameter, the results include all images for which you have launch permissions, regardless of ownership.</p>
    ///   - [`include_deprecated(bool)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::include_deprecated) / [`set_include_deprecated(Option<bool>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::set_include_deprecated): <p>Specifies whether to include deprecated AMIs.</p>  <p>Default: No deprecated AMIs are included in the response.</p> <note>   <p>If you are the AMI owner, all deprecated AMIs appear in the response regardless of what you specify for this parameter.</p>  </note>
    ///   - [`dry_run(bool)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::set_max_results): <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_images::builders::DescribeImagesFluentBuilder::set_next_token): <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    /// - On success, responds with [`DescribeImagesOutput`](crate::operation::describe_images::DescribeImagesOutput) with field(s):
    ///   - [`images(Option<Vec<Image>>)`](crate::operation::describe_images::DescribeImagesOutput::images): <p>Information about the images.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_images::DescribeImagesOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<DescribeImagesError>`](crate::operation::describe_images::DescribeImagesError)
    pub fn describe_images(
        &self,
    ) -> crate::operation::describe_images::builders::DescribeImagesFluentBuilder {
        crate::operation::describe_images::builders::DescribeImagesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
