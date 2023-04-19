// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_unsuccessful_item(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::UnsuccessfulItem, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::UnsuccessfulItem::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("error") /* Error com.amazonaws.ec2#UnsuccessfulItem$Error */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_unsuccessful_item_error::de_unsuccessful_item_error(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_error(var_1);
            }
            ,
            s if s.matches("resourceId") /* ResourceId com.amazonaws.ec2#UnsuccessfulItem$ResourceId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_id(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}