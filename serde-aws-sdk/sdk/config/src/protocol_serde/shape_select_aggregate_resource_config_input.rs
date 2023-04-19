// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_select_aggregate_resource_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::select_aggregate_resource_config::SelectAggregateResourceConfigInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.expression {
        object.key("Expression").string(var_1.as_str());
    }
    if let Some(var_2) = &input.configuration_aggregator_name {
        object
            .key("ConfigurationAggregatorName")
            .string(var_2.as_str());
    }
    if input.limit != 0 {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.limit).into()),
        );
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    Ok(())
}
