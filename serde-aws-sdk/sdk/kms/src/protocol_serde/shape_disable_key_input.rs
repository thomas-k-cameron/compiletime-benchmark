// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disable_key_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::disable_key::DisableKeyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.key_id {
        object.key("KeyId").string(var_1.as_str());
    }
    Ok(())
}
