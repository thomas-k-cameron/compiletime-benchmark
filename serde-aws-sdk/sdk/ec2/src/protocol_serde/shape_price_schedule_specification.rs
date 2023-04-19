// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_price_schedule_specification(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::PriceScheduleSpecification,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CurrencyCode");
    if let Some(var_2) = &input.currency_code {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Price");
    if let Some(var_4) = &input.price {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Term");
    if let Some(var_6) = &input.term {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    Ok(())
}
