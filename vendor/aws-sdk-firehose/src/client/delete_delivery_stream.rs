// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDeliveryStream`](crate::operation::delete_delivery_stream::builders::DeleteDeliveryStreamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`delivery_stream_name(impl Into<String>)`](crate::operation::delete_delivery_stream::builders::DeleteDeliveryStreamFluentBuilder::delivery_stream_name) / [`set_delivery_stream_name(Option<String>)`](crate::operation::delete_delivery_stream::builders::DeleteDeliveryStreamFluentBuilder::set_delivery_stream_name):<br>required: **true**<br><p>The name of the delivery stream.</p><br>
    ///   - [`allow_force_delete(bool)`](crate::operation::delete_delivery_stream::builders::DeleteDeliveryStreamFluentBuilder::allow_force_delete) / [`set_allow_force_delete(Option<bool>)`](crate::operation::delete_delivery_stream::builders::DeleteDeliveryStreamFluentBuilder::set_allow_force_delete):<br>required: **false**<br><p>Set this to true if you want to delete the delivery stream even if Kinesis Data Firehose is unable to retire the grant for the CMK. Kinesis Data Firehose might be unable to retire the grant due to a customer error, such as when the CMK or the grant are in an invalid state. If you force deletion, you can then use the <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_RevokeGrant.html">RevokeGrant</a> operation to revoke the grant you gave to Kinesis Data Firehose. If a failure to retire the grant happens due to an Amazon Web Services KMS issue, Kinesis Data Firehose keeps retrying the delete operation.</p>  <p>The default value is false.</p><br>
    /// - On success, responds with [`DeleteDeliveryStreamOutput`](crate::operation::delete_delivery_stream::DeleteDeliveryStreamOutput)
    /// - On failure, responds with [`SdkError<DeleteDeliveryStreamError>`](crate::operation::delete_delivery_stream::DeleteDeliveryStreamError)
    pub fn delete_delivery_stream(&self) -> crate::operation::delete_delivery_stream::builders::DeleteDeliveryStreamFluentBuilder {
        crate::operation::delete_delivery_stream::builders::DeleteDeliveryStreamFluentBuilder::new(self.handle.clone())
    }
}
