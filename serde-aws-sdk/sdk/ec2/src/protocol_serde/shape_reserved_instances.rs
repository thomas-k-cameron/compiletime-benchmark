// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_reserved_instances(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ReservedInstances, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ReservedInstances::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("availabilityZone") /* AvailabilityZone com.amazonaws.ec2#ReservedInstances$AvailabilityZone */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_1);
            }
            ,
            s if s.matches("duration") /* Duration com.amazonaws.ec2#ReservedInstances$Duration */ =>  {
                let var_2 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.ec2#Long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_duration(var_2);
            }
            ,
            s if s.matches("end") /* End com.amazonaws.ec2#ReservedInstances$End */ =>  {
                let var_3 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_end(var_3);
            }
            ,
            s if s.matches("fixedPrice") /* FixedPrice com.amazonaws.ec2#ReservedInstances$FixedPrice */ =>  {
                let var_4 =
                    Some(
                         {
                            <f32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (float: `com.amazonaws.ec2#Float`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_fixed_price(var_4);
            }
            ,
            s if s.matches("instanceCount") /* InstanceCount com.amazonaws.ec2#ReservedInstances$InstanceCount */ =>  {
                let var_5 =
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
                builder = builder.set_instance_count(var_5);
            }
            ,
            s if s.matches("instanceType") /* InstanceType com.amazonaws.ec2#ReservedInstances$InstanceType */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::InstanceType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InstanceType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_type(var_6);
            }
            ,
            s if s.matches("productDescription") /* ProductDescription com.amazonaws.ec2#ReservedInstances$ProductDescription */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::RiProductDescription, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::RiProductDescription::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_product_description(var_7);
            }
            ,
            s if s.matches("reservedInstancesId") /* ReservedInstancesId com.amazonaws.ec2#ReservedInstances$ReservedInstancesId */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_reserved_instances_id(var_8);
            }
            ,
            s if s.matches("start") /* Start com.amazonaws.ec2#ReservedInstances$Start */ =>  {
                let var_9 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_start(var_9);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#ReservedInstances$State */ =>  {
                let var_10 =
                    Some(
                        Result::<crate::types::ReservedInstanceState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ReservedInstanceState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_10);
            }
            ,
            s if s.matches("usagePrice") /* UsagePrice com.amazonaws.ec2#ReservedInstances$UsagePrice */ =>  {
                let var_11 =
                    Some(
                         {
                            <f32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (float: `com.amazonaws.ec2#Float`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_usage_price(var_11);
            }
            ,
            s if s.matches("currencyCode") /* CurrencyCode com.amazonaws.ec2#ReservedInstances$CurrencyCode */ =>  {
                let var_12 =
                    Some(
                        Result::<crate::types::CurrencyCodeValues, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::CurrencyCodeValues::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_currency_code(var_12);
            }
            ,
            s if s.matches("instanceTenancy") /* InstanceTenancy com.amazonaws.ec2#ReservedInstances$InstanceTenancy */ =>  {
                let var_13 =
                    Some(
                        Result::<crate::types::Tenancy, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::Tenancy::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_tenancy(var_13);
            }
            ,
            s if s.matches("offeringClass") /* OfferingClass com.amazonaws.ec2#ReservedInstances$OfferingClass */ =>  {
                let var_14 =
                    Some(
                        Result::<crate::types::OfferingClassType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::OfferingClassType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_offering_class(var_14);
            }
            ,
            s if s.matches("offeringType") /* OfferingType com.amazonaws.ec2#ReservedInstances$OfferingType */ =>  {
                let var_15 =
                    Some(
                        Result::<crate::types::OfferingTypeValues, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::OfferingTypeValues::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_offering_type(var_15);
            }
            ,
            s if s.matches("recurringCharges") /* RecurringCharges com.amazonaws.ec2#ReservedInstances$RecurringCharges */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_recurring_charges_list::de_recurring_charges_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_recurring_charges(var_16);
            }
            ,
            s if s.matches("scope") /* Scope com.amazonaws.ec2#ReservedInstances$Scope */ =>  {
                let var_17 =
                    Some(
                        Result::<crate::types::Scope, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::Scope::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_scope(var_17);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#ReservedInstances$Tags */ =>  {
                let var_18 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_18);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
