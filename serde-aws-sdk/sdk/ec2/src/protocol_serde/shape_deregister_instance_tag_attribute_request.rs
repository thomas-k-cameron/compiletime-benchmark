// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_deregister_instance_tag_attribute_request(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::DeregisterInstanceTagAttributeRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("IncludeAllTagsOfInstance");
    if let Some(var_2) = &input.include_all_tags_of_instance {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("InstanceTagKey");
    if let Some(var_4) = &input.instance_tag_keys {
        let mut list_6 = scope_3.start_list(true, Some("item"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    Ok(())
}
