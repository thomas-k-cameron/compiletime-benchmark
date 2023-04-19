// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_price_schedule(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::PriceSchedule, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::PriceSchedule::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("active") /* Active com.amazonaws.ec2#PriceSchedule$Active */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_active(var_1);
            }
            ,
            s if s.matches("currencyCode") /* CurrencyCode com.amazonaws.ec2#PriceSchedule$CurrencyCode */ =>  {
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
            s if s.matches("price") /* Price com.amazonaws.ec2#PriceSchedule$Price */ =>  {
                let var_3 =
                    Some(
                         {
                            <f64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.ec2#Double`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_price(var_3);
            }
            ,
            s if s.matches("term") /* Term com.amazonaws.ec2#PriceSchedule$Term */ =>  {
                let var_4 =
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
                builder = builder.set_term(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}