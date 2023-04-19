// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transact_write_items_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::transact_write_items::TransactWriteItemsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.transact_items {
        let mut array_2 = object.key("TransactItems").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_transact_write_item::ser_transact_write_item(
                    &mut object_4,
                    item_3,
                )?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.return_consumed_capacity {
        object.key("ReturnConsumedCapacity").string(var_5.as_str());
    }
    if let Some(var_6) = &input.return_item_collection_metrics {
        object
            .key("ReturnItemCollectionMetrics")
            .string(var_6.as_str());
    }
    if let Some(var_7) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_7.as_str());
    }
    Ok(())
}