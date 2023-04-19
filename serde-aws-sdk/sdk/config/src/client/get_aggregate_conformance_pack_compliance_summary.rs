// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAggregateConformancePackComplianceSummary`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_aggregator_name(impl Into<String>)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::configuration_aggregator_name) / [`set_configuration_aggregator_name(Option<String>)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::set_configuration_aggregator_name): <p>The name of the configuration aggregator.</p>
    ///   - [`filters(AggregateConformancePackComplianceSummaryFilters)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::filters) / [`set_filters(Option<AggregateConformancePackComplianceSummaryFilters>)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::set_filters): <p>Filters the results based on the <code>AggregateConformancePackComplianceSummaryFilters</code> object.</p>
    ///   - [`group_by_key(AggregateConformancePackComplianceSummaryGroupKey)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::group_by_key) / [`set_group_by_key(Option<AggregateConformancePackComplianceSummaryGroupKey>)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::set_group_by_key): <p>Groups the result based on Amazon Web Services account ID or Amazon Web Services Region.</p>
    ///   - [`limit(i32)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::limit) / [`set_limit(i32)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::set_limit): <p>The maximum number of results returned on each page. The default is maximum. If you specify 0, Config uses the default.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::set_next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    /// - On success, responds with [`GetAggregateConformancePackComplianceSummaryOutput`](crate::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryOutput) with field(s):
    ///   - [`aggregate_conformance_pack_compliance_summaries(Option<Vec<AggregateConformancePackComplianceSummary>>)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryOutput::aggregate_conformance_pack_compliance_summaries): <p>Returns a list of <code>AggregateConformancePackComplianceSummary</code> object.</p>
    ///   - [`group_by_key(Option<String>)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryOutput::group_by_key): <p>Groups the result based on Amazon Web Services account ID or Amazon Web Services Region.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryOutput::next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    /// - On failure, responds with [`SdkError<GetAggregateConformancePackComplianceSummaryError>`](crate::operation::get_aggregate_conformance_pack_compliance_summary::GetAggregateConformancePackComplianceSummaryError)
    pub fn get_aggregate_conformance_pack_compliance_summary(&self) -> crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder{
        crate::operation::get_aggregate_conformance_pack_compliance_summary::builders::GetAggregateConformancePackComplianceSummaryFluentBuilder::new(self.handle.clone())
    }
}
