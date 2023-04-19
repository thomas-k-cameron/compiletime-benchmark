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
pub struct ListConformancePackComplianceScoresInput {
    /// <p>Filters the results based on the <code>ConformancePackComplianceScoresFilters</code>.</p>
    #[doc(hidden)]
    pub filters: std::option::Option<crate::types::ConformancePackComplianceScoresFilters>,
    /// <p>Determines the order in which conformance pack compliance scores are sorted. Either in ascending or descending order.</p>
    /// <p>By default, conformance pack compliance scores are sorted in alphabetical order by name of the conformance pack. Conformance pack compliance scores are sorted in reverse alphabetical order if you enter <code>DESCENDING</code>.</p>
    /// <p>You can sort conformance pack compliance scores by the numerical value of the compliance score by entering <code>SCORE</code> in the <code>SortBy</code> action. When compliance scores are sorted by <code>SCORE</code>, conformance packs with a compliance score of <code>INSUFFICIENT_DATA</code> will be last when sorting by ascending order and first when sorting by descending order.</p>
    #[doc(hidden)]
    pub sort_order: std::option::Option<crate::types::SortOrder>,
    /// <p>Sorts your conformance pack compliance scores in either ascending or descending order, depending on <code>SortOrder</code>.</p>
    /// <p>By default, conformance pack compliance scores are sorted in alphabetical order by name of the conformance pack. Enter <code>SCORE</code>, to sort conformance pack compliance scores by the numerical value of the compliance score.</p>
    #[doc(hidden)]
    pub sort_by: std::option::Option<crate::types::SortBy>,
    /// <p>The maximum number of conformance pack compliance scores returned on each page.</p>
    #[doc(hidden)]
    pub limit: i32,
    /// <p>The <code>nextToken</code> string in a prior request that you can use to get the paginated response for next set of conformance pack compliance scores.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListConformancePackComplianceScoresInput {
    /// <p>Filters the results based on the <code>ConformancePackComplianceScoresFilters</code>.</p>
    pub fn filters(
        &self,
    ) -> std::option::Option<&crate::types::ConformancePackComplianceScoresFilters> {
        self.filters.as_ref()
    }
    /// <p>Determines the order in which conformance pack compliance scores are sorted. Either in ascending or descending order.</p>
    /// <p>By default, conformance pack compliance scores are sorted in alphabetical order by name of the conformance pack. Conformance pack compliance scores are sorted in reverse alphabetical order if you enter <code>DESCENDING</code>.</p>
    /// <p>You can sort conformance pack compliance scores by the numerical value of the compliance score by entering <code>SCORE</code> in the <code>SortBy</code> action. When compliance scores are sorted by <code>SCORE</code>, conformance packs with a compliance score of <code>INSUFFICIENT_DATA</code> will be last when sorting by ascending order and first when sorting by descending order.</p>
    pub fn sort_order(&self) -> std::option::Option<&crate::types::SortOrder> {
        self.sort_order.as_ref()
    }
    /// <p>Sorts your conformance pack compliance scores in either ascending or descending order, depending on <code>SortOrder</code>.</p>
    /// <p>By default, conformance pack compliance scores are sorted in alphabetical order by name of the conformance pack. Enter <code>SCORE</code>, to sort conformance pack compliance scores by the numerical value of the compliance score.</p>
    pub fn sort_by(&self) -> std::option::Option<&crate::types::SortBy> {
        self.sort_by.as_ref()
    }
    /// <p>The maximum number of conformance pack compliance scores returned on each page.</p>
    pub fn limit(&self) -> i32 {
        self.limit
    }
    /// <p>The <code>nextToken</code> string in a prior request that you can use to get the paginated response for next set of conformance pack compliance scores.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListConformancePackComplianceScoresInput {
    /// Creates a new builder-style object to manufacture [`ListConformancePackComplianceScoresInput`](crate::operation::list_conformance_pack_compliance_scores::ListConformancePackComplianceScoresInput).
    pub fn builder() -> crate::operation::list_conformance_pack_compliance_scores::builders::ListConformancePackComplianceScoresInputBuilder{
        crate::operation::list_conformance_pack_compliance_scores::builders::ListConformancePackComplianceScoresInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_conformance_pack_compliance_scores::ListConformancePackComplianceScoresInput;
/// A builder for [`ListConformancePackComplianceScoresInput`](crate::operation::list_conformance_pack_compliance_scores::ListConformancePackComplianceScoresInput).
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
pub struct ListConformancePackComplianceScoresInputBuilder {
    pub(crate) filters: std::option::Option<crate::types::ConformancePackComplianceScoresFilters>,
    pub(crate) sort_order: std::option::Option<crate::types::SortOrder>,
    pub(crate) sort_by: std::option::Option<crate::types::SortBy>,
    pub(crate) limit: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl ListConformancePackComplianceScoresInputBuilder {
    /// <p>Filters the results based on the <code>ConformancePackComplianceScoresFilters</code>.</p>
    pub fn filters(mut self, input: crate::types::ConformancePackComplianceScoresFilters) -> Self {
        self.filters = Some(input);
        self
    }
    /// <p>Filters the results based on the <code>ConformancePackComplianceScoresFilters</code>.</p>
    pub fn set_filters(
        mut self,
        input: std::option::Option<crate::types::ConformancePackComplianceScoresFilters>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>Determines the order in which conformance pack compliance scores are sorted. Either in ascending or descending order.</p>
    /// <p>By default, conformance pack compliance scores are sorted in alphabetical order by name of the conformance pack. Conformance pack compliance scores are sorted in reverse alphabetical order if you enter <code>DESCENDING</code>.</p>
    /// <p>You can sort conformance pack compliance scores by the numerical value of the compliance score by entering <code>SCORE</code> in the <code>SortBy</code> action. When compliance scores are sorted by <code>SCORE</code>, conformance packs with a compliance score of <code>INSUFFICIENT_DATA</code> will be last when sorting by ascending order and first when sorting by descending order.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.sort_order = Some(input);
        self
    }
    /// <p>Determines the order in which conformance pack compliance scores are sorted. Either in ascending or descending order.</p>
    /// <p>By default, conformance pack compliance scores are sorted in alphabetical order by name of the conformance pack. Conformance pack compliance scores are sorted in reverse alphabetical order if you enter <code>DESCENDING</code>.</p>
    /// <p>You can sort conformance pack compliance scores by the numerical value of the compliance score by entering <code>SCORE</code> in the <code>SortBy</code> action. When compliance scores are sorted by <code>SCORE</code>, conformance packs with a compliance score of <code>INSUFFICIENT_DATA</code> will be last when sorting by ascending order and first when sorting by descending order.</p>
    pub fn set_sort_order(mut self, input: std::option::Option<crate::types::SortOrder>) -> Self {
        self.sort_order = input;
        self
    }
    /// <p>Sorts your conformance pack compliance scores in either ascending or descending order, depending on <code>SortOrder</code>.</p>
    /// <p>By default, conformance pack compliance scores are sorted in alphabetical order by name of the conformance pack. Enter <code>SCORE</code>, to sort conformance pack compliance scores by the numerical value of the compliance score.</p>
    pub fn sort_by(mut self, input: crate::types::SortBy) -> Self {
        self.sort_by = Some(input);
        self
    }
    /// <p>Sorts your conformance pack compliance scores in either ascending or descending order, depending on <code>SortOrder</code>.</p>
    /// <p>By default, conformance pack compliance scores are sorted in alphabetical order by name of the conformance pack. Enter <code>SCORE</code>, to sort conformance pack compliance scores by the numerical value of the compliance score.</p>
    pub fn set_sort_by(mut self, input: std::option::Option<crate::types::SortBy>) -> Self {
        self.sort_by = input;
        self
    }
    /// <p>The maximum number of conformance pack compliance scores returned on each page.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>The maximum number of conformance pack compliance scores returned on each page.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>The <code>nextToken</code> string in a prior request that you can use to get the paginated response for next set of conformance pack compliance scores.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string in a prior request that you can use to get the paginated response for next set of conformance pack compliance scores.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListConformancePackComplianceScoresInput`](crate::operation::list_conformance_pack_compliance_scores::ListConformancePackComplianceScoresInput).
    pub fn build(self) -> Result<crate::operation::list_conformance_pack_compliance_scores::ListConformancePackComplianceScoresInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::list_conformance_pack_compliance_scores::ListConformancePackComplianceScoresInput {
                filters: self.filters
                ,
                sort_order: self.sort_order
                ,
                sort_by: self.sort_by
                ,
                limit: self.limit
                    .unwrap_or_default()
                ,
                next_token: self.next_token
                ,
            }
        )
    }
}