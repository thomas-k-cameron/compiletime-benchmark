// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_replication_rule_and_operator(
    input: &crate::types::ReplicationRuleAndOperator,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.prefix {
        let mut inner_writer = scope.start_el("Prefix").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.tags {
        let mut inner_writer = scope.start_el("Tags").finish();
        for list_item_3 in var_2 {
            {
                let inner_writer = inner_writer.start_el("member");
                crate::protocol_serde::shape_s3_tag::ser_s3_tag(list_item_3, inner_writer)?
            }
        }
    }
    scope.finish();
    Ok(())
}

pub fn de_replication_rule_and_operator(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ReplicationRuleAndOperator, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ReplicationRuleAndOperator::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3control#ReplicationRuleAndOperator$Prefix */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_4);
            }
            ,
            s if s.matches("Tags") /* Tags com.amazonaws.s3control#ReplicationRuleAndOperator$Tags */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_s3_tag_set::de_s3_tag_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
