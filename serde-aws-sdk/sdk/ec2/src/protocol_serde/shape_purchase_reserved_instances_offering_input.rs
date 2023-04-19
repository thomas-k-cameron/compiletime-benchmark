// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_purchase_reserved_instances_offering_input_input(
    input: &crate::operation::purchase_reserved_instances_offering::PurchaseReservedInstancesOfferingInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(
        &mut out,
        "PurchaseReservedInstancesOffering",
        "2016-11-15",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("InstanceCount");
    if let Some(var_2) = &input.instance_count {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ReservedInstancesOfferingId");
    if let Some(var_4) = &input.reserved_instances_offering_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DryRun");
    if let Some(var_6) = &input.dry_run {
        scope_5.boolean(*var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("LimitPrice");
    if let Some(var_8) = &input.limit_price {
        crate::protocol_serde::shape_reserved_instance_limit_price::ser_reserved_instance_limit_price(scope_7, var_8)?;
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("PurchaseTime");
    if let Some(var_10) = &input.purchase_time {
        scope_9.date_time(var_10, aws_smithy_types::date_time::Format::DateTime)?;
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
