// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_view_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_view::CreateViewInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filters {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Filters").start_object();
        crate::protocol_serde::shape_search_filter::ser_search_filter(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.included_properties {
        let mut array_5 = object.key("IncludedProperties").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_included_property::ser_included_property(
                    &mut object_7,
                    item_6,
                )?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.tags {
        #[allow(unused_mut)]
        let mut object_9 = object.key("Tags").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    if let Some(var_12) = &input.view_name {
        object.key("ViewName").string(var_12.as_str());
    }
    Ok(())
}
