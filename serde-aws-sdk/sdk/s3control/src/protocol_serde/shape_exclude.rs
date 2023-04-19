// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_exclude(
    input: &crate::types::Exclude,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.buckets {
        let mut inner_writer = scope.start_el("Buckets").finish();
        for list_item_2 in var_1 {
            {
                let mut inner_writer = inner_writer.start_el("Arn").finish();
                inner_writer.data(list_item_2.as_str());
            }
        }
    }
    if let Some(var_3) = &input.regions {
        let mut inner_writer = scope.start_el("Regions").finish();
        for list_item_4 in var_3 {
            {
                let mut inner_writer = inner_writer.start_el("Region").finish();
                inner_writer.data(list_item_4.as_str());
            }
        }
    }
    scope.finish();
    Ok(())
}

pub fn de_exclude(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Exclude, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Exclude::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Buckets") /* Buckets com.amazonaws.s3control#Exclude$Buckets */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_buckets::de_buckets(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_buckets(var_5);
            }
            ,
            s if s.matches("Regions") /* Regions com.amazonaws.s3control#Exclude$Regions */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_regions::de_regions(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_regions(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
