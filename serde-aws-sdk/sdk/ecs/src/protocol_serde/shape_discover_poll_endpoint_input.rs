// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_discover_poll_endpoint_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::discover_poll_endpoint::DiscoverPollEndpointInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.container_instance {
        object.key("containerInstance").string(var_1.as_str());
    }
    if let Some(var_2) = &input.cluster {
        object.key("cluster").string(var_2.as_str());
    }
    Ok(())
}