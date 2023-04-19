// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListReusableDelegationSets`](crate::operation::list_reusable_delegation_sets::builders::ListReusableDelegationSetsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`marker(impl Into<String>)`](crate::operation::list_reusable_delegation_sets::builders::ListReusableDelegationSetsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_reusable_delegation_sets::builders::ListReusableDelegationSetsFluentBuilder::set_marker): <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more reusable delegation sets. To get another group, submit another <code>ListReusableDelegationSets</code> request. </p>  <p>For the value of <code>marker</code>, specify the value of <code>NextMarker</code> from the previous response, which is the ID of the first reusable delegation set that Amazon Route 53 will return if you submit another request.</p>  <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more reusable delegation sets to get.</p>
    ///   - [`max_items(i32)`](crate::operation::list_reusable_delegation_sets::builders::ListReusableDelegationSetsFluentBuilder::max_items) / [`set_max_items(Option<i32>)`](crate::operation::list_reusable_delegation_sets::builders::ListReusableDelegationSetsFluentBuilder::set_max_items): <p>The number of reusable delegation sets that you want Amazon Route 53 to return in the response to this request. If you specify a value greater than 100, Route 53 returns only the first 100 reusable delegation sets.</p>
    /// - On success, responds with [`ListReusableDelegationSetsOutput`](crate::operation::list_reusable_delegation_sets::ListReusableDelegationSetsOutput) with field(s):
    ///   - [`delegation_sets(Option<Vec<DelegationSet>>)`](crate::operation::list_reusable_delegation_sets::ListReusableDelegationSetsOutput::delegation_sets): <p>A complex type that contains one <code>DelegationSet</code> element for each reusable delegation set that was created by the current Amazon Web Services account.</p>
    ///   - [`marker(Option<String>)`](crate::operation::list_reusable_delegation_sets::ListReusableDelegationSetsOutput::marker): <p>For the second and subsequent calls to <code>ListReusableDelegationSets</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the request that produced the current response.</p>
    ///   - [`is_truncated(bool)`](crate::operation::list_reusable_delegation_sets::ListReusableDelegationSetsOutput::is_truncated): <p>A flag that indicates whether there are more reusable delegation sets to be listed.</p>
    ///   - [`next_marker(Option<String>)`](crate::operation::list_reusable_delegation_sets::ListReusableDelegationSetsOutput::next_marker): <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the next reusable delegation set that Amazon Route 53 will return if you submit another <code>ListReusableDelegationSets</code> request and specify the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    ///   - [`max_items(Option<i32>)`](crate::operation::list_reusable_delegation_sets::ListReusableDelegationSetsOutput::max_items): <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListReusableDelegationSets</code> that produced the current response.</p>
    /// - On failure, responds with [`SdkError<ListReusableDelegationSetsError>`](crate::operation::list_reusable_delegation_sets::ListReusableDelegationSetsError)
    pub fn list_reusable_delegation_sets(&self) -> crate::operation::list_reusable_delegation_sets::builders::ListReusableDelegationSetsFluentBuilder{
        crate::operation::list_reusable_delegation_sets::builders::ListReusableDelegationSetsFluentBuilder::new(self.handle.clone())
    }
}
