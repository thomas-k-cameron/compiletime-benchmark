// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_key_description_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_key_description::UpdateKeyDescriptionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.key_id {
        object.key("KeyId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    Ok(())
}
