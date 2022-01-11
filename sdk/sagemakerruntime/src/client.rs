// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    pub(crate) client: aws_smithy_client::Client<C, M, R>,
    pub(crate) conf: crate::Config,
}

/// Client for Amazon SageMaker Runtime
///
/// Client for invoking operations on Amazon SageMaker Runtime. Each operation on Amazon SageMaker Runtime is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_sagemakerruntime::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
///     let shared_config = aws_config::load_from_env().await;
///     let config = aws_sdk_sagemakerruntime::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_sagemakerruntime::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the [`InvokeEndpoint`](crate::client::fluent_builders::InvokeEndpoint) operation.
    ///
    /// - Takes [`InvokeEndpointInput`](crate::input::InvokeEndpointInput) with field(s):
    ///   - [`endpoint_name(Option<String>)`](crate::input::InvokeEndpointInput::endpoint_name): <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API. </p>
    ///   - [`body(Option<Blob>)`](crate::input::InvokeEndpointInput::body): <p>Provides input data, in the format specified in the <code>ContentType</code> request header. Amazon SageMaker passes all of the data in the body to the model. </p>  <p>For information about the format of the request body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
    ///   - [`content_type(Option<String>)`](crate::input::InvokeEndpointInput::content_type): <p>The MIME type of the input data in the request body.</p>
    ///   - [`accept(Option<String>)`](crate::input::InvokeEndpointInput::accept): <p>The desired MIME type of the inference in the response.</p>
    ///   - [`custom_attributes(Option<String>)`](crate::input::InvokeEndpointInput::custom_attributes): <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p>  <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p>  <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK.</p>
    ///   - [`target_model(Option<String>)`](crate::input::InvokeEndpointInput::target_model): <p>The model to request for inference when invoking a multi-model endpoint.</p>
    ///   - [`target_variant(Option<String>)`](crate::input::InvokeEndpointInput::target_variant): <p>Specify the production variant to send the inference request to when invoking an endpoint that is running two or more variants. Note that this parameter overrides the default behavior for the endpoint, which is to distribute the invocation traffic based on the variant weights.</p>  <p>For information about how to use variant targeting to perform a/b testing, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-ab-testing.html">Test models in production</a> </p>
    ///   - [`target_container_hostname(Option<String>)`](crate::input::InvokeEndpointInput::target_container_hostname): <p>If the endpoint hosts multiple containers and is configured to use direct invocation, this parameter specifies the host name of the container to invoke.</p>
    ///   - [`inference_id(Option<String>)`](crate::input::InvokeEndpointInput::inference_id): <p>If you provide a value, it is added to the captured data when you enable data capture on the endpoint. For information about data capture, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor-data-capture.html">Capture Data</a>.</p>
    /// - On success, responds with [`InvokeEndpointOutput`](crate::output::InvokeEndpointOutput) with field(s):
    ///   - [`body(Option<Blob>)`](crate::output::InvokeEndpointOutput::body): <p>Includes the inference provided by the model.</p>  <p>For information about the format of the response body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
    ///   - [`content_type(Option<String>)`](crate::output::InvokeEndpointOutput::content_type): <p>The MIME type of the inference returned in the response body.</p>
    ///   - [`invoked_production_variant(Option<String>)`](crate::output::InvokeEndpointOutput::invoked_production_variant): <p>Identifies the production variant that was invoked.</p>
    ///   - [`custom_attributes(Option<String>)`](crate::output::InvokeEndpointOutput::custom_attributes): <p>Provides additional information in the response about the inference returned by a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to return an ID received in the <code>CustomAttributes</code> header of a request or other metadata that a service endpoint was programmed to produce. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). If the customer wants the custom attribute returned, the model must set the custom attribute to be included on the way back. </p>  <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p>  <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK.</p>
    /// - On failure, responds with [`SdkError<InvokeEndpointError>`](crate::error::InvokeEndpointError)
    pub fn invoke_endpoint(&self) -> fluent_builders::InvokeEndpoint<C, M, R> {
        fluent_builders::InvokeEndpoint::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`InvokeEndpointAsync`](crate::client::fluent_builders::InvokeEndpointAsync) operation.
    ///
    /// - Takes [`InvokeEndpointAsyncInput`](crate::input::InvokeEndpointAsyncInput) with field(s):
    ///   - [`endpoint_name(Option<String>)`](crate::input::InvokeEndpointAsyncInput::endpoint_name): <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html"> <code>CreateEndpoint</code> </a> API.</p>
    ///   - [`content_type(Option<String>)`](crate::input::InvokeEndpointAsyncInput::content_type): <p>The MIME type of the input data in the request body.</p>
    ///   - [`accept(Option<String>)`](crate::input::InvokeEndpointAsyncInput::accept): <p>The desired MIME type of the inference in the response.</p>
    ///   - [`custom_attributes(Option<String>)`](crate::input::InvokeEndpointAsyncInput::custom_attributes): <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p>  <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID</code>: in your post-processing function. </p>  <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK. </p>
    ///   - [`inference_id(Option<String>)`](crate::input::InvokeEndpointAsyncInput::inference_id): <p>The identifier for the inference request. Amazon SageMaker will generate an identifier for you if none is specified. </p>
    ///   - [`input_location(Option<String>)`](crate::input::InvokeEndpointAsyncInput::input_location): <p>The Amazon S3 URI where the inference request payload is stored.</p>
    ///   - [`request_ttl_seconds(Option<i32>)`](crate::input::InvokeEndpointAsyncInput::request_ttl_seconds): <p>Maximum age in seconds a request can be in the queue before it is marked as expired.</p>
    /// - On success, responds with [`InvokeEndpointAsyncOutput`](crate::output::InvokeEndpointAsyncOutput) with field(s):
    ///   - [`inference_id(Option<String>)`](crate::output::InvokeEndpointAsyncOutput::inference_id): <p>Identifier for an inference request. This will be the same as the <code>InferenceId</code> specified in the input. Amazon SageMaker will generate an identifier for you if you do not specify one.</p>
    ///   - [`output_location(Option<String>)`](crate::output::InvokeEndpointAsyncOutput::output_location): <p>The Amazon S3 URI where the inference response payload is stored.</p>
    /// - On failure, responds with [`SdkError<InvokeEndpointAsyncError>`](crate::error::InvokeEndpointAsyncError)
    pub fn invoke_endpoint_async(&self) -> fluent_builders::InvokeEndpointAsync<C, M, R> {
        fluent_builders::InvokeEndpointAsync::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `InvokeEndpoint`.
    ///
    /// <p>After you deploy a model into production using Amazon SageMaker hosting services, your client applications use this API to get inferences from the model hosted at the specified endpoint. </p>
    /// <p>For an overview of Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    /// <p>Amazon SageMaker strips all POST headers except those supported by the API. Amazon SageMaker might add additional headers. You should not rely on the behavior of headers outside those enumerated in the request syntax. </p>
    /// <p>Calls to <code>InvokeEndpoint</code> are authenticated by using Amazon Web Services Signature Version 4. For information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html">Authenticating Requests (Amazon Web Services Signature Version 4)</a> in the <i>Amazon S3 API Reference</i>.</p>
    /// <p>A customer's model containers must respond to requests within 60 seconds. The model itself can have a maximum processing time of 60 seconds before responding to invocations. If your model is going to take 50-60 seconds of processing time, the SDK socket timeout should be set to be 70 seconds.</p> <note>
    /// <p>Endpoints are scoped to an individual account, and are not public. The URL does not contain the account ID, but Amazon SageMaker determines the account ID from the authentication token that is supplied by the caller.</p>
    /// </note>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct InvokeEndpoint<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::invoke_endpoint_input::Builder,
    }
    impl<C, M, R> InvokeEndpoint<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `InvokeEndpoint`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::InvokeEndpointOutput,
            aws_smithy_http::result::SdkError<crate::error::InvokeEndpointError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::InvokeEndpointInputOperationOutputAlias,
                crate::output::InvokeEndpointOutput,
                crate::error::InvokeEndpointError,
                crate::input::InvokeEndpointInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API. </p>
        pub fn endpoint_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.endpoint_name(input.into());
            self
        }
        /// <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API. </p>
        pub fn set_endpoint_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_endpoint_name(input);
            self
        }
        /// <p>Provides input data, in the format specified in the <code>ContentType</code> request header. Amazon SageMaker passes all of the data in the body to the model. </p>
        /// <p>For information about the format of the request body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
        pub fn body(mut self, input: aws_smithy_types::Blob) -> Self {
            self.inner = self.inner.body(input);
            self
        }
        /// <p>Provides input data, in the format specified in the <code>ContentType</code> request header. Amazon SageMaker passes all of the data in the body to the model. </p>
        /// <p>For information about the format of the request body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
        pub fn set_body(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.inner = self.inner.set_body(input);
            self
        }
        /// <p>The MIME type of the input data in the request body.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.content_type(input.into());
            self
        }
        /// <p>The MIME type of the input data in the request body.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_content_type(input);
            self
        }
        /// <p>The desired MIME type of the inference in the response.</p>
        pub fn accept(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.accept(input.into());
            self
        }
        /// <p>The desired MIME type of the inference in the response.</p>
        pub fn set_accept(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_accept(input);
            self
        }
        /// <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p>
        /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p>
        /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK.</p>
        pub fn custom_attributes(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.custom_attributes(input.into());
            self
        }
        /// <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p>
        /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p>
        /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK.</p>
        pub fn set_custom_attributes(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_custom_attributes(input);
            self
        }
        /// <p>The model to request for inference when invoking a multi-model endpoint.</p>
        pub fn target_model(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_model(input.into());
            self
        }
        /// <p>The model to request for inference when invoking a multi-model endpoint.</p>
        pub fn set_target_model(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_target_model(input);
            self
        }
        /// <p>Specify the production variant to send the inference request to when invoking an endpoint that is running two or more variants. Note that this parameter overrides the default behavior for the endpoint, which is to distribute the invocation traffic based on the variant weights.</p>
        /// <p>For information about how to use variant targeting to perform a/b testing, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-ab-testing.html">Test models in production</a> </p>
        pub fn target_variant(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_variant(input.into());
            self
        }
        /// <p>Specify the production variant to send the inference request to when invoking an endpoint that is running two or more variants. Note that this parameter overrides the default behavior for the endpoint, which is to distribute the invocation traffic based on the variant weights.</p>
        /// <p>For information about how to use variant targeting to perform a/b testing, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-ab-testing.html">Test models in production</a> </p>
        pub fn set_target_variant(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_target_variant(input);
            self
        }
        /// <p>If the endpoint hosts multiple containers and is configured to use direct invocation, this parameter specifies the host name of the container to invoke.</p>
        pub fn target_container_hostname(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.target_container_hostname(input.into());
            self
        }
        /// <p>If the endpoint hosts multiple containers and is configured to use direct invocation, this parameter specifies the host name of the container to invoke.</p>
        pub fn set_target_container_hostname(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_target_container_hostname(input);
            self
        }
        /// <p>If you provide a value, it is added to the captured data when you enable data capture on the endpoint. For information about data capture, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor-data-capture.html">Capture Data</a>.</p>
        pub fn inference_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.inference_id(input.into());
            self
        }
        /// <p>If you provide a value, it is added to the captured data when you enable data capture on the endpoint. For information about data capture, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor-data-capture.html">Capture Data</a>.</p>
        pub fn set_inference_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_inference_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `InvokeEndpointAsync`.
    ///
    /// <p>After you deploy a model into production using Amazon SageMaker hosting services, your client applications use this API to get inferences from the model hosted at the specified endpoint in an asynchronous manner.</p>
    /// <p>Inference requests sent to this API are enqueued for asynchronous processing. The processing of the inference request may or may not complete before the you receive a response from this API. The response from this API will not contain the result of the inference request but contain information about where you can locate it.</p>
    /// <p>Amazon SageMaker strips all <code>POST</code> headers except those supported by the API. Amazon SageMaker might add additional headers. You should not rely on the behavior of headers outside those enumerated in the request syntax.</p>
    /// <p>Calls to <code>InvokeEndpointAsync</code> are authenticated by using Amazon Web Services Signature Version 4. For information, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html">Authenticating Requests (Amazon Web Services Signature Version 4)</a> in the <i>Amazon S3 API Reference</i>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct InvokeEndpointAsync<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::invoke_endpoint_async_input::Builder,
    }
    impl<C, M, R> InvokeEndpointAsync<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `InvokeEndpointAsync`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::InvokeEndpointAsyncOutput,
            aws_smithy_http::result::SdkError<crate::error::InvokeEndpointAsyncError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::InvokeEndpointAsyncInputOperationOutputAlias,
                crate::output::InvokeEndpointAsyncOutput,
                crate::error::InvokeEndpointAsyncError,
                crate::input::InvokeEndpointAsyncInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html"> <code>CreateEndpoint</code> </a> API.</p>
        pub fn endpoint_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.endpoint_name(input.into());
            self
        }
        /// <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html"> <code>CreateEndpoint</code> </a> API.</p>
        pub fn set_endpoint_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_endpoint_name(input);
            self
        }
        /// <p>The MIME type of the input data in the request body.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.content_type(input.into());
            self
        }
        /// <p>The MIME type of the input data in the request body.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_content_type(input);
            self
        }
        /// <p>The desired MIME type of the inference in the response.</p>
        pub fn accept(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.accept(input.into());
            self
        }
        /// <p>The desired MIME type of the inference in the response.</p>
        pub fn set_accept(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_accept(input);
            self
        }
        /// <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p>
        /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID</code>: in your post-processing function. </p>
        /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK. </p>
        pub fn custom_attributes(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.custom_attributes(input.into());
            self
        }
        /// <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p>
        /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID</code>: in your post-processing function. </p>
        /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK. </p>
        pub fn set_custom_attributes(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_custom_attributes(input);
            self
        }
        /// <p>The identifier for the inference request. Amazon SageMaker will generate an identifier for you if none is specified. </p>
        pub fn inference_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.inference_id(input.into());
            self
        }
        /// <p>The identifier for the inference request. Amazon SageMaker will generate an identifier for you if none is specified. </p>
        pub fn set_inference_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_inference_id(input);
            self
        }
        /// <p>The Amazon S3 URI where the inference request payload is stored.</p>
        pub fn input_location(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.input_location(input.into());
            self
        }
        /// <p>The Amazon S3 URI where the inference request payload is stored.</p>
        pub fn set_input_location(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_input_location(input);
            self
        }
        /// <p>Maximum age in seconds a request can be in the queue before it is marked as expired.</p>
        pub fn request_ttl_seconds(mut self, input: i32) -> Self {
            self.inner = self.inner.request_ttl_seconds(input);
            self
        }
        /// <p>Maximum age in seconds a request can be in the queue before it is marked as expired.</p>
        pub fn set_request_ttl_seconds(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_request_ttl_seconds(input);
            self
        }
    }
}

impl<C> Client<C, crate::middleware::DefaultMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(conn)
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        crate::middleware::DefaultMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https()
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
