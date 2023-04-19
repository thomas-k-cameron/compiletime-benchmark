// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_hosted_zone_input_input(
    input: &crate::operation::create_hosted_zone::CreateHostedZoneInput,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.caller_reference {
        let mut inner_writer = scope.start_el("CallerReference").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.delegation_set_id {
        let mut inner_writer = scope.start_el("DelegationSetId").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.hosted_zone_config {
        let inner_writer = scope.start_el("HostedZoneConfig");
        crate::protocol_serde::shape_hosted_zone_config::ser_hosted_zone_config(
            var_3,
            inner_writer,
        )?
    }
    if let Some(var_4) = &input.name {
        let mut inner_writer = scope.start_el("Name").finish();
        inner_writer.data(var_4.as_str());
    }
    if let Some(var_5) = &input.vpc {
        let inner_writer = scope.start_el("VPC");
        crate::protocol_serde::shape_vpc::ser_vpc(var_5, inner_writer)?
    }
    scope.finish();
    Ok(())
}
