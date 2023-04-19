// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_pending_aggregation_request_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_pending_aggregation_request::DeletePendingAggregationRequestInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.requester_account_id {
        object.key("RequesterAccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.requester_aws_region {
        object.key("RequesterAwsRegion").string(var_2.as_str());
    }
    Ok(())
}
