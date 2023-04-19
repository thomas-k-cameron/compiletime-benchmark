// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_ssh_public_key_input_input(
    input: &crate::operation::delete_ssh_public_key::DeleteSshPublicKeyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DeleteSSHPublicKey", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("UserName");
    if let Some(var_2) = &input.user_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SSHPublicKeyId");
    if let Some(var_4) = &input.ssh_public_key_id {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
