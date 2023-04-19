// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_detach_network_interface_input_input(
    input: &crate::operation::detach_network_interface::DetachNetworkInterfaceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DetachNetworkInterface", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AttachmentId");
    if let Some(var_2) = &input.attachment_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DryRun");
    if let Some(var_4) = &input.dry_run {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Force");
    if let Some(var_6) = &input.force {
        scope_5.boolean(*var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
