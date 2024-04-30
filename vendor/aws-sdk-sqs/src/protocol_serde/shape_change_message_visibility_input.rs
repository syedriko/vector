// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_change_message_visibility_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::change_message_visibility::ChangeMessageVisibilityInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.queue_url {
        object.key("QueueUrl").string(var_1.as_str());
    }
    if let Some(var_2) = &input.receipt_handle {
        object.key("ReceiptHandle").string(var_2.as_str());
    }
    if let Some(var_3) = &input.visibility_timeout {
        object.key("VisibilityTimeout").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    Ok(())
}
