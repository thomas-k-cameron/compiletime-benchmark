// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resource_evaluation_filters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ResourceEvaluationFilters,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.evaluation_mode {
        object.key("EvaluationMode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.time_window {
        #[allow(unused_mut)]
        let mut object_3 = object.key("TimeWindow").start_object();
        crate::protocol_serde::shape_time_window::ser_time_window(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.evaluation_context_identifier {
        object
            .key("EvaluationContextIdentifier")
            .string(var_4.as_str());
    }
    Ok(())
}