// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeRegions`](crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder::set_filters): <p>The filters.</p>  <ul>   <li> <p> <code>endpoint</code> - The endpoint of the Region (for example, <code>ec2.us-east-1.amazonaws.com</code>).</p> </li>   <li> <p> <code>opt-in-status</code> - The opt-in status of the Region (<code>opt-in-not-required</code> | <code>opted-in</code> | <code>not-opted-in</code>).</p> </li>   <li> <p> <code>region-name</code> - The name of the Region (for example, <code>us-east-1</code>).</p> </li>  </ul>
    ///   - [`region_names(Vec<String>)`](crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder::region_names) / [`set_region_names(Option<Vec<String>>)`](crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder::set_region_names): <p>The names of the Regions. You can specify any Regions, whether they are enabled and disabled for your account.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`all_regions(bool)`](crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder::all_regions) / [`set_all_regions(Option<bool>)`](crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder::set_all_regions): <p>Indicates whether to display all Regions, including Regions that are disabled for your account.</p>
    /// - On success, responds with [`DescribeRegionsOutput`](crate::operation::describe_regions::DescribeRegionsOutput) with field(s):
    ///   - [`regions(Option<Vec<Region>>)`](crate::operation::describe_regions::DescribeRegionsOutput::regions): <p>Information about the Regions.</p>
    /// - On failure, responds with [`SdkError<DescribeRegionsError>`](crate::operation::describe_regions::DescribeRegionsError)
    pub fn describe_regions(
        &self,
    ) -> crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder {
        crate::operation::describe_regions::builders::DescribeRegionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}