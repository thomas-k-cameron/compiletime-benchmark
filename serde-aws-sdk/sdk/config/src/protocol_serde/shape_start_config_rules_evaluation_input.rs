// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_config_rules_evaluation_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.config_rule_names {
        let mut array_2 = object.key("ConfigRuleNames").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}
