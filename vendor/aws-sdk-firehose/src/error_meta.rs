// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Another modification has already happened. Fetch <code>VersionId</code> again and use it to update the destination.</p>
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p>Kinesis Data Firehose throws this exception when an attempt to put records or to start or stop delivery stream encryption fails. This happens when the KMS service throws one of the following exception types: <code>AccessDeniedException</code>, <code>InvalidStateException</code>, <code>DisabledException</code>, or <code>NotFoundException</code>.</p>
    InvalidKmsResourceException(crate::error::InvalidKmsResourceException),
    /// <p>You have already reached the limit for a requested resource.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The resource is already in use and not available for this operation.</p>
    ResourceInUseException(crate::error::ResourceInUseException),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The service is unavailable. Back off and retry the operation. If you continue to see the exception, throughput limits for the delivery stream may have been exceeded. For more information about limits and how to request an increase, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::InvalidArgumentException(inner) => inner.fmt(f),
            Error::InvalidKmsResourceException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceInUseException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateDeliveryStreamError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateDeliveryStreamError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateDeliveryStreamError> for Error {
    fn from(err: crate::error::CreateDeliveryStreamError) -> Self {
        match err.kind {
            crate::error::CreateDeliveryStreamErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::CreateDeliveryStreamErrorKind::InvalidKmsResourceException(inner) => {
                Error::InvalidKmsResourceException(inner)
            }
            crate::error::CreateDeliveryStreamErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::CreateDeliveryStreamErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::CreateDeliveryStreamErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteDeliveryStreamError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteDeliveryStreamError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteDeliveryStreamError> for Error {
    fn from(err: crate::error::DeleteDeliveryStreamError) -> Self {
        match err.kind {
            crate::error::DeleteDeliveryStreamErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::DeleteDeliveryStreamErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteDeliveryStreamErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeDeliveryStreamError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeDeliveryStreamError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeDeliveryStreamError> for Error {
    fn from(err: crate::error::DescribeDeliveryStreamError) -> Self {
        match err.kind {
            crate::error::DescribeDeliveryStreamErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeDeliveryStreamErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListDeliveryStreamsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListDeliveryStreamsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListDeliveryStreamsError> for Error {
    fn from(err: crate::error::ListDeliveryStreamsError) -> Self {
        match err.kind {
            crate::error::ListDeliveryStreamsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForDeliveryStreamError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForDeliveryStreamError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListTagsForDeliveryStreamError> for Error {
    fn from(err: crate::error::ListTagsForDeliveryStreamError) -> Self {
        match err.kind {
            crate::error::ListTagsForDeliveryStreamErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::ListTagsForDeliveryStreamErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::ListTagsForDeliveryStreamErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListTagsForDeliveryStreamErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutRecordError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutRecordError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutRecordError> for Error {
    fn from(err: crate::error::PutRecordError) -> Self {
        match err.kind {
            crate::error::PutRecordErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::PutRecordErrorKind::InvalidKmsResourceException(inner) => {
                Error::InvalidKmsResourceException(inner)
            }
            crate::error::PutRecordErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::PutRecordErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::PutRecordErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutRecordBatchError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutRecordBatchError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutRecordBatchError> for Error {
    fn from(err: crate::error::PutRecordBatchError) -> Self {
        match err.kind {
            crate::error::PutRecordBatchErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::PutRecordBatchErrorKind::InvalidKmsResourceException(inner) => {
                Error::InvalidKmsResourceException(inner)
            }
            crate::error::PutRecordBatchErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::PutRecordBatchErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::PutRecordBatchErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartDeliveryStreamEncryptionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartDeliveryStreamEncryptionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartDeliveryStreamEncryptionError> for Error {
    fn from(err: crate::error::StartDeliveryStreamEncryptionError) -> Self {
        match err.kind {
            crate::error::StartDeliveryStreamEncryptionErrorKind::InvalidArgumentException(
                inner,
            ) => Error::InvalidArgumentException(inner),
            crate::error::StartDeliveryStreamEncryptionErrorKind::InvalidKmsResourceException(
                inner,
            ) => Error::InvalidKmsResourceException(inner),
            crate::error::StartDeliveryStreamEncryptionErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::StartDeliveryStreamEncryptionErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::StartDeliveryStreamEncryptionErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::StartDeliveryStreamEncryptionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopDeliveryStreamEncryptionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StopDeliveryStreamEncryptionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StopDeliveryStreamEncryptionError> for Error {
    fn from(err: crate::error::StopDeliveryStreamEncryptionError) -> Self {
        match err.kind {
            crate::error::StopDeliveryStreamEncryptionErrorKind::InvalidArgumentException(
                inner,
            ) => Error::InvalidArgumentException(inner),
            crate::error::StopDeliveryStreamEncryptionErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::StopDeliveryStreamEncryptionErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::StopDeliveryStreamEncryptionErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::StopDeliveryStreamEncryptionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagDeliveryStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::TagDeliveryStreamError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::TagDeliveryStreamError> for Error {
    fn from(err: crate::error::TagDeliveryStreamError) -> Self {
        match err.kind {
            crate::error::TagDeliveryStreamErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::TagDeliveryStreamErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::TagDeliveryStreamErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::TagDeliveryStreamErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::TagDeliveryStreamErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagDeliveryStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UntagDeliveryStreamError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UntagDeliveryStreamError> for Error {
    fn from(err: crate::error::UntagDeliveryStreamError) -> Self {
        match err.kind {
            crate::error::UntagDeliveryStreamErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::UntagDeliveryStreamErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::UntagDeliveryStreamErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::UntagDeliveryStreamErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UntagDeliveryStreamErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateDestinationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateDestinationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateDestinationError> for Error {
    fn from(err: crate::error::UpdateDestinationError) -> Self {
        match err.kind {
            crate::error::UpdateDestinationErrorKind::ConcurrentModificationException(inner) => {
                Error::ConcurrentModificationException(inner)
            }
            crate::error::UpdateDestinationErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::UpdateDestinationErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::UpdateDestinationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UpdateDestinationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
