// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_resource_requests_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_resource_requests::ListResourceRequestsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.next_token {
        object.key("NextToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.resource_request_status_filter {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ResourceRequestStatusFilter").start_object();
        crate::protocol_serde::shape_resource_request_status_filter::ser_resource_request_status_filter(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}