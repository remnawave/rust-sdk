fn main() {
    let spec_path = "api.json";
    println!("cargo:rerun-if-changed={}", spec_path);

    let file = std::fs::File::open(spec_path).unwrap();
    let mut spec: openapiv3::OpenAPI = serde_json::from_reader(file).unwrap();

    // Fix progenitor issue with multiple response types
    // Remove "default" responses when specific status codes are present
    fix_response_conflicts(&mut spec);

    let mut binding = progenitor::GenerationSettings::default();
    let settings = binding
        .with_interface(progenitor::InterfaceStyle::Builder);

    let mut generator = progenitor::Generator::new(&settings);
    let tokens = generator.generate_tokens(&spec).unwrap();

    let ast = syn::parse2(tokens).unwrap();
    let src = prettyplease::unparse(&ast);

    // Генерируем код контроллеров
    let (controller_structs, client_methods) = generate_controllers(&spec);

    let out_file = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("remnawave_api.rs");
    
    let controllers_file = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("controllers.rs");

    // Write only the original generated API
    std::fs::write(out_file, src).unwrap();
    
    // Write our custom controllers separately  
    std::fs::write(controllers_file, format!("{}\n{}", controller_structs, client_methods)).unwrap();
}

fn fix_response_conflicts(spec: &mut openapiv3::OpenAPI) {
    for path_item in spec.paths.paths.values_mut() {
        if let openapiv3::ReferenceOr::Item(path_item) = path_item {
            // Check all operations in the path
            let operations = [
                &mut path_item.get,
                &mut path_item.post,
                &mut path_item.put,
                &mut path_item.delete,
                &mut path_item.options,
                &mut path_item.head,
                &mut path_item.patch,
                &mut path_item.trace,
            ];

            for operation in operations.into_iter().filter_map(|op| op.as_mut()) {
                fix_operation_responses(&mut operation.responses);
            }
        }
    }
}

fn fix_operation_responses(responses: &mut openapiv3::Responses) {
    // If there are both specific status codes and a default response,
    // remove the default response to avoid progenitor conflicts
    let has_specific_responses = responses.responses.keys().any(|key| match key {
        openapiv3::StatusCode::Code(_) => true,
        openapiv3::StatusCode::Range(_) => true,
    });

    if has_specific_responses && responses.default.is_some() {
        // Remove the default response to resolve the conflict
        responses.default = None;
    }
}

use std::collections::HashMap;

fn generate_controllers(spec: &openapiv3::OpenAPI) -> (String, String) {
    let mut controllers: HashMap<String, Vec<(String, String, String, String)>> = HashMap::new();
    
    // Анализируем все пути и группируем по контроллерам на основе operation_id
    for (_path, path_item) in &spec.paths.paths {
        if let openapiv3::ReferenceOr::Item(path_item) = path_item {
            let operations = [
                ("GET", &path_item.get),
                ("POST", &path_item.post),
                ("PUT", &path_item.put),
                ("DELETE", &path_item.delete),
                ("PATCH", &path_item.patch),
                ("HEAD", &path_item.head),
                ("OPTIONS", &path_item.options),
                ("TRACE", &path_item.trace),
            ];

            for (method, operation) in operations {
                if let Some(operation) = operation {
                    if let Some(operation_id) = &operation.operation_id {
                        // Группируем по контроллерам на основе operation_id
                        if let Some((controller_name, method_name)) = extract_controller_from_operation_id(operation_id) {
                            let summary = operation.summary.clone().unwrap_or_default();
                            
                            controllers
                                .entry(controller_name)
                                .or_insert_with(Vec::new)
                                .push((method_name, operation_id.clone(), summary, method.to_string()));
                        }
                    }
                }
            }
        }
    }

    // Удаляем дублирующиеся методы
    for methods in controllers.values_mut() {
        methods.sort_by(|a, b| a.0.cmp(&b.0));
        methods.dedup_by(|a, b| a.0 == b.0);
    }

    // Генерируем код контроллеров
    let controller_structs = generate_controller_code(&controllers);
    let client_methods = generate_client_methods(&controllers);
    
    (controller_structs, client_methods)
}

