#[macro_export]
macro_rules! api_request_common {
    ($self:expr, $method:ident, $url:expr, $body:expr) => {{
        let mut request = $self.client.http_client().$method(&$url);

        if let Some(token) = &$self.client.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let request = if let Some(body) = $body {
            request.json(&body)
        } else {
            request
        };

        request.send().await.map_err(|e| crate::ApiError {
            status_code: 0,
            url: $url.clone(),
            request_body: None,
            response_body: e.to_string(),
            response_headers: std::collections::HashMap::new(),
            timestamp: None,
            path: None,
            message: Some(e.to_string()),
            error_code: None,
            error: None,
        })
    }};
}

#[macro_export]
macro_rules! api_get {
    ($controller:ident, $method_name:ident, $path:expr, $response_type:ty) => {
        impl $controller {
            #[doc = concat!("GET ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self) -> Result<$response_type, crate::ApiError> {
                let url = format!("{}{}", self.client.base_url(), $path);
                let response = api_request_common!(self, get, url, None::<()>)?;
                self.handle_response(response, url).await
            }
        }
    };
}

/// Macro for generating POST endpoints with body
#[macro_export]
macro_rules! api_post {
    ($controller:ident, $method_name:ident, $path:expr, $request_type:ty, $response_type:ty) => {
        impl $controller {
            #[doc = concat!("POST ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self, request: $request_type) -> Result<$response_type, crate::ApiError> {
                let url = format!("{}{}", self.client.base_url(), $path);
                let response = api_request_common!(self, post, url, Some(request))?;
                self.handle_response(response, url).await
            }
        }
    };
}

/// Macro for generating POST endpoints with no body
#[macro_export]
macro_rules! api_post_no_body {
    ($controller:ident, $method_name:ident, $path:expr, $response_type:ty) => {
        impl $controller {
            #[doc = concat!("POST ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self) -> Result<$response_type, crate::ApiError> {
                let url = format!("{}{}", self.client.base_url(), $path);
                let response = api_request_common!(self, post, url, None::<()>)?;
                self.handle_response(response, url).await
            }
        }
    };
}

/// Macro for generating GET endpoints with path parameters
#[macro_export]
macro_rules! api_get_with_path {
    ($controller:ident, $method_name:ident, $path:expr, $response_type:ty, $($param:ident: $param_type:ty),*) => {
        impl $controller {
            #[doc = concat!("GET ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self, $($param: $param_type),*) -> Result<$response_type, crate::ApiError> {
                let url = format!("{}{}", self.client.base_url(), format!($path, $($param),*));
                let response = api_request_common!(self, get, url, None::<()>)?;
                self.handle_response(response, url).await
            }
        }
    };
}

/// Macro for generating GET endpoints with query parameters
#[macro_export]
macro_rules! api_get_with_query {
    ($controller:ident, $method_name:ident, $path:expr, $response_type:ty, $($param:ident: $param_type:ty),*) => {
        impl $controller {
            #[doc = concat!("GET ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self, $($param: $param_type),*) -> Result<$response_type, crate::ApiError> {
                let mut url = format!("{}{}", self.client.base_url(), $path);

                let mut query_params = Vec::new();
                $(
                    if let Some(value) = $param.as_ref() {
                        query_params.push(format!("{}={}", stringify!($param), value));
                    }
                )*

                if !query_params.is_empty() {
                    url = format!("{}?{}", url, query_params.join("&"));
                }

                let response = api_request_common!(self, get, url, None::<()>)?;
                self.handle_response(response, url).await
            }
        }
    };
}

/// Macro for generating GET endpoints with both path and query parameters
#[macro_export]
macro_rules! api_get_with_path_and_query {
    ($controller:ident, $method_name:ident, $path:expr, $response_type:ty, path_params: [$($path_param:ident: $path_param_type:ty),*], query_params: [$($query_param:ident: $query_param_type:ty),*]) => {
        impl $controller {
            #[doc = concat!("GET ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self, $($path_param: $path_param_type,)* $($query_param: $query_param_type),*) -> Result<$response_type, crate::ApiError> {
                let mut url = format!("{}{}", self.client.base_url(), format!($path, $($path_param),*));

                let mut query_params = Vec::new();
                $(
                    if let Some(value) = $query_param.as_ref() {
                        query_params.push(format!("{}={}", stringify!($query_param), value));
                    }
                )*

                if !query_params.is_empty() {
                    url = format!("{}?{}", url, query_params.join("&"));
                }

                let response = api_request_common!(self, get, url, None::<()>)?;
                self.handle_response(response, url).await
            }
        }
    };
}

/// Macro for generating PATCH endpoints with request body
#[macro_export]
macro_rules! api_patch {
    ($controller:ident, $method_name:ident, $path:expr, $request_type:ty, $response_type:ty) => {
        impl $controller {
            #[doc = concat!("PATCH ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self, request: $request_type) -> Result<$response_type, crate::ApiError> {
                let url = format!("{}{}", self.client.base_url(), $path);
                let response = api_request_common!(self, patch, url, Some(request))?;
                self.handle_response(response, url).await
            }
        }
    };
}

