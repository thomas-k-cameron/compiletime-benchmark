// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_account_settings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_account_settings::ListAccountSettingsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.value {
        object.key("value").string(var_2.as_str());
    }
    if let Some(var_3) = &input.principal_arn {
        object.key("principalArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.effective_settings {
        object.key("effectiveSettings").boolean(*var_4);
    }
    if let Some(var_5) = &input.next_token {
        object.key("nextToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    Ok(())
}
