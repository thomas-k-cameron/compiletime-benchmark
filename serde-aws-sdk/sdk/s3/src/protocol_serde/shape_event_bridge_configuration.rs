// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_event_bridge_configuration(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::EventBridgeConfiguration, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::EventBridgeConfiguration::builder();
    let _ = decoder;
    Ok(builder.build())
}
