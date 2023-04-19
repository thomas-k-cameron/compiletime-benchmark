// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_tables_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_tables::ListTablesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.exclusive_start_table_name {
        object.key("ExclusiveStartTableName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    Ok(())
}
