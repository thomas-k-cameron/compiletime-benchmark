// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_condition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Condition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.attribute_value_list {
        let mut array_2 = object.key("AttributeValueList").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_attribute_value::ser_attribute_value(
                    &mut object_4,
                    item_3,
                )?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.comparison_operator {
        object.key("ComparisonOperator").string(var_5.as_str());
    }
    Ok(())
}