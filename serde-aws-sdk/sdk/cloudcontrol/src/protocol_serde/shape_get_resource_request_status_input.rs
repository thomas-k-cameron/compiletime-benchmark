// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_resource_request_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_resource_request_status::GetResourceRequestStatusInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.request_token {
        object.key("RequestToken").string(var_1.as_str());
    }
    Ok(())
}