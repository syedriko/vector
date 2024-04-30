// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ChangeMessageVisibilityBatch`](crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`queue_url(impl Into<String>)`](crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder::set_queue_url):<br>required: **true**<br><p>The URL of the Amazon SQS queue whose messages' visibility is changed.</p>  <p>Queue URLs and names are case-sensitive.</p><br>
    ///   - [`entries(ChangeMessageVisibilityBatchRequestEntry)`](crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder::entries) / [`set_entries(Option<Vec::<ChangeMessageVisibilityBatchRequestEntry>>)`](crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder::set_entries):<br>required: **true**<br><p>Lists the receipt handles of the messages for which the visibility timeout must be changed.</p><br>
    /// - On success, responds with [`ChangeMessageVisibilityBatchOutput`](crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput) with field(s):
    ///   - [`successful(Vec::<ChangeMessageVisibilityBatchResultEntry>)`](crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput::successful): <p>A list of <code> <code>ChangeMessageVisibilityBatchResultEntry</code> </code> items.</p>
    ///   - [`failed(Vec::<BatchResultErrorEntry>)`](crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchOutput::failed): <p>A list of <code> <code>BatchResultErrorEntry</code> </code> items.</p>
    /// - On failure, responds with [`SdkError<ChangeMessageVisibilityBatchError>`](crate::operation::change_message_visibility_batch::ChangeMessageVisibilityBatchError)
    pub fn change_message_visibility_batch(
        &self,
    ) -> crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder {
        crate::operation::change_message_visibility_batch::builders::ChangeMessageVisibilityBatchFluentBuilder::new(self.handle.clone())
    }
}
