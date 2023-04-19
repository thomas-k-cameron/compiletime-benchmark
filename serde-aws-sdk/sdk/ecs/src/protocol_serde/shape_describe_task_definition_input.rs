// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_task_definition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_task_definition::DescribeTaskDefinitionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.task_definition {
        object.key("taskDefinition").string(var_1.as_str());
    }
    if let Some(var_2) = &input.include {
        let mut array_3 = object.key("include").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}
