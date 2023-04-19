// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_lifecycle_rule_and_operator(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LifecycleRuleAndOperator, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LifecycleRuleAndOperator::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3#LifecycleRuleAndOperator$Prefix */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_1);
            }
            ,
            s if s.matches("Tag") /* Tags com.amazonaws.s3#LifecycleRuleAndOperator$Tags */ =>  {
                let var_2 =
                    Some(
                        Result::<std::vec::Vec<crate::types::Tag>, aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_3 = builder.tags.take().unwrap_or_default();
                            list_3.push(
                                crate::protocol_serde::shape_tag::de_tag(&mut tag)
                                ?
                            );
                            list_3
                        })
                        ?
                    )
                ;
                builder = builder.set_tags(var_2);
            }
            ,
            s if s.matches("ObjectSizeGreaterThan") /* ObjectSizeGreaterThan com.amazonaws.s3#LifecycleRuleAndOperator$ObjectSizeGreaterThan */ =>  {
                let var_4 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.s3#ObjectSizeGreaterThanBytes`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_object_size_greater_than(var_4);
            }
            ,
            s if s.matches("ObjectSizeLessThan") /* ObjectSizeLessThan com.amazonaws.s3#LifecycleRuleAndOperator$ObjectSizeLessThan */ =>  {
                let var_5 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.s3#ObjectSizeLessThanBytes`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_object_size_less_than(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_lifecycle_rule_and_operator(
    input: &crate::types::LifecycleRuleAndOperator,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_6) = &input.prefix {
        let mut inner_writer = scope.start_el("Prefix").finish();
        inner_writer.data(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        for list_item_8 in var_7 {
            {
                let inner_writer = scope.start_el("Tag");
                crate::protocol_serde::shape_tag::ser_tag(list_item_8, inner_writer)?
            }
        }
    }
    if input.object_size_greater_than != 0 {
        let mut inner_writer = scope.start_el("ObjectSizeGreaterThan").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(input.object_size_greater_than).encode(),
        );
    }
    if input.object_size_less_than != 0 {
        let mut inner_writer = scope.start_el("ObjectSizeLessThan").finish();
        inner_writer
            .data(aws_smithy_types::primitive::Encoder::from(input.object_size_less_than).encode());
    }
    scope.finish();
    Ok(())
}
