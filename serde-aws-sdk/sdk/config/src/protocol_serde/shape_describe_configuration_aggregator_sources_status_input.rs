// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_configuration_aggregator_sources_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_configuration_aggregator_sources_status::DescribeConfigurationAggregatorSourcesStatusInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.configuration_aggregator_name {
        object
            .key("ConfigurationAggregatorName")
            .string(var_1.as_str());
    }
    if let Some(var_2) = &input.update_status {
        let mut array_3 = object.key("UpdateStatus").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.next_token {
        object.key("NextToken").string(var_5.as_str());
    }
    if input.limit != 0 {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.limit).into()),
        );
    }
    Ok(())
}
