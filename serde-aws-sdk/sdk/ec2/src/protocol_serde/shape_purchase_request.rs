// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_purchase_request(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::PurchaseRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("InstanceCount");
    if let Some(var_2) = &input.instance_count {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PurchaseToken");
    if let Some(var_4) = &input.purchase_token {
        scope_3.string(var_4);
    }
    Ok(())
}
