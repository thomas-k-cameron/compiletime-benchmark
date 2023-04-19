// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_fast_launch_snapshot_configuration_request(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::FastLaunchSnapshotConfigurationRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TargetResourceCount");
    if let Some(var_2) = &input.target_resource_count {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    Ok(())
}
