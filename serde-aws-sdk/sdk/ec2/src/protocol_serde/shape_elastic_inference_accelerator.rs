// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_elastic_inference_accelerator(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::ElasticInferenceAccelerator,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Type");
    if let Some(var_2) = &input.r#type {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Count");
    if let Some(var_4) = &input.count {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    Ok(())
}
