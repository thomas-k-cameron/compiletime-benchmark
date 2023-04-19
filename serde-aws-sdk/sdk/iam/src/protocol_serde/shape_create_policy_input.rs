// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_policy_input_input(
    input: &crate::operation::create_policy::CreatePolicyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreatePolicy", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("PolicyName");
    if let Some(var_2) = &input.policy_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Path");
    if let Some(var_4) = &input.path {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PolicyDocument");
    if let Some(var_6) = &input.policy_document {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Description");
    if let Some(var_8) = &input.description {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Tags");
    if let Some(var_10) = &input.tags {
        let mut list_12 = scope_9.start_list(false, None);
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_13, item_11)?;
        }
        list_12.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
