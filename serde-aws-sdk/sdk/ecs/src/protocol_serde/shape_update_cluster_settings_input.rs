// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_cluster_settings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_cluster_settings::UpdateClusterSettingsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster {
        object.key("cluster").string(var_1.as_str());
    }
    if let Some(var_2) = &input.settings {
        let mut array_3 = object.key("settings").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_cluster_setting::ser_cluster_setting(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}
