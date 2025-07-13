use remnawave::RemnawaveClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Пример использования клиента
    let base_url = std::env::var("REMNAWAVE_URL")
        .unwrap_or_else(|_| "https://localhost".to_string());
    
    let token = std::env::var("REMNAWAVE_TOKEN")
        .unwrap_or_else(|_| "your-token-here".to_string());

    println!("Создание клиента RemnaWave...");
    println!("Base URL: {}", base_url);
    
    let client = RemnawaveClient::new(base_url, token)?;
    
    println!("Клиент успешно создан!");
    
    // Демонстрируем текущий API (builder pattern)
    println!("\n=== Текущий API (builder pattern) ===");
    println!("client.auth().get_status() - возвращает builder");
    println!("client.auth().login() - возвращает builder");
    
    // Показываем примеры вызовов через builder
    let _auth_controller = client.auth();
    let _status_builder = _auth_controller.get_status();
    
    // Демонстрируем желаемый API (простые функции)
    println!("\n=== Желаемый API (простые функции) ===");
    println!("// client.auth().login(\"username\", \"password\").await - прямой вызов с параметрами");
    println!("// client.auth().get_status().await - простой вызов без параметров");
    println!("// client.users().create(\"test\", \"test@test.com\").await - создание пользователя");
    println!("// client.users().get_all().await - получение всех пользователей");
    
    println!("\nДля полноценной реализации нужно:");
    println!("1. Проанализировать каждую операцию в OpenAPI");
    println!("2. Извлечь типы параметров из схем");
    println!("3. Создать функции-обертки с правильными типами");
    println!("4. Автоматически формировать body из параметров");
    
    println!("\nТекущий статус:");
    println!("✅ Группировка по контроллерам");
    println!("✅ Автоматическая генерация из OpenAPI");
    println!("⚠️  Простые параметры (требует анализа типов)");
    
    println!("\nТест завершен успешно!");
    Ok(())
}
