// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_inventory_schedule(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::InventorySchedule, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InventorySchedule::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Frequency") /* Frequency com.amazonaws.s3#InventorySchedule$Frequency */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::InventoryFrequency, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InventoryFrequency::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_frequency(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::inventory_schedule_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}

pub fn ser_inventory_schedule(
    input: &crate::types::InventorySchedule,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    {
        let mut inner_writer = scope.start_el("Frequency").finish();
        inner_writer.data(input.frequency.as_str());
    }
    scope.finish();
    Ok(())
}
