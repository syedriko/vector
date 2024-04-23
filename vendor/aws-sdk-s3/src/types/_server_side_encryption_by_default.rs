// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the default server-side encryption to apply to new objects in the bucket. If a PUT Object request doesn't specify any server-side encryption, this default encryption will be applied. If you don't specify a customer managed key at configuration, Amazon S3 automatically creates an Amazon Web Services KMS key in your Amazon Web Services account the first time that you add an object encrypted with SSE-KMS to a bucket. By default, Amazon S3 uses this KMS key for SSE-KMS. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTencryption.html">PUT Bucket encryption</a> in the <i>Amazon S3 API Reference</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ServerSideEncryptionByDefault {
    /// <p>Server-side encryption algorithm to use for the default encryption.</p>
    pub sse_algorithm: crate::types::ServerSideEncryption,
    /// <p>Amazon Web Services Key Management Service (KMS) customer Amazon Web Services KMS key ID to use for the default encryption. This parameter is allowed if and only if <code>SSEAlgorithm</code> is set to <code>aws:kms</code>.</p>
    /// <p>You can specify the key ID, key alias, or the Amazon Resource Name (ARN) of the KMS key.</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key Alias: <code>alias/alias-name</code> </p> </li>
    /// </ul>
    /// <p>If you use a key ID, you can run into a LogDestination undeliverable error when creating a VPC flow log. </p>
    /// <p>If you are using encryption with cross-account or Amazon Web Services service operations you must use a fully qualified KMS key ARN. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html#bucket-encryption-update-bucket-policy">Using encryption for cross-account operations</a>.</p> <important>
    /// <p>Amazon S3 only supports symmetric encryption KMS keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric keys in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    /// </important>
    pub kms_master_key_id: ::std::option::Option<::std::string::String>,
}
impl ServerSideEncryptionByDefault {
    /// <p>Server-side encryption algorithm to use for the default encryption.</p>
    pub fn sse_algorithm(&self) -> &crate::types::ServerSideEncryption {
        &self.sse_algorithm
    }
    /// <p>Amazon Web Services Key Management Service (KMS) customer Amazon Web Services KMS key ID to use for the default encryption. This parameter is allowed if and only if <code>SSEAlgorithm</code> is set to <code>aws:kms</code>.</p>
    /// <p>You can specify the key ID, key alias, or the Amazon Resource Name (ARN) of the KMS key.</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key Alias: <code>alias/alias-name</code> </p> </li>
    /// </ul>
    /// <p>If you use a key ID, you can run into a LogDestination undeliverable error when creating a VPC flow log. </p>
    /// <p>If you are using encryption with cross-account or Amazon Web Services service operations you must use a fully qualified KMS key ARN. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html#bucket-encryption-update-bucket-policy">Using encryption for cross-account operations</a>.</p> <important>
    /// <p>Amazon S3 only supports symmetric encryption KMS keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric keys in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    /// </important>
    pub fn kms_master_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_master_key_id.as_deref()
    }
}
impl ::std::fmt::Debug for ServerSideEncryptionByDefault {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ServerSideEncryptionByDefault");
        formatter.field("sse_algorithm", &self.sse_algorithm);
        formatter.field("kms_master_key_id", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl ServerSideEncryptionByDefault {
    /// Creates a new builder-style object to manufacture [`ServerSideEncryptionByDefault`](crate::types::ServerSideEncryptionByDefault).
    pub fn builder() -> crate::types::builders::ServerSideEncryptionByDefaultBuilder {
        crate::types::builders::ServerSideEncryptionByDefaultBuilder::default()
    }
}

/// A builder for [`ServerSideEncryptionByDefault`](crate::types::ServerSideEncryptionByDefault).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ServerSideEncryptionByDefaultBuilder {
    pub(crate) sse_algorithm: ::std::option::Option<crate::types::ServerSideEncryption>,
    pub(crate) kms_master_key_id: ::std::option::Option<::std::string::String>,
}
impl ServerSideEncryptionByDefaultBuilder {
    /// <p>Server-side encryption algorithm to use for the default encryption.</p>
    /// This field is required.
    pub fn sse_algorithm(mut self, input: crate::types::ServerSideEncryption) -> Self {
        self.sse_algorithm = ::std::option::Option::Some(input);
        self
    }
    /// <p>Server-side encryption algorithm to use for the default encryption.</p>
    pub fn set_sse_algorithm(mut self, input: ::std::option::Option<crate::types::ServerSideEncryption>) -> Self {
        self.sse_algorithm = input;
        self
    }
    /// <p>Server-side encryption algorithm to use for the default encryption.</p>
    pub fn get_sse_algorithm(&self) -> &::std::option::Option<crate::types::ServerSideEncryption> {
        &self.sse_algorithm
    }
    /// <p>Amazon Web Services Key Management Service (KMS) customer Amazon Web Services KMS key ID to use for the default encryption. This parameter is allowed if and only if <code>SSEAlgorithm</code> is set to <code>aws:kms</code>.</p>
    /// <p>You can specify the key ID, key alias, or the Amazon Resource Name (ARN) of the KMS key.</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key Alias: <code>alias/alias-name</code> </p> </li>
    /// </ul>
    /// <p>If you use a key ID, you can run into a LogDestination undeliverable error when creating a VPC flow log. </p>
    /// <p>If you are using encryption with cross-account or Amazon Web Services service operations you must use a fully qualified KMS key ARN. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html#bucket-encryption-update-bucket-policy">Using encryption for cross-account operations</a>.</p> <important>
    /// <p>Amazon S3 only supports symmetric encryption KMS keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric keys in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    /// </important>
    pub fn kms_master_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_master_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Web Services Key Management Service (KMS) customer Amazon Web Services KMS key ID to use for the default encryption. This parameter is allowed if and only if <code>SSEAlgorithm</code> is set to <code>aws:kms</code>.</p>
    /// <p>You can specify the key ID, key alias, or the Amazon Resource Name (ARN) of the KMS key.</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key Alias: <code>alias/alias-name</code> </p> </li>
    /// </ul>
    /// <p>If you use a key ID, you can run into a LogDestination undeliverable error when creating a VPC flow log. </p>
    /// <p>If you are using encryption with cross-account or Amazon Web Services service operations you must use a fully qualified KMS key ARN. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html#bucket-encryption-update-bucket-policy">Using encryption for cross-account operations</a>.</p> <important>
    /// <p>Amazon S3 only supports symmetric encryption KMS keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric keys in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    /// </important>
    pub fn set_kms_master_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_master_key_id = input;
        self
    }
    /// <p>Amazon Web Services Key Management Service (KMS) customer Amazon Web Services KMS key ID to use for the default encryption. This parameter is allowed if and only if <code>SSEAlgorithm</code> is set to <code>aws:kms</code>.</p>
    /// <p>You can specify the key ID, key alias, or the Amazon Resource Name (ARN) of the KMS key.</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key Alias: <code>alias/alias-name</code> </p> </li>
    /// </ul>
    /// <p>If you use a key ID, you can run into a LogDestination undeliverable error when creating a VPC flow log. </p>
    /// <p>If you are using encryption with cross-account or Amazon Web Services service operations you must use a fully qualified KMS key ARN. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html#bucket-encryption-update-bucket-policy">Using encryption for cross-account operations</a>.</p> <important>
    /// <p>Amazon S3 only supports symmetric encryption KMS keys. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric keys in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    /// </important>
    pub fn get_kms_master_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_master_key_id
    }
    /// Consumes the builder and constructs a [`ServerSideEncryptionByDefault`](crate::types::ServerSideEncryptionByDefault).
    /// This method will fail if any of the following fields are not set:
    /// - [`sse_algorithm`](crate::types::builders::ServerSideEncryptionByDefaultBuilder::sse_algorithm)
    pub fn build(self) -> ::std::result::Result<crate::types::ServerSideEncryptionByDefault, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ServerSideEncryptionByDefault {
            sse_algorithm: self.sse_algorithm.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "sse_algorithm",
                    "sse_algorithm was not specified but it is required when building ServerSideEncryptionByDefault",
                )
            })?,
            kms_master_key_id: self.kms_master_key_id,
        })
    }
}
impl ::std::fmt::Debug for ServerSideEncryptionByDefaultBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ServerSideEncryptionByDefaultBuilder");
        formatter.field("sse_algorithm", &self.sse_algorithm);
        formatter.field("kms_master_key_id", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
