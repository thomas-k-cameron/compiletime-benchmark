// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ownership_controls(
    input: &crate::types::OwnershipControls,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.rules {
        for list_item_2 in var_1 {
            {
                let inner_writer = scope.start_el("Rule");
                crate::protocol_serde::shape_ownership_controls_rule::ser_ownership_controls_rule(
                    list_item_2,
                    inner_writer,
                )?
            }
        }
    }
    scope.finish();
    Ok(())
}

pub fn de_ownership_controls(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::OwnershipControls, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::OwnershipControls::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Rule") /* Rules com.amazonaws.s3#OwnershipControls$Rules */ =>  {
                let var_3 =
                    Some(
                        Result::<std::vec::Vec<crate::types::OwnershipControlsRule>, aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_4 = builder.rules.take().unwrap_or_default();
                            list_4.push(
                                crate::protocol_serde::shape_ownership_controls_rule::de_ownership_controls_rule(&mut tag)
                                ?
                            );
                            list_4
                        })
                        ?
                    )
                ;
                builder = builder.set_rules(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}