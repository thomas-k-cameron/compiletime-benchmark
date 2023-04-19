// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetNetworkInsightsAccessScopeAnalysisFindings`](crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`network_insights_access_scope_analysis_id(impl Into<String>)`](crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder::network_insights_access_scope_analysis_id) / [`set_network_insights_access_scope_analysis_id(Option<String>)`](crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder::set_network_insights_access_scope_analysis_id): <p>The ID of the Network Access Scope analysis.</p>
    ///   - [`max_results(i32)`](crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`dry_run(bool)`](crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`GetNetworkInsightsAccessScopeAnalysisFindingsOutput`](crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput) with field(s):
    ///   - [`network_insights_access_scope_analysis_id(Option<String>)`](crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput::network_insights_access_scope_analysis_id): <p>The ID of the Network Access Scope analysis.</p>
    ///   - [`analysis_status(Option<AnalysisStatus>)`](crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput::analysis_status): <p>The status of Network Access Scope Analysis.</p>
    ///   - [`analysis_findings(Option<Vec<AccessScopeAnalysisFinding>>)`](crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput::analysis_findings): <p>The findings associated with Network Access Scope Analysis.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<GetNetworkInsightsAccessScopeAnalysisFindingsError>`](crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsError)
    pub fn get_network_insights_access_scope_analysis_findings(&self) -> crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder{
        crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsFluentBuilder::new(self.handle.clone())
    }
}
