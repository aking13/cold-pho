use actix_web::{test, web, App};
use std::sync::{Arc, Mutex};
use serde_json::json;

// Import your structs and functions
use super::*;

#[actix_web::test]
async fn test_get_tasks_empty() {
    // Setup app state with empty tasks
    let state = Arc::new(AppState {
        tasks: Mutex::new(Vec::new()),
        next_id: Mutex::new(1),
    });

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/tasks", web::get().to(get_tasks))
    ).await;

    // Create request
    let req = test::TestRequest::get().uri("/tasks").to_request();

    // Execute request and verify response
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Check response body
    let body = test::read_body(resp).await;
    let tasks: Vec<Task> = serde_json::from_slice(&body).unwrap();
    assert_eq!(tasks.len(), 0);
}

#[actix_web::test]
async fn test_create_task() {
    // Setup app state
    let state = Arc::new(AppState {
        tasks: Mutex::new(Vec::new()),
        next_id: Mutex::new(1),
    });

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/tasks", web::post().to(create_task))
    ).await;

    // Create request with JSON body
    let req = test::TestRequest::post()
        .uri("/tasks")
        .set_json(&json!({"title": "Test Task"}))
        .to_request();

    // Execute request and verify response
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Check response body
    let body = test::read_body(resp).await;
    let task: Task = serde_json::from_slice(&body).unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.title, "Test Task");
    
    // Verify state was updated
    let tasks = state.tasks.lock().unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].title, "Test Task");
}

#[actix_web::test]
async fn test_get_tasks_with_data() {
    // Setup app state with pre-populated tasks
    let state = Arc::new(AppState {
        tasks: Mutex::new(vec![
            Task { id: 1, title: "Task 1".to_string() },
            Task { id: 2, title: "Task 2".to_string() },
        ]),
        next_id: Mutex::new(3),
    });

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/tasks", web::get().to(get_tasks))
    ).await;

    // Create request
    let req = test::TestRequest::get().uri("/tasks").to_request();

    // Execute request and verify response
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Check response body
    let body = test::read_body(resp).await;
    let tasks: Vec<Task> = serde_json::from_slice(&body).unwrap();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].title, "Task 1");
    assert_eq!(tasks[1].id, 2);
    assert_eq!(tasks[1].title, "Task 2");
}

#[actix_web::test]
async fn test_update_task() {
    // Setup app state with pre-populated task
    let state = Arc::new(AppState {
        tasks: Mutex::new(vec![
            Task { id: 1, title: "Task 1".to_string() },
        ]),
        next_id: Mutex::new(2),
    });

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/tasks/{id}", web::put().to(update_task))
    ).await;

    // Create request with JSON body
    let req = test::TestRequest::put()
        .uri("/tasks/1")
        .set_json(&json!({"title": "Updated Task"}))
        .to_request();

    // Execute request and verify response
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Check response body
    let body = test::read_body(resp).await;
    let task: Task = serde_json::from_slice(&body).unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.title, "Updated Task");
    
    // Verify state was updated
    let tasks = state.tasks.lock().unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[0].title, "Updated Task");
}

#[actix_web::test]
async fn test_update_task_not_found() {
    // Setup app state with pre-populated task
    let state = Arc::new(AppState {
        tasks: Mutex::new(vec![
            Task { id: 1, title: "Task 1".to_string() },
        ]),
        next_id: Mutex::new(2),
    });

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/tasks/{id}", web::put().to(update_task))
    ).await;

    // Create request with JSON body for a non-existent task
    let req = test::TestRequest::put()
        .uri("/tasks/999")
        .set_json(&json!({"title": "Updated Task"}))
        .to_request();

    // Execute request and verify response
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
    
    // Check response body
    let body = test::read_body(resp).await;
    assert_eq!(body, "Task not found");
}

#[actix_web::test]
async fn test_delete_task() {
    // Setup app state with pre-populated task
    let state = Arc::new(AppState {
        tasks: Mutex::new(vec![
            Task { id: 1, title: "Task 1".to_string() },
        ]),
        next_id: Mutex::new(2),
    });

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/tasks/{id}", web::delete().to(delete_task))
    ).await;

    // Create request to delete task
    let req = test::TestRequest::delete()
        .uri("/tasks/1")
        .to_request();

    // Execute request and verify response
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Check response body
    let body = test::read_body(resp).await;
    assert_eq!(body, "Task deleted");
    
    // Verify state was updated
    let tasks = state.tasks.lock().unwrap();
    assert_eq!(tasks.len(), 0);
}

#[actix_web::test]
async fn test_delete_task_not_found() {
    // Setup app state with pre-populated task
    let state = Arc::new(AppState {
        tasks: Mutex::new(vec![
            Task { id: 1, title: "Task 1".to_string() },
        ]),
        next_id: Mutex::new(2),
    });

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/tasks/{id}", web::delete().to(delete_task))
    ).await;

    // Create request to delete a non-existent task
    let req = test::TestRequest::delete()
        .uri("/tasks/999")
        .to_request();

    // Execute request and verify response
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
    
    // Check response body
    let body = test::read_body(resp).await;
    assert_eq!(body, "Task not found");
}

#[actix_web::test]
async fn test_welcome() {
    // Create test app
    let app = test::init_service(
        App::new()
            .route("/", web::get().to(welcome))
    ).await;

    // Create request
    let req = test::TestRequest::get().uri("/").to_request();

    // Execute request and verify response
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Check response body contains expected string
    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("I'm up and running!"));
}