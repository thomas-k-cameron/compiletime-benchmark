// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_total_local_storage_gb_request(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::TotalLocalStorageGbRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Min");
    if let Some(var_2) = &input.min {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Max");
    if let Some(var_4) = &input.max {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_4).into()),
        );
    }
    Ok(())
}
