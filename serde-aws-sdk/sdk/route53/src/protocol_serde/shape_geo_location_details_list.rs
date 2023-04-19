// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_geo_location_details_list(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<std::vec::Vec<crate::types::GeoLocationDetails>, aws_smithy_xml::decode::XmlDecodeError>
{
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("GeoLocationDetails") /* member com.amazonaws.route53#GeoLocationDetailsList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_geo_location_details::de_geo_location_details(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
