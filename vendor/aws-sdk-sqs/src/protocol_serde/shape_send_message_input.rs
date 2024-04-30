// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_message_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::send_message::SendMessageInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.queue_url {
        object.key("QueueUrl").string(var_1.as_str());
    }
    if let Some(var_2) = &input.message_body {
        object.key("MessageBody").string(var_2.as_str());
    }
    if let Some(var_3) = &input.delay_seconds {
        object.key("DelaySeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.message_attributes {
        #[allow(unused_mut)]
        let mut object_5 = object.key("MessageAttributes").start_object();
        for (key_6, value_7) in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_8 = object_5.key(key_6.as_str()).start_object();
                crate::protocol_serde::shape_message_attribute_value::ser_message_attribute_value(&mut object_8, value_7)?;
                object_8.finish();
            }
        }
        object_5.finish();
    }
    if let Some(var_9) = &input.message_system_attributes {
        #[allow(unused_mut)]
        let mut object_10 = object.key("MessageSystemAttributes").start_object();
        for (key_11, value_12) in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_13 = object_10.key(key_11.as_str()).start_object();
                crate::protocol_serde::shape_message_system_attribute_value::ser_message_system_attribute_value(&mut object_13, value_12)?;
                object_13.finish();
            }
        }
        object_10.finish();
    }
    if let Some(var_14) = &input.message_deduplication_id {
        object.key("MessageDeduplicationId").string(var_14.as_str());
    }
    if let Some(var_15) = &input.message_group_id {
        object.key("MessageGroupId").string(var_15.as_str());
    }
    Ok(())
}
