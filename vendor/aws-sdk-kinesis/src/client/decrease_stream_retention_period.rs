// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DecreaseStreamRetentionPeriod`](crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stream_name(impl Into<String>)`](crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodFluentBuilder::stream_name) / [`set_stream_name(Option<String>)`](crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodFluentBuilder::set_stream_name):<br>required: **false**<br><p>The name of the stream to modify.</p><br>
    ///   - [`retention_period_hours(i32)`](crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodFluentBuilder::retention_period_hours) / [`set_retention_period_hours(Option<i32>)`](crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodFluentBuilder::set_retention_period_hours):<br>required: **true**<br><p>The new retention period of the stream, in hours. Must be less than the current retention period.</p><br>
    ///   - [`stream_arn(impl Into<String>)`](crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodFluentBuilder::set_stream_arn):<br>required: **false**<br><p>The ARN of the stream.</p><br>
    /// - On success, responds with [`DecreaseStreamRetentionPeriodOutput`](crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodOutput)
    /// - On failure, responds with [`SdkError<DecreaseStreamRetentionPeriodError>`](crate::operation::decrease_stream_retention_period::DecreaseStreamRetentionPeriodError)
    pub fn decrease_stream_retention_period(
        &self,
    ) -> crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodFluentBuilder {
        crate::operation::decrease_stream_retention_period::builders::DecreaseStreamRetentionPeriodFluentBuilder::new(self.handle.clone())
    }
}
