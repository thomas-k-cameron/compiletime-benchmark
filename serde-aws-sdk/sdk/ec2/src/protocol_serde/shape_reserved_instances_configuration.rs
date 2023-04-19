// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_reserved_instances_configuration(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::ReservedInstancesConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AvailabilityZone");
    if let Some(var_2) = &input.availability_zone {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("InstanceCount");
    if let Some(var_4) = &input.instance_count {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("InstanceType");
    if let Some(var_6) = &input.instance_type {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Platform");
    if let Some(var_8) = &input.platform {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Scope");
    if let Some(var_10) = &input.scope {
        scope_9.string(var_10.as_str());
    }
    Ok(())
}

pub fn de_reserved_instances_configuration(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ReservedInstancesConfiguration, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ReservedInstancesConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("availabilityZone") /* AvailabilityZone com.amazonaws.ec2#ReservedInstancesConfiguration$AvailabilityZone */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_11);
            }
            ,
            s if s.matches("instanceCount") /* InstanceCount com.amazonaws.ec2#ReservedInstancesConfiguration$InstanceCount */ =>  {
                let var_12 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_instance_count(var_12);
            }
            ,
            s if s.matches("instanceType") /* InstanceType com.amazonaws.ec2#ReservedInstancesConfiguration$InstanceType */ =>  {
                let var_13 =
                    Some(
                        Result::<crate::types::InstanceType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InstanceType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_type(var_13);
            }
            ,
            s if s.matches("platform") /* Platform com.amazonaws.ec2#ReservedInstancesConfiguration$Platform */ =>  {
                let var_14 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_platform(var_14);
            }
            ,
            s if s.matches("scope") /* Scope com.amazonaws.ec2#ReservedInstancesConfiguration$Scope */ =>  {
                let var_15 =
                    Some(
                        Result::<crate::types::Scope, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::Scope::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_scope(var_15);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
