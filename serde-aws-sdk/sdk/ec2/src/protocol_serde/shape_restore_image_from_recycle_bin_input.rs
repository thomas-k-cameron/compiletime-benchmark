// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_restore_image_from_recycle_bin_input_input(
    input: &crate::operation::restore_image_from_recycle_bin::RestoreImageFromRecycleBinInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "RestoreImageFromRecycleBin", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ImageId");
    if let Some(var_2) = &input.image_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DryRun");
    if let Some(var_4) = &input.dry_run {
        scope_3.boolean(*var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
