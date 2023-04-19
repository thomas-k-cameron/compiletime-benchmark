// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_select_object_content_input_input(
    input: &crate::operation::select_object_content::SelectObjectContentInput,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.expression {
        let mut inner_writer = scope.start_el("Expression").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.expression_type {
        let mut inner_writer = scope.start_el("ExpressionType").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.input_serialization {
        let inner_writer = scope.start_el("InputSerialization");
        crate::protocol_serde::shape_input_serialization::ser_input_serialization(
            var_3,
            inner_writer,
        )?
    }
    if let Some(var_4) = &input.output_serialization {
        let inner_writer = scope.start_el("OutputSerialization");
        crate::protocol_serde::shape_output_serialization::ser_output_serialization(
            var_4,
            inner_writer,
        )?
    }
    if let Some(var_5) = &input.request_progress {
        let inner_writer = scope.start_el("RequestProgress");
        crate::protocol_serde::shape_request_progress::ser_request_progress(var_5, inner_writer)?
    }
    if let Some(var_6) = &input.scan_range {
        let inner_writer = scope.start_el("ScanRange");
        crate::protocol_serde::shape_scan_range::ser_scan_range(var_6, inner_writer)?
    }
    scope.finish();
    Ok(())
}
