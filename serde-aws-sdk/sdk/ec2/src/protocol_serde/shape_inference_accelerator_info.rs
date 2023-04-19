// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_inference_accelerator_info(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::InferenceAcceleratorInfo, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InferenceAcceleratorInfo::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("accelerators") /* Accelerators com.amazonaws.ec2#InferenceAcceleratorInfo$Accelerators */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_inference_device_info_list::de_inference_device_info_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_accelerators(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
