// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_route_list(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    std::vec::Vec<crate::types::MultiRegionAccessPointRoute>,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Route") /* member com.amazonaws.s3control#RouteList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_multi_region_access_point_route::de_multi_region_access_point_route(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}