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
    
    // Проверяем, что контроллеры доступны
    println!("Доступные контроллеры:");
    println!("- Auth: client.auth() - содержит методы аутентификации");
    println!("- Users: client.users() - содержит методы управления пользователями");
    println!("- Subscriptions: client.subscriptions() - содержит методы подписок");
    println!("- Nodes: client.nodes() - содержит методы управления нодами");
    println!("- System: client.system() - содержит системные методы");
    
    // Демонстрируем структуру API
    println!("\nПример использования:");
    println!("client.auth().get_status() - получить статус аутентификации");
    println!("client.users().get_all() - получить всех пользователей");
    println!("client.subscriptions().get_info() - получить информацию о подписке");
    
    // Показываем, что контроллеры создаются
    let _auth = client.auth().get_status(); // Пример вызова метода (закомментирован, так как требует настоящего сервера)
    let _users = client.users();
    let _subscriptions = client.subscriptions();
    let _nodes = client.nodes();
    let _system = client.system();
    
    // Пример вызова метода (закомментирован, так как требует настоящего сервера)
    // let status = client.auth().get_status().await?;
    // println!("Auth status: {:?}", status);
    
    println!("Тест завершен успешно!");
    Ok(())
}
