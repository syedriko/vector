// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBucketMetricsConfiguration`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::set_bucket):<br>required: **true**<br><p>The name of the bucket containing the metrics configuration to delete.</p><br>
    ///   - [`id(impl Into<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::set_id):<br>required: **true**<br><p>The ID used to identify the metrics configuration. The ID has a 64 character limit and can only contain letters, numbers, periods, dashes, and underscores.</p><br>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::set_expected_bucket_owner):<br>required: **false**<br><p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p><br>
    /// - On success, responds with [`DeleteBucketMetricsConfigurationOutput`](crate::operation::delete_bucket_metrics_configuration::DeleteBucketMetricsConfigurationOutput)
    /// - On failure, responds with [`SdkError<DeleteBucketMetricsConfigurationError>`](crate::operation::delete_bucket_metrics_configuration::DeleteBucketMetricsConfigurationError)
    pub fn delete_bucket_metrics_configuration(
        &self,
    ) -> crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder {
        crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::new(self.handle.clone())
    }
}
