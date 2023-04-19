// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetComplianceDetailsByResource`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_type(impl Into<String>)`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::resource_type) / [`set_resource_type(Option<String>)`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::set_resource_type): <p>The type of the Amazon Web Services resource for which you want compliance information.</p>
    ///   - [`resource_id(impl Into<String>)`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::set_resource_id): <p>The ID of the Amazon Web Services resource for which you want compliance information.</p>
    ///   - [`compliance_types(Vec<ComplianceType>)`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::compliance_types) / [`set_compliance_types(Option<Vec<ComplianceType>>)`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::set_compliance_types): <p>Filters the results by compliance.</p>  <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::set_next_token): <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    ///   - [`resource_evaluation_id(impl Into<String>)`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::resource_evaluation_id) / [`set_resource_evaluation_id(Option<String>)`](crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::set_resource_evaluation_id): <p>The unique ID of Amazon Web Services resource execution for which you want to retrieve evaluation results. </p> <note>   <p>You need to only provide either a <code>ResourceEvaluationID</code> or a <code>ResourceID </code>and <code>ResourceType</code>.</p>  </note>
    /// - On success, responds with [`GetComplianceDetailsByResourceOutput`](crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceOutput) with field(s):
    ///   - [`evaluation_results(Option<Vec<EvaluationResult>>)`](crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceOutput::evaluation_results): <p>Indicates whether the specified Amazon Web Services resource complies each Config rule.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceOutput::next_token): <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    /// - On failure, responds with [`SdkError<GetComplianceDetailsByResourceError>`](crate::operation::get_compliance_details_by_resource::GetComplianceDetailsByResourceError)
    pub fn get_compliance_details_by_resource(&self) -> crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder{
        crate::operation::get_compliance_details_by_resource::builders::GetComplianceDetailsByResourceFluentBuilder::new(self.handle.clone())
    }
}
