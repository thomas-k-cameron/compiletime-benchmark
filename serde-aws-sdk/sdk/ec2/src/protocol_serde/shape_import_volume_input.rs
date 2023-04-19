// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_volume_input_input(
    input: &crate::operation::import_volume::ImportVolumeInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ImportVolume", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AvailabilityZone");
    if let Some(var_2) = &input.availability_zone {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Description");
    if let Some(var_4) = &input.description {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DryRun");
    if let Some(var_6) = &input.dry_run {
        scope_5.boolean(*var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Image");
    if let Some(var_8) = &input.image {
        crate::protocol_serde::shape_disk_image_detail::ser_disk_image_detail(scope_7, var_8)?;
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Volume");
    if let Some(var_10) = &input.volume {
        crate::protocol_serde::shape_volume_detail::ser_volume_detail(scope_9, var_10)?;
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
