// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeLaunchTemplates`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`launch_template_ids(Vec<String>)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::launch_template_ids) / [`set_launch_template_ids(Option<Vec<String>>)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::set_launch_template_ids): <p>One or more launch template IDs.</p>
    ///   - [`launch_template_names(Vec<String>)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::launch_template_names) / [`set_launch_template_names(Option<Vec<String>>)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::set_launch_template_names): <p>One or more launch template names.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>create-time</code> - The time the launch template was created.</p> </li>   <li> <p> <code>launch-template-name</code> - The name of the launch template.</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>  </ul>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::set_next_token): <p>The token to request the next page of results.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::set_max_results): <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value. This value can be between 1 and 200.</p>
    /// - On success, responds with [`DescribeLaunchTemplatesOutput`](crate::operation::describe_launch_templates::DescribeLaunchTemplatesOutput) with field(s):
    ///   - [`launch_templates(Option<Vec<LaunchTemplate>>)`](crate::operation::describe_launch_templates::DescribeLaunchTemplatesOutput::launch_templates): <p>Information about the launch templates.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_launch_templates::DescribeLaunchTemplatesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeLaunchTemplatesError>`](crate::operation::describe_launch_templates::DescribeLaunchTemplatesError)
    pub fn describe_launch_templates(
        &self,
    ) -> crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder
    {
        crate::operation::describe_launch_templates::builders::DescribeLaunchTemplatesFluentBuilder::new(self.handle.clone())
    }
}
