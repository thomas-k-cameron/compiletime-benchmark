// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_attributes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_attributes::PutAttributesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster {
        object.key("cluster").string(var_1.as_str());
    }
    if let Some(var_2) = &input.attributes {
        let mut array_3 = object.key("attributes").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_attribute::ser_attribute(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}