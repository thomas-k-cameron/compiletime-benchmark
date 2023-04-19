// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeConformancePackCompliance`](crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`conformance_pack_name(impl Into<String>)`](crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder::conformance_pack_name) / [`set_conformance_pack_name(Option<String>)`](crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder::set_conformance_pack_name): <p>Name of the conformance pack.</p>
    ///   - [`filters(ConformancePackComplianceFilters)`](crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder::filters) / [`set_filters(Option<ConformancePackComplianceFilters>)`](crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder::set_filters): <p>A <code>ConformancePackComplianceFilters</code> object.</p>
    ///   - [`limit(i32)`](crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder::limit) / [`set_limit(i32)`](crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder::set_limit): <p>The maximum number of Config rules within a conformance pack are returned on each page.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder::set_next_token): <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    /// - On success, responds with [`DescribeConformancePackComplianceOutput`](crate::operation::describe_conformance_pack_compliance::DescribeConformancePackComplianceOutput) with field(s):
    ///   - [`conformance_pack_name(Option<String>)`](crate::operation::describe_conformance_pack_compliance::DescribeConformancePackComplianceOutput::conformance_pack_name): <p>Name of the conformance pack.</p>
    ///   - [`conformance_pack_rule_compliance_list(Option<Vec<ConformancePackRuleCompliance>>)`](crate::operation::describe_conformance_pack_compliance::DescribeConformancePackComplianceOutput::conformance_pack_rule_compliance_list): <p>Returns a list of <code>ConformancePackRuleCompliance</code> objects.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_conformance_pack_compliance::DescribeConformancePackComplianceOutput::next_token): <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    /// - On failure, responds with [`SdkError<DescribeConformancePackComplianceError>`](crate::operation::describe_conformance_pack_compliance::DescribeConformancePackComplianceError)
    pub fn describe_conformance_pack_compliance(&self) -> crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder{
        crate::operation::describe_conformance_pack_compliance::builders::DescribeConformancePackComplianceFluentBuilder::new(self.handle.clone())
    }
}