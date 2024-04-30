// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBucketTagging`](crate::operation::delete_bucket_tagging::builders::DeleteBucketTaggingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::delete_bucket_tagging::builders::DeleteBucketTaggingFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_bucket_tagging::builders::DeleteBucketTaggingFluentBuilder::set_bucket):<br>required: **true**<br><p>The bucket that has the tag set to be removed.</p><br>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::delete_bucket_tagging::builders::DeleteBucketTaggingFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::delete_bucket_tagging::builders::DeleteBucketTaggingFluentBuilder::set_expected_bucket_owner):<br>required: **false**<br><p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p><br>
    /// - On success, responds with [`DeleteBucketTaggingOutput`](crate::operation::delete_bucket_tagging::DeleteBucketTaggingOutput)
    /// - On failure, responds with [`SdkError<DeleteBucketTaggingError>`](crate::operation::delete_bucket_tagging::DeleteBucketTaggingError)
    pub fn delete_bucket_tagging(&self) -> crate::operation::delete_bucket_tagging::builders::DeleteBucketTaggingFluentBuilder {
        crate::operation::delete_bucket_tagging::builders::DeleteBucketTaggingFluentBuilder::new(self.handle.clone())
    }
}
