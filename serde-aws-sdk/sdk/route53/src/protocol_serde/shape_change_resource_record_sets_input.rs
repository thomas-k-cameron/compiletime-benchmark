// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_change_resource_record_sets_input_input(
    input: &crate::operation::change_resource_record_sets::ChangeResourceRecordSetsInput,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.change_batch {
        let inner_writer = scope.start_el("ChangeBatch");
        crate::protocol_serde::shape_change_batch::ser_change_batch(var_1, inner_writer)?
    }
    scope.finish();
    Ok(())
}