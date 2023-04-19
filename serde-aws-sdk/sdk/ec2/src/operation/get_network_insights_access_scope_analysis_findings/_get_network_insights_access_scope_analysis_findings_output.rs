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
pub struct GetNetworkInsightsAccessScopeAnalysisFindingsOutput {
    /// <p>The ID of the Network Access Scope analysis.</p>
    #[doc(hidden)]
    pub network_insights_access_scope_analysis_id: std::option::Option<std::string::String>,
    /// <p>The status of Network Access Scope Analysis.</p>
    #[doc(hidden)]
    pub analysis_status: std::option::Option<crate::types::AnalysisStatus>,
    /// <p>The findings associated with Network Access Scope Analysis.</p>
    #[doc(hidden)]
    pub analysis_findings:
        std::option::Option<std::vec::Vec<crate::types::AccessScopeAnalysisFinding>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetNetworkInsightsAccessScopeAnalysisFindingsOutput {
    /// <p>The ID of the Network Access Scope analysis.</p>
    pub fn network_insights_access_scope_analysis_id(&self) -> std::option::Option<&str> {
        self.network_insights_access_scope_analysis_id.as_deref()
    }
    /// <p>The status of Network Access Scope Analysis.</p>
    pub fn analysis_status(&self) -> std::option::Option<&crate::types::AnalysisStatus> {
        self.analysis_status.as_ref()
    }
    /// <p>The findings associated with Network Access Scope Analysis.</p>
    pub fn analysis_findings(
        &self,
    ) -> std::option::Option<&[crate::types::AccessScopeAnalysisFinding]> {
        self.analysis_findings.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetNetworkInsightsAccessScopeAnalysisFindingsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetNetworkInsightsAccessScopeAnalysisFindingsOutput {
    /// Creates a new builder-style object to manufacture [`GetNetworkInsightsAccessScopeAnalysisFindingsOutput`](crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput).
    pub fn builder() -> crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsOutputBuilder{
        crate::operation::get_network_insights_access_scope_analysis_findings::builders::GetNetworkInsightsAccessScopeAnalysisFindingsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput;
/// A builder for [`GetNetworkInsightsAccessScopeAnalysisFindingsOutput`](crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput).
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
pub struct GetNetworkInsightsAccessScopeAnalysisFindingsOutputBuilder {
    pub(crate) network_insights_access_scope_analysis_id: std::option::Option<std::string::String>,
    pub(crate) analysis_status: std::option::Option<crate::types::AnalysisStatus>,
    pub(crate) analysis_findings:
        std::option::Option<std::vec::Vec<crate::types::AccessScopeAnalysisFinding>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetNetworkInsightsAccessScopeAnalysisFindingsOutputBuilder {
    /// <p>The ID of the Network Access Scope analysis.</p>
    pub fn network_insights_access_scope_analysis_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.network_insights_access_scope_analysis_id = Some(input.into());
        self
    }
    /// <p>The ID of the Network Access Scope analysis.</p>
    pub fn set_network_insights_access_scope_analysis_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_insights_access_scope_analysis_id = input;
        self
    }
    /// <p>The status of Network Access Scope Analysis.</p>
    pub fn analysis_status(mut self, input: crate::types::AnalysisStatus) -> Self {
        self.analysis_status = Some(input);
        self
    }
    /// <p>The status of Network Access Scope Analysis.</p>
    pub fn set_analysis_status(
        mut self,
        input: std::option::Option<crate::types::AnalysisStatus>,
    ) -> Self {
        self.analysis_status = input;
        self
    }
    /// Appends an item to `analysis_findings`.
    ///
    /// To override the contents of this collection use [`set_analysis_findings`](Self::set_analysis_findings).
    ///
    /// <p>The findings associated with Network Access Scope Analysis.</p>
    pub fn analysis_findings(mut self, input: crate::types::AccessScopeAnalysisFinding) -> Self {
        let mut v = self.analysis_findings.unwrap_or_default();
        v.push(input);
        self.analysis_findings = Some(v);
        self
    }
    /// <p>The findings associated with Network Access Scope Analysis.</p>
    pub fn set_analysis_findings(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AccessScopeAnalysisFinding>>,
    ) -> Self {
        self.analysis_findings = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetNetworkInsightsAccessScopeAnalysisFindingsOutput`](crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput).
    pub fn build(self) -> crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput{
        crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsOutput {
            network_insights_access_scope_analysis_id: self.network_insights_access_scope_analysis_id
            ,
            analysis_status: self.analysis_status
            ,
            analysis_findings: self.analysis_findings
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}