// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_virtual_mfa_device_input_input(
    input: &crate::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "CreateVirtualMFADevice", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Path");
    if let Some(var_2) = &input.path {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VirtualMFADeviceName");
    if let Some(var_4) = &input.virtual_mfa_device_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Tags");
    if let Some(var_6) = &input.tags {
        let mut list_8 = scope_5.start_list(false, None);
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_9, item_7)?;
        }
        list_8.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}