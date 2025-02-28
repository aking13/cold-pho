use actix_web::{web, HttpResponse, Responder};
use actix_web::http::header;
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, Arc};
use rand::random;
use log::{info, debug, warn};

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    title: String,
}

struct AppState {
    tasks: Mutex<Vec<Task>>,
    next_id: Mutex<usize>,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTask {
    title: String,
}

async fn get_tasks(data: web::Data<Arc<AppState>>) -> impl Responder {
    debug!("Fetching all tasks");
    let tasks = data.tasks.lock().unwrap();
    info!("Retrieved {} tasks", tasks.len());
    HttpResponse::Ok().json(tasks.clone())
}

async fn create_task(task: web::Json<CreateTask>, data: web::Data<Arc<AppState>>) -> impl Responder {
    info!("Creating new task with title: {}", task.title);
    let mut tasks = data.tasks.lock().unwrap();
    let mut next_id = data.next_id.lock().unwrap();
    let new_task = Task {
        id: *next_id,
        title: task.title.clone(),
    };
    *next_id += 1;
    tasks.push(new_task.clone());
    info!("Successfully created task with id: {}", new_task.id);
    HttpResponse::Ok().json(new_task)
}

async fn update_task(path: web::Path<usize>, task: web::Json<UpdateTask>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let id = path.into_inner();
    debug!("Attempting to update task with id: {}", id);
    let mut tasks = data.tasks.lock().unwrap();
    for t in tasks.iter_mut() {
        if t.id == id {
            info!("Updating task {} title from '{}' to '{}'", id, t.title, task.title);
            t.title = task.title.clone();
            return HttpResponse::Ok().json(t.clone());
        }
    }
    warn!("Attempted to update non-existent task with id: {}", id);
    HttpResponse::NotFound().body("Task not found")
}

async fn delete_task(path: web::Path<usize>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let id = path.into_inner();
    debug!("Attempting to delete task with id: {}", id);
    let mut tasks = data.tasks.lock().unwrap();
    let initial_len = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() == initial_len {
        warn!("Attempted to delete non-existent task with id: {}", id);
        return HttpResponse::NotFound().body("Task not found");
    }
    info!("Successfully deleted task with id: {}", id);
    HttpResponse::Ok().body("Task deleted")
}

// Root path handler
async fn welcome() -> impl Responder {
    let random_number: i32 = random();
    info!("Welcome endpoint called, generated random number: {}", random_number);
    HttpResponse::Ok().body(format!("I'm up and running! My favorite number is {}", random_number))
}

#[shuttle_runtime::main]
async fn shuttle_main() -> shuttle_actix_web::ShuttleActixWeb<impl Fn(&mut web::ServiceConfig) + Send + Clone + 'static> {
    info!("Initializing application...");
    let state = Arc::new(AppState {
        tasks: Mutex::new(Vec::new()),
        next_id: Mutex::new(1),
    });
    info!("Application state initialized");

    let app = move |cfg: &mut web::ServiceConfig| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
            
        debug!("Configuring CORS and routes");
        cfg.service(
            web::scope("")
                .wrap(cors)
                .app_data(web::Data::new(state.clone()))
                .route("/", web::get().to(welcome))
                .route("/tasks", web::get().to(get_tasks))
                .route("/tasks", web::post().to(create_task))
                .route("/tasks/{id}", web::put().to(update_task))
                .route("/tasks/{id}", web::delete().to(delete_task))
        );
        info!("Routes configured successfully");
    };

    info!("Server initialization complete");
    Ok(app.into())
}

#[cfg(test)]
mod test;