/// Macro for generating DELETE endpoints with path parameters
#[macro_export]
macro_rules! api_delete {
    ($controller:ident, $method_name:ident, $path:expr, $response_type:ty, $param:ident: $param_type:ty) => {
        impl $controller {
            #[doc = concat!("DELETE ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self, $param: $param_type) -> Result<$response_type, crate::ApiError> {
                let url = format!("{}{}", self.client.base_url(), $path.replace(&format!("{{{}}}", stringify!($param)), &$param.to_string()));
                let response = api_request_common!(self, delete, url, None::<()>)?;
                self.handle_response(response, url).await
            }
        }
    };
}

#[macro_export]
macro_rules! api_post_with_path {
    ($controller:ident, $method_name:ident, $path:expr, $request_type:ty, $response_type:ty, $($param:ident: $param_type:ty),*) => {
        impl $controller {
            #[doc = concat!("POST ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self, $($param: $param_type,)* request: $request_type) -> Result<$response_type, crate::ApiError> {
                let url = format!("{}{}", self.client.base_url(), format!($path, $($param),*));
                let response = api_request_common!(self, post, url, Some(request))?;
                self.handle_response(response, url).await
            }
        }
    };
}

/// Macro for generating POST endpoints with path parameters and no request body
#[macro_export]
macro_rules! api_post_with_path_no_body {
    ($controller:ident, $method_name:ident, $path:expr, $response_type:ty, $($param:ident: $param_type:ty),*) => {
        impl $controller {
            #[doc = concat!("POST ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self, $($param: $param_type),*) -> Result<$response_type, crate::ApiError> {
                let url = format!("{}{}", self.client.base_url(), format!($path, $($param),*));
                let response = api_request_common!(self, post, url, None::<()>)?;
                self.handle_response(response, url).await
            }
        }
    };
}

/// Macro for generating PATCH endpoints with path parameters
#[macro_export]
macro_rules! api_patch_with_path {
    ($controller:ident, $method_name:ident, $path:expr, $request_type:ty, $response_type:ty, $($param:ident: $param_type:ty),*) => {
        impl $controller {
            #[doc = concat!("PATCH ", $path, " - ", stringify!($controller))]
            pub async fn $method_name(&self, $($param: $param_type,)* request: $request_type) -> Result<$response_type, crate::ApiError> {
                let url = format!("{}{}", self.client.base_url(), format!($path, $($param),*));
                let response = api_request_common!(self, patch, url, Some(request))?;
                self.handle_response(response, url).await
            }
        }
    };
}

/// Macro for generating controller structs
#[macro_export]
macro_rules! api_controller {
    ($controller:ident) => {
        pub struct $controller {
            client: std::sync::Arc<crate::api::client::ApiClient>,
        }

        impl $controller {
            pub fn new(client: std::sync::Arc<crate::api::client::ApiClient>) -> Self {
                Self {
                    client,
                }
            }

            async fn handle_response<T>(&self, response: reqwest::Response, url: String) -> Result<T, crate::ApiError>
            where
                T: serde::de::DeserializeOwned,
            {
                let status = response.status();
                let response_headers: std::collections::HashMap<String, String> =
                    response.headers().iter().filter_map(|(name, value)| value.to_str().ok().map(|v| (name.to_string(), v.to_string()))).collect();

                if status.is_success() {
                    let response_text = response.text().await.map_err(|e| crate::ApiError {
                        status_code: status.as_u16(),
                        url: url.clone(),
                        request_body: None,
                        response_body: e.to_string(),
                        response_headers: response_headers.clone(),
                        timestamp: None,
                        path: None,
                        message: Some("Failed to read response body".to_string()),
                        error_code: None,
                        error: None,
                    })?;

                    serde_json::from_str(&response_text).map_err(|e| crate::ApiError {
                        status_code: status.as_u16(),
                        url: url.clone(),
                        request_body: None,
                        response_body: response_text,
                        response_headers,
                        timestamp: None,
                        path: None,
                        message: Some(format!("Failed to deserialize response: {}", e)),
                        error_code: None,
                        error: None,
                    })
                } else {
                    let response_body = response.text().await.unwrap_or_default();

                    // Try to parse error details from response
                    let (timestamp, path, message, error_code, error) = if !response_body.is_empty() {
                        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&response_body) {
                            let timestamp = json_value.get("timestamp").and_then(|v| v.as_str()).map(|s| s.to_string());
                            let path = json_value.get("path").and_then(|v| v.as_str()).map(|s| s.to_string());
                            let message = json_value.get("message").and_then(|v| v.as_str()).map(|s| s.to_string());
                            let error_code = json_value.get("errorCode").and_then(|v| v.as_str()).map(|s| s.to_string());
                            let error = json_value.get("error").and_then(|v| v.as_str()).map(|s| s.to_string());
                            (timestamp, path, message, error_code, error)
                        } else {
                            (None, None, None, None, None)
                        }
                    } else {
                        (None, None, None, None, None)
                    };

                    Err(crate::ApiError {
                        status_code: status.as_u16(),
                        url,
                        request_body: None,
                        response_body,
                        response_headers,
                        timestamp,
                        path,
                        message,
                        error_code,
                        error,
                    })
                }
            }
        }
    };
}
