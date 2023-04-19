// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_get_item_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::batch_get_item::BatchGetItemInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.request_items {
        #[allow(unused_mut)]
        let mut object_2 = object.key("RequestItems").start_object();
        for (key_3, value_4) in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_5 = object_2.key(key_3.as_str()).start_object();
                crate::protocol_serde::shape_keys_and_attributes::ser_keys_and_attributes(
                    &mut object_5,
                    value_4,
                )?;
                object_5.finish();
            }
        }
        object_2.finish();
    }
    if let Some(var_6) = &input.return_consumed_capacity {
        object.key("ReturnConsumedCapacity").string(var_6.as_str());
    }
    Ok(())
}