//! The macros here are for generating request builders and specifying typed relationships to API
//! paths and types.
//!
//! # Example
//!
//! View documentation on each for macro for more information on how each macro works.
//!
//! ```rust,ignore
//! // Common imports used by macros.
//! imports!();
//!
//! // Define request builder types.
//! new_builder!(
//!     User,
//!     Users,
//! );
//!
//! // Request builders we intend to extend must be imported separately.
//! use crate::client::GetQueryBuilder;
//!
//! // Generate `From<GetQueryBuilder>` implementations for our request builders.
//! from!(
//!     @GetQuery
//!         -> User,
//!         -> Users,
//! );
//!
//! // Implement methods on GetQueryBuilder for accessing our new request builder types.
//! impl_builder!(
//!     @GetQuery
//!         -> users ["users"] -> Users,
//!         => user  ["users"] -> User = id,
//! );
//!
//! // Mark request builders as executable and specify the return type.
//! exec!(
//!     User  -> crate::models::User,
//!     Users -> Vec<crate::models::User>,
//! );
//! ```

/// Necessary imports for when all macros are used within a file.
macro_rules! imports {
    () => {
        #[cfg(feature = "rustls")]
        type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
        #[cfg(feature = "rust-native-tls")]
        use hyper_tls;
        #[cfg(feature = "rust-native-tls")]
        type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;
        use async_trait::async_trait;
        use hyper::client::Client;
        use paste::paste;
        use std::error::Error;
        use std::sync::Arc;
        use std::sync::RwLock;

        use $crate::builder::Builder;
        use $crate::client::ApiResponse;
        use $crate::client::Executor;

        // This is only used in the `impl_client!` macro.
        #[allow(unused_imports)]
        use std::collections::HashMap;
    };
}

