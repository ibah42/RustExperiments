
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::sync::RwLock;

pub async fn test_channel()
{
    // Создаем канал с буфером на 32 сообщения
    let (tx, mut rx) = mpsc::channel(32);

    // Запускаем первую асинхронную задачу
    tokio::spawn(async move {
        for i in 0..10 {
            // Отправляем сообщение в канал
            if let Err(_) = tx.send(i).await {
                println!("Receiver dropped");
                return;
            }
            else{
                println!( "{}", "Receiver sent".to_string() + &i.to_string() );
            }
        }
    });

    // Запускаем вторую асинхронную задачу
    tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            // Обрабатываем полученное сообщение
            println!("Received: {}", value.to_string());
        }
    });

    // Ждем завершения задач
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await
}



pub async fn test_mutex()
{
    let data = Arc::new(Mutex::new(0));

    let data_clone = Arc::clone(&data);
    tokio::spawn(async move {
        let mut num = data_clone.lock().await;
        *num += 1;
        println!("Incremented: {}", *num);
    });

    let data_clone = Arc::clone(&data);
    tokio::spawn(async move {
        let mut num = data_clone.lock().await;
        *num += 2;
        println!("Incremented: {}", *num);
    });

    // Ждем завершения задач
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}


pub async fn test_events()
{
    let notify = Arc::new(Notify::new());

    let notify_clone = Arc::clone(&notify);
    tokio::spawn(async move {
        // Ждем уведомления
        notify_clone.notified().await;
        println!("Задача 1: Получено уведомление!");
    });

    let notify_clone = Arc::clone(&notify);
    tokio::spawn(async move {
        // Ждем некоторое время, затем отправляем уведомление
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        notify_clone.notify_one();
        println!("Задача 2: Уведомление отправлено!");
    });

    // Ждем завершения задач
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}





pub async fn test_locks()
{
    let data = Arc::new(RwLock::new(0));

    let data_clone = Arc::clone(&data);
    tokio::spawn(async move {
        let mut num = data_clone.write().await;
        *num += 1;
        println!("Записано: {}", *num);
    });

    let data_clone = Arc::clone(&data);
    tokio::spawn(async move {
        let num = data_clone.read().await;
        println!("Прочитано: {}", *num);
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}