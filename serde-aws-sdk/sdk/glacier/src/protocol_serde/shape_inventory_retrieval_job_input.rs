// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_inventory_retrieval_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::InventoryRetrievalJobInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.start_date {
        object.key("StartDate").string(var_1.as_str());
    }
    if let Some(var_2) = &input.end_date {
        object.key("EndDate").string(var_2.as_str());
    }
    if let Some(var_3) = &input.limit {
        object.key("Limit").string(var_3.as_str());
    }
    if let Some(var_4) = &input.marker {
        object.key("Marker").string(var_4.as_str());
    }
    Ok(())
}