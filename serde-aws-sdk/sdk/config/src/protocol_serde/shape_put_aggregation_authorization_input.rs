// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_aggregation_authorization_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_aggregation_authorization::PutAggregationAuthorizationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.authorized_account_id {
        object.key("AuthorizedAccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.authorized_aws_region {
        object.key("AuthorizedAwsRegion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tags {
        let mut array_4 = object.key("Tags").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}
