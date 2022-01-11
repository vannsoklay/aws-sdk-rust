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

/// Client for AWS SSO OIDC
///
/// Client for invoking operations on AWS SSO OIDC. Each operation on AWS SSO OIDC is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_ssooidc::Client::new(&shared_config);
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
///     let config = aws_sdk_ssooidc::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_ssooidc::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`CreateToken`](crate::client::fluent_builders::CreateToken) operation.
    ///
    /// - Takes [`CreateTokenInput`](crate::input::CreateTokenInput) with field(s):
    ///   - [`client_id(Option<String>)`](crate::input::CreateTokenInput::client_id): <p>The unique identifier string for each client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    ///   - [`client_secret(Option<String>)`](crate::input::CreateTokenInput::client_secret): <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    ///   - [`grant_type(Option<String>)`](crate::input::CreateTokenInput::grant_type): <p>Supports grant types for authorization code, refresh token, and device code request.</p>
    ///   - [`device_code(Option<String>)`](crate::input::CreateTokenInput::device_code): <p>Used only when calling this API for the device code grant type. This short-term code is used to identify this authentication attempt. This should come from an in-memory reference to the result of the <code>StartDeviceAuthorization</code> API.</p>
    ///   - [`code(Option<String>)`](crate::input::CreateTokenInput::code): <p>The authorization code received from the authorization service. This parameter is required to perform an authorization grant request to get access to a token.</p>
    ///   - [`refresh_token(Option<String>)`](crate::input::CreateTokenInput::refresh_token): <p>The token used to obtain an access token in the event that the access token is invalid or expired. This token is not issued by the service.</p>
    ///   - [`scope(Option<Vec<String>>)`](crate::input::CreateTokenInput::scope): <p>The list of scopes that is defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    ///   - [`redirect_uri(Option<String>)`](crate::input::CreateTokenInput::redirect_uri): <p>The location of the application that will receive the authorization code. Users authorize the service to send the request to this location.</p>
    /// - On success, responds with [`CreateTokenOutput`](crate::output::CreateTokenOutput) with field(s):
    ///   - [`access_token(Option<String>)`](crate::output::CreateTokenOutput::access_token): <p>An opaque token to access AWS SSO resources assigned to a user.</p>
    ///   - [`token_type(Option<String>)`](crate::output::CreateTokenOutput::token_type): <p>Used to notify the client that the returned token is an access token. The supported type is <code>BearerToken</code>.</p>
    ///   - [`expires_in(i32)`](crate::output::CreateTokenOutput::expires_in): <p>Indicates the time in seconds when an access token will expire.</p>
    ///   - [`refresh_token(Option<String>)`](crate::output::CreateTokenOutput::refresh_token): <p>A token that, if present, can be used to refresh a previously issued access token that might have expired.</p>
    ///   - [`id_token(Option<String>)`](crate::output::CreateTokenOutput::id_token): <p>The identifier of the user that associated with the access token, if present.</p>
    /// - On failure, responds with [`SdkError<CreateTokenError>`](crate::error::CreateTokenError)
    pub fn create_token(&self) -> fluent_builders::CreateToken<C, M, R> {
        fluent_builders::CreateToken::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`RegisterClient`](crate::client::fluent_builders::RegisterClient) operation.
    ///
    /// - Takes [`RegisterClientInput`](crate::input::RegisterClientInput) with field(s):
    ///   - [`client_name(Option<String>)`](crate::input::RegisterClientInput::client_name): <p>The friendly name of the client.</p>
    ///   - [`client_type(Option<String>)`](crate::input::RegisterClientInput::client_type): <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    ///   - [`scopes(Option<Vec<String>>)`](crate::input::RegisterClientInput::scopes): <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    /// - On success, responds with [`RegisterClientOutput`](crate::output::RegisterClientOutput) with field(s):
    ///   - [`client_id(Option<String>)`](crate::output::RegisterClientOutput::client_id): <p>The unique identifier string for each client. This client uses this identifier to get authenticated by the service in subsequent calls.</p>
    ///   - [`client_secret(Option<String>)`](crate::output::RegisterClientOutput::client_secret): <p>A secret string generated for the client. The client will use this string to get authenticated by the service in subsequent calls.</p>
    ///   - [`client_id_issued_at(i64)`](crate::output::RegisterClientOutput::client_id_issued_at): <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> were issued.</p>
    ///   - [`client_secret_expires_at(i64)`](crate::output::RegisterClientOutput::client_secret_expires_at): <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> will become invalid.</p>
    ///   - [`authorization_endpoint(Option<String>)`](crate::output::RegisterClientOutput::authorization_endpoint): <p>The endpoint where the client can request authorization.</p>
    ///   - [`token_endpoint(Option<String>)`](crate::output::RegisterClientOutput::token_endpoint): <p>The endpoint where the client can get an access token.</p>
    /// - On failure, responds with [`SdkError<RegisterClientError>`](crate::error::RegisterClientError)
    pub fn register_client(&self) -> fluent_builders::RegisterClient<C, M, R> {
        fluent_builders::RegisterClient::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`StartDeviceAuthorization`](crate::client::fluent_builders::StartDeviceAuthorization) operation.
    ///
    /// - Takes [`StartDeviceAuthorizationInput`](crate::input::StartDeviceAuthorizationInput) with field(s):
    ///   - [`client_id(Option<String>)`](crate::input::StartDeviceAuthorizationInput::client_id): <p>The unique identifier string for the client that is registered with AWS SSO. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
    ///   - [`client_secret(Option<String>)`](crate::input::StartDeviceAuthorizationInput::client_secret): <p>A secret string that is generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
    ///   - [`start_url(Option<String>)`](crate::input::StartDeviceAuthorizationInput::start_url): <p>The URL for the AWS SSO user portal. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/using-the-portal.html">Using the User Portal</a> in the <i>AWS Single Sign-On User Guide</i>.</p>
    /// - On success, responds with [`StartDeviceAuthorizationOutput`](crate::output::StartDeviceAuthorizationOutput) with field(s):
    ///   - [`device_code(Option<String>)`](crate::output::StartDeviceAuthorizationOutput::device_code): <p>The short-lived code that is used by the device when polling for a session token.</p>
    ///   - [`user_code(Option<String>)`](crate::output::StartDeviceAuthorizationOutput::user_code): <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
    ///   - [`verification_uri(Option<String>)`](crate::output::StartDeviceAuthorizationOutput::verification_uri): <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
    ///   - [`verification_uri_complete(Option<String>)`](crate::output::StartDeviceAuthorizationOutput::verification_uri_complete): <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
    ///   - [`expires_in(i32)`](crate::output::StartDeviceAuthorizationOutput::expires_in): <p>Indicates the number of seconds in which the verification code will become invalid.</p>
    ///   - [`interval(i32)`](crate::output::StartDeviceAuthorizationOutput::interval): <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
    /// - On failure, responds with [`SdkError<StartDeviceAuthorizationError>`](crate::error::StartDeviceAuthorizationError)
    pub fn start_device_authorization(&self) -> fluent_builders::StartDeviceAuthorization<C, M, R> {
        fluent_builders::StartDeviceAuthorization::new(self.handle.clone())
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
    /// Fluent builder constructing a request to `CreateToken`.
    ///
    /// <p>Creates and returns an access token for the authorized client. The access token issued will be used to fetch short-term credentials for the assigned roles in the AWS account.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct CreateToken<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_token_input::Builder,
    }
    impl<C, M, R> CreateToken<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `CreateToken`.
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
            crate::output::CreateTokenOutput,
            aws_smithy_http::result::SdkError<crate::error::CreateTokenError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateTokenInputOperationOutputAlias,
                crate::output::CreateTokenOutput,
                crate::error::CreateTokenError,
                crate::input::CreateTokenInputOperationRetryAlias,
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
        /// <p>The unique identifier string for each client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
        pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_id(input.into());
            self
        }
        /// <p>The unique identifier string for each client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_id(input);
            self
        }
        /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
        pub fn client_secret(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_secret(input.into());
            self
        }
        /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
        pub fn set_client_secret(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_client_secret(input);
            self
        }
        /// <p>Supports grant types for authorization code, refresh token, and device code request.</p>
        pub fn grant_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.grant_type(input.into());
            self
        }
        /// <p>Supports grant types for authorization code, refresh token, and device code request.</p>
        pub fn set_grant_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_grant_type(input);
            self
        }
        /// <p>Used only when calling this API for the device code grant type. This short-term code is used to identify this authentication attempt. This should come from an in-memory reference to the result of the <code>StartDeviceAuthorization</code> API.</p>
        pub fn device_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_code(input.into());
            self
        }
        /// <p>Used only when calling this API for the device code grant type. This short-term code is used to identify this authentication attempt. This should come from an in-memory reference to the result of the <code>StartDeviceAuthorization</code> API.</p>
        pub fn set_device_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_code(input);
            self
        }
        /// <p>The authorization code received from the authorization service. This parameter is required to perform an authorization grant request to get access to a token.</p>
        pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.code(input.into());
            self
        }
        /// <p>The authorization code received from the authorization service. This parameter is required to perform an authorization grant request to get access to a token.</p>
        pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_code(input);
            self
        }
        /// <p>The token used to obtain an access token in the event that the access token is invalid or expired. This token is not issued by the service.</p>
        pub fn refresh_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.refresh_token(input.into());
            self
        }
        /// <p>The token used to obtain an access token in the event that the access token is invalid or expired. This token is not issued by the service.</p>
        pub fn set_refresh_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_refresh_token(input);
            self
        }
        /// Appends an item to `scope`.
        ///
        /// To override the contents of this collection use [`set_scope`](Self::set_scope).
        ///
        /// <p>The list of scopes that is defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
        pub fn scope(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.scope(input.into());
            self
        }
        /// <p>The list of scopes that is defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
        pub fn set_scope(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_scope(input);
            self
        }
        /// <p>The location of the application that will receive the authorization code. Users authorize the service to send the request to this location.</p>
        pub fn redirect_uri(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.redirect_uri(input.into());
            self
        }
        /// <p>The location of the application that will receive the authorization code. Users authorize the service to send the request to this location.</p>
        pub fn set_redirect_uri(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_redirect_uri(input);
            self
        }
    }
    /// Fluent builder constructing a request to `RegisterClient`.
    ///
    /// <p>Registers a client with AWS SSO. This allows clients to initiate device authorization. The output should be persisted for reuse through many authentication requests.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct RegisterClient<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::register_client_input::Builder,
    }
    impl<C, M, R> RegisterClient<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `RegisterClient`.
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
            crate::output::RegisterClientOutput,
            aws_smithy_http::result::SdkError<crate::error::RegisterClientError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::RegisterClientInputOperationOutputAlias,
                crate::output::RegisterClientOutput,
                crate::error::RegisterClientError,
                crate::input::RegisterClientInputOperationRetryAlias,
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
        /// <p>The friendly name of the client.</p>
        pub fn client_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_name(input.into());
            self
        }
        /// <p>The friendly name of the client.</p>
        pub fn set_client_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_name(input);
            self
        }
        /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
        pub fn client_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_type(input.into());
            self
        }
        /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
        pub fn set_client_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_type(input);
            self
        }
        /// Appends an item to `scopes`.
        ///
        /// To override the contents of this collection use [`set_scopes`](Self::set_scopes).
        ///
        /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
        pub fn scopes(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.scopes(input.into());
            self
        }
        /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
        pub fn set_scopes(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_scopes(input);
            self
        }
    }
    /// Fluent builder constructing a request to `StartDeviceAuthorization`.
    ///
    /// <p>Initiates device authorization by requesting a pair of verification codes from the authorization service.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct StartDeviceAuthorization<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::start_device_authorization_input::Builder,
    }
    impl<C, M, R> StartDeviceAuthorization<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `StartDeviceAuthorization`.
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
            crate::output::StartDeviceAuthorizationOutput,
            aws_smithy_http::result::SdkError<crate::error::StartDeviceAuthorizationError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::StartDeviceAuthorizationInputOperationOutputAlias,
                crate::output::StartDeviceAuthorizationOutput,
                crate::error::StartDeviceAuthorizationError,
                crate::input::StartDeviceAuthorizationInputOperationRetryAlias,
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
        /// <p>The unique identifier string for the client that is registered with AWS SSO. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
        pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_id(input.into());
            self
        }
        /// <p>The unique identifier string for the client that is registered with AWS SSO. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_client_id(input);
            self
        }
        /// <p>A secret string that is generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
        pub fn client_secret(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_secret(input.into());
            self
        }
        /// <p>A secret string that is generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API operation.</p>
        pub fn set_client_secret(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_client_secret(input);
            self
        }
        /// <p>The URL for the AWS SSO user portal. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/using-the-portal.html">Using the User Portal</a> in the <i>AWS Single Sign-On User Guide</i>.</p>
        pub fn start_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.start_url(input.into());
            self
        }
        /// <p>The URL for the AWS SSO user portal. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/using-the-portal.html">Using the User Portal</a> in the <i>AWS Single Sign-On User Guide</i>.</p>
        pub fn set_start_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_start_url(input);
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
