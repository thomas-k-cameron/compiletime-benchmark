// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_kinesis_streaming_destination_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.table_name {
        object.key("TableName").string(var_1.as_str());
    }
    Ok(())
}