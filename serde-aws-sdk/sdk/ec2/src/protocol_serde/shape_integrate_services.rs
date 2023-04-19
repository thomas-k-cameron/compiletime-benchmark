// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_integrate_services(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::IntegrateServices,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AthenaIntegration");
    if let Some(var_2) = &input.athena_integrations {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            crate::protocol_serde::shape_athena_integration::ser_athena_integration(
                entry_5, item_3,
            )?;
        }
        list_4.finish();
    }
    Ok(())
}
