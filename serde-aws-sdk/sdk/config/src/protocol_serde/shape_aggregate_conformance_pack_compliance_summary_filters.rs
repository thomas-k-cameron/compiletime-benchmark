// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aggregate_conformance_pack_compliance_summary_filters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AggregateConformancePackComplianceSummaryFilters,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("AccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.aws_region {
        object.key("AwsRegion").string(var_2.as_str());
    }
    Ok(())
}
