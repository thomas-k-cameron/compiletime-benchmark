// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListMultiRegionAccessPoints`](crate::operation::list_multi_region_access_points::builders::ListMultiRegionAccessPointsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_multi_region_access_points::builders::ListMultiRegionAccessPointsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::list_multi_region_access_points::builders::ListMultiRegionAccessPointsFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::list_multi_region_access_points::builders::ListMultiRegionAccessPointsFluentBuilder::set_account_id): <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_multi_region_access_points::builders::ListMultiRegionAccessPointsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_multi_region_access_points::builders::ListMultiRegionAccessPointsFluentBuilder::set_next_token): <p>Not currently used. Do not use this parameter.</p>
    ///   - [`max_results(i32)`](crate::operation::list_multi_region_access_points::builders::ListMultiRegionAccessPointsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_multi_region_access_points::builders::ListMultiRegionAccessPointsFluentBuilder::set_max_results): <p>Not currently used. Do not use this parameter.</p>
    /// - On success, responds with [`ListMultiRegionAccessPointsOutput`](crate::operation::list_multi_region_access_points::ListMultiRegionAccessPointsOutput) with field(s):
    ///   - [`access_points(Option<Vec<MultiRegionAccessPointReport>>)`](crate::operation::list_multi_region_access_points::ListMultiRegionAccessPointsOutput::access_points): <p>The list of Multi-Region Access Points associated with the user.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_multi_region_access_points::ListMultiRegionAccessPointsOutput::next_token): <p>If the specified bucket has more Multi-Region Access Points than can be returned in one call to this action, this field contains a continuation token. You can use this token tin subsequent calls to this action to retrieve additional Multi-Region Access Points.</p>
    /// - On failure, responds with [`SdkError<ListMultiRegionAccessPointsError>`](crate::operation::list_multi_region_access_points::ListMultiRegionAccessPointsError)
    pub fn list_multi_region_access_points(&self) -> crate::operation::list_multi_region_access_points::builders::ListMultiRegionAccessPointsFluentBuilder{
        crate::operation::list_multi_region_access_points::builders::ListMultiRegionAccessPointsFluentBuilder::new(self.handle.clone())
    }
}
