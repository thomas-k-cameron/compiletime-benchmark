// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_bucket_lifecycle_configuration(
    input: &crate::types::BucketLifecycleConfiguration,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.rules {
        for list_item_2 in var_1 {
            {
                let inner_writer = scope.start_el("Rule");
                crate::protocol_serde::shape_lifecycle_rule::ser_lifecycle_rule(
                    list_item_2,
                    inner_writer,
                )?
            }
        }
    }
    scope.finish();
    Ok(())
}
