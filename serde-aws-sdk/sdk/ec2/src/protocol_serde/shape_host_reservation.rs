// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_host_reservation(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::HostReservation, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::HostReservation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("count") /* Count com.amazonaws.ec2#HostReservation$Count */ =>  {
                let var_1 =
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
                builder = builder.set_count(var_1);
            }
            ,
            s if s.matches("currencyCode") /* CurrencyCode com.amazonaws.ec2#HostReservation$CurrencyCode */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::CurrencyCodeValues, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::CurrencyCodeValues::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_currency_code(var_2);
            }
            ,
            s if s.matches("duration") /* Duration com.amazonaws.ec2#HostReservation$Duration */ =>  {
                let var_3 =
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
                builder = builder.set_duration(var_3);
            }
            ,
            s if s.matches("end") /* End com.amazonaws.ec2#HostReservation$End */ =>  {
                let var_4 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_end(var_4);
            }
            ,
            s if s.matches("hostIdSet") /* HostIdSet com.amazonaws.ec2#HostReservation$HostIdSet */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_response_host_id_set::de_response_host_id_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_host_id_set(var_5);
            }
            ,
            s if s.matches("hostReservationId") /* HostReservationId com.amazonaws.ec2#HostReservation$HostReservationId */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_host_reservation_id(var_6);
            }
            ,
            s if s.matches("hourlyPrice") /* HourlyPrice com.amazonaws.ec2#HostReservation$HourlyPrice */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_hourly_price(var_7);
            }
            ,
            s if s.matches("instanceFamily") /* InstanceFamily com.amazonaws.ec2#HostReservation$InstanceFamily */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_family(var_8);
            }
            ,
            s if s.matches("offeringId") /* OfferingId com.amazonaws.ec2#HostReservation$OfferingId */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_offering_id(var_9);
            }
            ,
            s if s.matches("paymentOption") /* PaymentOption com.amazonaws.ec2#HostReservation$PaymentOption */ =>  {
                let var_10 =
                    Some(
                        Result::<crate::types::PaymentOption, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::PaymentOption::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_payment_option(var_10);
            }
            ,
            s if s.matches("start") /* Start com.amazonaws.ec2#HostReservation$Start */ =>  {
                let var_11 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_start(var_11);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#HostReservation$State */ =>  {
                let var_12 =
                    Some(
                        Result::<crate::types::ReservationState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ReservationState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_12);
            }
            ,
            s if s.matches("upfrontPrice") /* UpfrontPrice com.amazonaws.ec2#HostReservation$UpfrontPrice */ =>  {
                let var_13 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_upfront_price(var_13);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#HostReservation$Tags */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_14);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