fn extract_controller_from_operation_id(operation_id: &str) -> Option<(String, String)> {
    // operation_id имеет вид: "AuthController_login" или "UsersController_createUser"
    // Разбиваем на части по символу "_"
    let parts: Vec<&str> = operation_id.split('_').collect();
    
    if parts.len() < 2 {
        return None;
    }
    
    // Ищем "Controller" в первой части
    let controller_part = parts[0];
    if !controller_part.ends_with("Controller") {
        return None;
    }
    
    // Извлекаем имя контроллера (убираем "Controller")
    let controller_base = &controller_part[0..controller_part.len() - 10]; // убираем "Controller"
    let controller_name = match controller_base {
        "Auth" => "auth".to_string(),
        "Users" => "users".to_string(),
        "UsersBulkActions" => "users".to_string(),
        "Nodes" => "nodes".to_string(),
        "NodesUserUsageHistory" => "nodes".to_string(),
        "NodesUsageHistory" => "nodes".to_string(),
        "Hosts" => "hosts".to_string(),
        "HostsBulkActions" => "hosts".to_string(),
        "Inbounds" => "inbounds".to_string(),
        "InboundsBulkActions" => "inbounds".to_string(),
        "ApiTokens" => "api_tokens".to_string(),
        "Keygen" => "keygen".to_string(),
        "HwidUserDevices" => "hwid".to_string(),
        "SubscriptionSettings" => "subscription_settings".to_string(),
        "SubscriptionTemplate" => "subscription_templates".to_string(),
        "SubscriptionTemplates" => "subscription_templates".to_string(),
        "Subscription" => "subscriptions".to_string(),
        "Subscriptions" => "subscriptions".to_string(),
        "XrayConfig" => "xray_config".to_string(),
        "System" => "system".to_string(),
        other => other.to_lowercase(),
    };
    
    // Извлекаем имя метода (все части после первой)
    let method_parts = &parts[1..];
    let method_name = create_method_name_from_operation(method_parts, &controller_name, controller_base);
    
    Some((controller_name, method_name))
}

fn create_method_name_from_operation(parts: &[&str], controller_name: &str, _controller_base: &str) -> String {
    if parts.is_empty() {
        return "unknown".to_string();
    }
    
    let method_str = parts.join("_");
    let method_lower = method_str.to_lowercase();
    
    // Маппинг специальных случаев для разных контроллеров
    match controller_name {
        "subscriptions" => {
            match method_lower.as_str() {
                "getsubscriptioninfobyshortuuid" => "get_info".to_string(),
                "getsubscription" => "get".to_string(),
                "getsubscriptionbyclienttype" => "get_by_client_type".to_string(),
                "getsubscriptionwithtype" => "get_with_type".to_string(),
                "getallsubscriptions" => "get_all".to_string(),
                "getsubscriptionbyusername" => "get_by_username".to_string(),
                _ => snake_case(&method_str),
            }
        }
        "users" => {
            match method_lower.as_str() {
                "bulkdeleteusers" => "bulk_delete".to_string(),
                "bulkdeleteusersbystatus" => "bulk_delete_by_status".to_string(),
                "bulkrevokeusersubscription" => "bulk_revoke_subscription".to_string(),
                "bulkresetusertraffic" => "bulk_reset_traffic".to_string(),
                "bulkupdateusers" => "bulk_update".to_string(),
                "bulkupdateusersinbounds" => "bulk_update_inbounds".to_string(),
                "bulkupdateallusers" => "bulk_update_all".to_string(),
                "bulkallresetusertraffic" => "bulk_reset_all_traffic".to_string(),
                "getallusers" => "get_all".to_string(),
                "createuser" => "create".to_string(),
                "updateuser" => "update".to_string(),
                "getuserbyuuid" => "get_by_uuid".to_string(),
                "deleteuser" => "delete".to_string(),
                "getalltags" => "get_tags".to_string(),
                "getuserbyshortuuid" => "get_by_short_uuid".to_string(),
                "getuserbysubscriptionuuid" => "get_by_subscription_uuid".to_string(),
                "getuserbyusername" => "get_by_username".to_string(),
                "getuserbytelegramid" => "get_by_telegram_id".to_string(),
                "getusersbyemail" => "get_by_email".to_string(),
                "getusersbytag" => "get_by_tag".to_string(),
                "revokeusersubscription" => "revoke_subscription".to_string(),
                "disableuser" => "disable".to_string(),
                "enableuser" => "enable".to_string(),
                "resetusertraffic" => "reset_traffic".to_string(),
                "activateallinbounds" => "activate_all_inbounds".to_string(),
                _ => snake_case(&method_str),
            }
        }
        "subscription_templates" => {
            match method_lower.as_str() {
                "getallsubscriptiontemplates" => "get_all".to_string(),
                "createsubscriptiontemplate" => "create".to_string(),
                "gettemplate" => "get".to_string(),
                "updatetemplate" => "update".to_string(),
                "deletesubscriptiontemplate" => "delete".to_string(),
                _ => snake_case(&method_str),
            }
        }
        "auth" => {
            match method_lower.as_str() {
                "login" => "login".to_string(),
                "register" => "register".to_string(),
                "getstatus" => "get_status".to_string(),
                "telegramcallback" => "telegram_callback".to_string(),
                _ => snake_case(&method_str),
            }
        }
        _ => snake_case(&method_str),
    }
}