/// Specifies new request builders. Documentation is passed through.
///
/// # Example
///
/// ```
/// new_builder!(
///     User,
///     /// Documentation is passed through!
///     Users,
/// );
///
/// // Expands to ...
///
/// pub struct UserBuilder { ... }
/// unsafe impl Send for UserBuilder {}
/// /// Documentation is passed through!
/// pub struct UsersBuilder { ... }
/// unsafe impl Send for UsersBuilder {}
/// ```
macro_rules! new_builder {
    ($(
        $(#[$doc:meta])*
        $i: ident
    ),* $(,)?) => (
        $(paste! {
            $(#[$doc])*
            pub struct [<$i Builder>] {
                pub(crate) builder: Result<RwLock<Builder>, Box<dyn Error>>,
                pub(crate) client: Arc<Client<HttpsConnector>>,
            }
            unsafe impl Send for [<$i Builder>] {}
        })*
    );
}

/// Marks a request builder as executable by implementing the `Executor` trait and specifying a
/// return type. Documentation is passed through.
///
/// # Example
///
/// ```
/// exec!(
/// // builder     return type (implements serde::Deserialize)
/// //   ||             ||
/// //   \/             \/
///     User     -> models::User,      // Returns a struct.
///     Users    -> Vec<models::User>, // Returns an array of structs.
///     /// Documentation is passed through!
///     NoReturn -> (),                // Returns no body.
/// );
///
/// // Expands to ...
///
/// impl Executor for UserBuilder {
///     type T = models::User;
///
///     async fn execute(self) -> Result<ApiResponse<models::User>, Box<dyn Error>> { ... }
/// }
/// impl Executor for UsersBuilder {
///     type T = models::User;
///
///     async fn execute(self) -> Result<ApiResponse<Vec<models::User>>, Box<dyn Error>> { ... }
/// }
/// /// Documentation is passed through!
/// impl Executor for NoReturnBuilder {
///     type T = ();
///
///     async fn execute(self) -> Result<ApiResponse<()>, Box<dyn Error>> { ... }
/// }
/// ```
macro_rules! exec {
    ($(
        $(#[$doc:meta])*
        $i: ident -> $t: ty
    ),* $(,)?
    ) => (
        paste! {$(
            $(#[$doc])*
            #[async_trait]
            impl Executor for [<$i Builder>] {
                type T = $t;

                async fn execute(self) -> Result<ApiResponse<Self::T>, Box<dyn Error>> {
                    let client = self.client;
                    let builder = self.builder?.into_inner().unwrap();
                    let req = builder.build();
                    // dbg!(&req);
                    let res = client.request(req).await?;
                    let (parts, body) = res.into_parts();

                    let body = hyper::body::to_bytes(body).await?;
                    let body = if parts.status.is_success() {
                        Ok(serde_json::from_slice::<Self::T>(&body)?)
                    } else {
                        Err(serde_json::from_slice::<$crate::client::ApiError>(&body)?)
                    };

                    Ok(ApiResponse {
                        status_code: parts.status,
                        headers: parts.headers.into(),
                        response: body,
                    })
                }
            }
        )*}
    );
}

/// Generates `From<T>` implementations for converting typed request builders into other typed
/// request builders. The implementations here are used in the `impl_builder!` macro. Documentation
/// is passed through for target builders.
///
/// # Example
///
/// ```
/// from!(
///     // source builder
///     //    ||
///     //    \/
///     @GetQuery
///         -> User, // <= target builder
///         /// Documentation is passed through!
///         -> Users,
/// );
///
/// // Expands to ...
///
/// impl From<GetQueryBuilder> for UserBuilder {
///     fn from(f: GetQueryBuilder) -> Self { ... }
/// }
/// /// Documentation is passed through!
/// impl From<GetQueryBuilder> for UsersBuilder {
///     fn from(f: GetQueryBuilder) -> Self { ... }
/// }
/// ```
macro_rules! from {
    ($(@$f: ident
        $(
            $(#[$doc:meta])*
            -> $t: ident
        ),+ $(,)?
    )+) => {
        $($(paste! {
            $(#[$doc])*
            impl From<[<$f Builder>]> for [<$t Builder>] {
                fn from(f: [<$f Builder>]) -> Self {
                    Self {
                        builder: f.builder,
                        client: f.client,
                    }
                }
            }
        })+)+
    };
}

/// Generates code for extending request builders into other request builders. All builders must be
/// created by the `new_builder!` macro and have conversions specified by the `from!` macro.
///
/// # Example
///
/// ```
/// impl_builder!(
///     // source builder
///     //    ||
///     //    \/
///     @GetQuery
///         // There are two different types of impls we can generate:
///         //   1. `->` which generates an impl requiring no route variable.
///         //   2. `=>` which generates an impl requiring a route variable.
///         //       The route variable will be appended to the provided route path.
///         //
///         // method name      new builder
///         //  ||   route path     ||
///         //  ||       ||         ||  route variable name
///         //  \/       \/         \/          ||
///         -> users ["users"] -> Users,     // ||
///         /// Docs are passed through too!    \/
///         => user  ["users"] -> User       = id,
/// );
///
/// // Expands to ...
///
/// impl GetQueryBuilder {
///     pub fn users(mut self) -> UsersBuilder { ... }
///     /// Docs are passed through too!
///     pub fn user<T: ToString>(mut self, id: T) -> UserBuilder { ... }
/// }
/// ```
macro_rules! impl_builder {
    // This first line should contain @ and the struct we are implementing from, like @GetQuery.
    // The struct should have been generated from the new_builder! macro.
    ($(@$i: ident
       $(
           $(#[$doc:meta])*
            // Case 1
            // This case is for methods that don't need a route variable such, as getting all
            // users.
            // The syntax looks like: `-> <method name> [<route path>] -> <builder name>`.
            // Builder name should be a struct generated by the new_builder! macro.
            $(-> $fn1:ident [$($p1:literal)?] -> $t1:ident)?

            // Case 2
            // This case is for methods that need a route variable such as getting a user by id.
            // The syntax looks like: `=> <method name> [<route path>] -> <builder name> = <path variable name>`
            // Builder name should be a struct generated by the new_builder! macro.
            $(=> $fn2:ident [$($p2:literal)?] -> $t2:ident = $e2:ident)?

            $(?> $fn3:ident [$q3:literal] -> $n3:ident: $t3:ty)?
        ),*
    )+)=> (
        $(paste! {
            impl [<$i Builder>] {
            $(
                $(#[$doc])*
                // Case 1
                $(
                    pub fn $fn1(mut self) -> [<$t1 Builder>] {
                        join_path!(self, &[$($p1)?]);
                        self.into()
                    }
                )?
                // Case 2
                $(
                    pub fn $fn2<T: ToString>(mut self, $e2: T) -> [<$t2 Builder>] {
                        join_path!(self, &[$($p2,)? &$e2.to_string()]);
                        self.into()
                    }
                )?
                // Case 3
                $(
                    pub fn $fn3(mut self, $n3: $t3) -> [<$i Builder>] {
                        join_query!(self, $q3, $n3);
                        self.into()
                    }
                )?
            )*
            }
        })+
    );
}

macro_rules! impl_client {
    ($(
       $(#[$doc:meta])*
        $(-> $fn:ident [$p:literal] -> $t:ident)?
    ),*)=> (
        impl $crate::client::Coder {
            $(paste! {
                $(#[$doc])*
                $(
                    pub fn $fn(&self) -> [<$t Builder>] {
                        let mut b = [<$t Builder>] {
                            builder: self.new_request().map(|r| RwLock::new(Builder{
                                query: HashMap::new(),
                                url: self.url.clone(),
                                req: r,

                            })),
                            client: Arc::clone(&self.client),
                        };
                        join_path!(b, &[$p]);
                        b
                    }
                )?
            })*
        }
    );
}

macro_rules! join_path {
    ($e: ident, $p: expr) => {
        if $e.builder.is_ok() {
            // We've checked that this works
            let inner = $e.builder.unwrap();
            inner
                .write()
                .unwrap()
                .url
                .path_segments_mut()
                .unwrap()
                .extend($p);

            $e.builder = Ok(inner);
        }
    };
}

macro_rules! join_query {
    ($e: ident, $k: expr, $v: expr) => {
        if $e.builder.is_ok() {
            // We've checked that this works
            let inner = $e.builder.unwrap();
            inner.write().unwrap().query.insert($k, $v.to_string());

            $e.builder = Ok(inner);
        }
    };
}

macro_rules! id_string {
    ($($name:ident),*) => {
        $(
            impl std::ops::Deref for $name {
                type Target = String;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::convert::From<String> for $name {
                fn from(i: String) -> Self {
                    Self(i)
                }
            }

            impl std::convert::From<&str> for $name {
                fn from(i: &str) -> Self {
                    Self(i.to_string())
                }
            }
        )*
    }
}
