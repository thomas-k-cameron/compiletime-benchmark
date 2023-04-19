// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_group_identifier(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::GroupIdentifier, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::GroupIdentifier::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("groupName") /* GroupName com.amazonaws.ec2#GroupIdentifier$GroupName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_group_name(var_1);
            }
            ,
            s if s.matches("groupId") /* GroupId com.amazonaws.ec2#GroupIdentifier$GroupId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_group_id(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

#[allow(unused_mut)]
pub fn ser_group_identifier(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::GroupIdentifier,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("GroupName");
    if let Some(var_4) = &input.group_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("GroupId");
    if let Some(var_6) = &input.group_id {
        scope_5.string(var_6);
    }
    Ok(())
}