fn controller_name_to_struct_name(controller_name: &str) -> String {
    // Преобразуем "auth" в "AuthController"
    format!("{}Controller", to_pascal_case(controller_name))
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn controller_name_to_method_name(controller_name: &str) -> String {
    // Преобразуем "AuthController" в "auth"
    if controller_name.ends_with("Controller") {
        let base = &controller_name[..controller_name.len() - 10]; // убираем "Controller"
        snake_case(&base)
    } else {
        snake_case(controller_name)
    }
}

fn snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

fn generate_controller_code(controllers: &HashMap<String, Vec<(String, String, String, String)>>) -> String {
    let mut code = String::new();
    
    // Генерируем структуры контроллеров
    for (controller_name, methods) in controllers {
        let struct_name = controller_name_to_struct_name(controller_name);
        
        code.push_str(&format!(
            "/// {} methods wrapper\npub struct {}<'a> {{\n    client: &'a Client,\n}}\n\n",
            controller_name, struct_name
        ));
        
        code.push_str(&format!("impl<'a> {}<'a> {{\n", struct_name));
        
        for (method_name, operation_id, summary, _http_method) in methods {
            let clean_method_name = method_name.to_lowercase();
            
            code.push_str(&format!(
                "    /// {}\n    pub fn {}(&self) -> builder::{} {{\n        self.client.{}()\n    }}\n\n",
                summary,
                clean_method_name,
                operation_id_to_builder_name(operation_id),
                to_snake_case_operation_id(operation_id)
            ));
        }
        
        code.push_str("}\n\n");
    }

    code
}

fn generate_controller_code_with_params(controllers: &HashMap<String, Vec<(String, String, String, String, Vec<(String, String)>, Option<Vec<(String, String)>>)>>) -> String {
    let mut code = String::new();
    
    // Генерируем структуры контроллеров
    for (controller_name, methods) in controllers {
        let struct_name = controller_name_to_struct_name(controller_name);
        
        code.push_str(&format!(
            "/// {} methods wrapper\npub struct {}<'a> {{\n    client: &'a Client,\n}}\n\n",
            controller_name, struct_name
        ));
        
        code.push_str(&format!("impl<'a> {}<'a> {{\n", struct_name));
        
        for (method_name, operation_id, summary, _http_method, path_params, body_params) in methods {
            let clean_method_name = method_name.to_lowercase();
            
            // Generate function signature with actual parameters
            let mut fn_params = Vec::new();
            fn_params.push("&self".to_string());
            
            // Add path parameters
            for (param_name, param_type) in path_params {
                fn_params.push(format!("{}: {}", param_name, param_type));
            }
            
            // Add body parameters as individual arguments
            if let Some(body_fields) = body_params {
                for (field_name, field_type) in body_fields {
                    fn_params.push(format!("{}: {}", field_name, field_type));
                }
            }
            
            let fn_signature = fn_params.join(", ");
            let return_type = format!("impl std::future::Future<Output = Result<ResponseValue<ByteStream>, Error<types::{}Response>>> + '_", 
                operation_id_to_builder_name(operation_id));
            
            code.push_str(&format!(
                "    /// {}\n    pub fn {}({}) -> {} {{\n",
                summary, clean_method_name, fn_signature, return_type
            ));
            
            // Generate function body that constructs the request
            if body_params.is_some() && !body_params.as_ref().unwrap().is_empty() {
                // POST/PUT/PATCH with body
                let body_fields = body_params.as_ref().unwrap();
                code.push_str("        async move {\n");
                code.push_str(&format!("            let mut builder = self.client.{}();\n", to_snake_case_operation_id(operation_id)));
                
                if body_fields.len() == 2 && body_fields.iter().any(|(name, _)| name == "username") && body_fields.iter().any(|(name, _)| name == "password") {
                    // Special case for login-like endpoints
                    code.push_str(&format!(
                        "            builder = builder.body_map(|b| b.username(username).password(password));\n"
                    ));
                } else {
                    // Generic body construction
                    let field_calls: Vec<String> = body_fields.iter()
                        .map(|(field_name, _)| format!(".{}({})", field_name, field_name))
                        .collect();
                    code.push_str(&format!(
                        "            builder = builder.body_map(|b| b{});\n",
                        field_calls.join("")
                    ));
                }
                
                code.push_str("            builder.send().await\n");
                code.push_str("        }\n");
            } else {
                // GET/DELETE without body
                code.push_str("        async move {\n");
                code.push_str(&format!("            self.client.{}().send().await\n", to_snake_case_operation_id(operation_id)));
                code.push_str("        }\n");
            }
            
            code.push_str("    }\n\n");
        }
        
        code.push_str("}\n\n");
    }

    code
}

fn generate_client_methods(controllers: &HashMap<String, Vec<(String, String, String, String)>>) -> String {
    let mut code = String::new();
    
    code.push_str("impl RemnawaveClient {\n");
    
    for (controller_name, _methods) in controllers {
        let struct_name = controller_name_to_struct_name(controller_name);
        let method_name = controller_name_to_method_name(controller_name);
        
        code.push_str(&format!(
            "    /// {} methods\n    pub fn {}(&self) -> {} {{\n        {} {{ client: &self.client }}\n    }}\n\n",
            controller_name, method_name, struct_name, struct_name
        ));
    }
    
    code.push_str("}\n");
    code
}

fn generate_client_methods_simple(controllers: &HashMap<String, Vec<(String, String, String, String, Vec<(String, String)>, Option<Vec<(String, String)>>)>>) -> String {
    let mut code = String::new();
    
    code.push_str("impl RemnawaveClient {\n");
    
    for (controller_name, _methods) in controllers {
        let struct_name = controller_name_to_struct_name(controller_name);
        let method_name = controller_name_to_method_name(controller_name);
        
        code.push_str(&format!(
            "    /// {} methods\n    pub fn {}(&self) -> {} {{\n        {} {{ client: &self.client }}\n    }}\n\n",
            controller_name, method_name, struct_name, struct_name
        ));
    }
    
    code.push_str("}\n");
    code
}

// Add OpenAPI parameter analysis functions
fn analyze_operation_parameters(spec: &openapiv3::OpenAPI, operation: &openapiv3::Operation) -> (Vec<(String, String)>, Option<Vec<(String, String)>>) {
    let mut path_params = Vec::new();
    let mut query_params = Vec::new();
    let mut body_params = None;
    
    // Analyze parameters  
    for param_ref in &operation.parameters {
        if let openapiv3::ReferenceOr::Item(param) = param_ref {
            let _param_name = match param {
                openapiv3::Parameter::Path { parameter_data, .. } => {
                    path_params.push((parameter_data.name.clone(), "String".to_string()));
                    continue;
                }
                openapiv3::Parameter::Query { parameter_data, .. } => {
                    query_params.push((parameter_data.name.clone(), "String".to_string()));
                    continue;
                }
                openapiv3::Parameter::Header { parameter_data: _, .. } => {
                    continue; // Skip headers for now
                }
                openapiv3::Parameter::Cookie { parameter_data: _, .. } => {
                    continue; // Skip cookies for now
                }
            };
        }
    }
    
    // Analyze request body
    if let Some(request_body_ref) = &operation.request_body {
        if let openapiv3::ReferenceOr::Item(request_body) = request_body_ref {
            for (content_type, media_type) in &request_body.content {
                if content_type == "application/json" {
                    if let Some(schema_ref) = &media_type.schema {
                        body_params = Some(analyze_schema_for_parameters(spec, schema_ref));
                    }
                }
            }
        }
    }
    
    let mut all_params = path_params;
    all_params.extend(query_params);
    
    (all_params, body_params)
}

fn analyze_schema_for_parameters(spec: &openapiv3::OpenAPI, schema_ref: &openapiv3::ReferenceOr<openapiv3::Schema>) -> Vec<(String, String)> {
    match schema_ref {
        openapiv3::ReferenceOr::Reference { reference } => {
            // Extract schema name from reference like "#/components/schemas/LoginRequestDto"
            if let Some(schema_name) = reference.strip_prefix("#/components/schemas/") {
                if let Some(schema) = spec.components.as_ref()
                    .and_then(|c| c.schemas.get(schema_name)) {
                    if let openapiv3::ReferenceOr::Item(schema) = schema {
                        return extract_parameters_from_schema(schema);
                    }
                }
            }
            vec![("body".to_string(), "String".to_string())]
        }
        openapiv3::ReferenceOr::Item(schema) => {
            extract_parameters_from_schema(schema)
        }
    }
}

fn extract_parameters_from_schema(schema: &openapiv3::Schema) -> Vec<(String, String)> {
    match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(obj)) => {
            let mut params = Vec::new();
            for (prop_name, _prop_schema) in &obj.properties {
                // Simplified type detection - could be enhanced
                params.push((prop_name.clone(), "String".to_string()));
            }
            params
        }
        _ => vec![("body".to_string(), "String".to_string())],
    }
}

fn operation_id_to_builder_name(operation_id: &str) -> String {
    // Преобразуем "AuthController_login" в "AuthControllerLogin"
    operation_id
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn to_snake_case_operation_id(operation_id: &str) -> String {
    // Преобразуем "AuthController_login" в "auth_controller_login"
    let mut result = String::new();
    let mut chars = operation_id.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    
    result
}