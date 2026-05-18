use std::{fmt, net::SocketAddr, sync::Arc};

use axum::{
    extract::rejection::JsonRejection,
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use tokio::net::TcpListener;
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
struct MorningMetrics {
    current_rhr: u8,
    baseline_rhr: u8,
    hrv: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskLoad {
    Low,
    Medium,
    High,
}

impl TaskLoad {
    fn score(self) -> u8 {
        match self {
            TaskLoad::Low => 1,
            TaskLoad::Medium => 2,
            TaskLoad::High => 3,
        }
    }

    fn from_str(value: &str) -> Option<Self> {
        match value.trim().to_lowercase().as_str() {
            "low" => Some(TaskLoad::Low),
            "medium" => Some(TaskLoad::Medium),
            "high" => Some(TaskLoad::High),
            _ => None,
        }
    }
}

impl<'de> Deserialize<'de> for TaskLoad {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TaskLoad::from_str(&s).ok_or_else(|| de::Error::custom(format!("unknown task load: {}", s)))
    }
}

impl Serialize for TaskLoad {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            TaskLoad::Low => "Low",
            TaskLoad::Medium => "Medium",
            TaskLoad::High => "High",
        };
        serializer.serialize_str(s)
    }
}

impl fmt::Display for TaskLoad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            TaskLoad::Low => "Low",
            TaskLoad::Medium => "Medium",
            TaskLoad::High => "High",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Task {
    id: String,
    title: String,
    cognitive_load: TaskLoad,
    physical_load: TaskLoad,
    is_essential: bool,
}

#[derive(Debug, Deserialize)]
struct PlanRequest {
    morning_metrics: MorningMetrics,
    tasks: Vec<Task>,
}

#[derive(Debug, Serialize)]
struct DailyPlanResponse {
    status: String,
    do_recommendations: Vec<String>,
    dont_recommendations: Vec<String>,
    optimized_tasks: Vec<Task>,
}

#[derive(Debug, Serialize)]
struct ApiErrorResponse {
    error: String,
}

#[derive(Debug, Error)]
enum ApiError {
    #[error("invalid JSON payload")]
    InvalidPayload(#[from] JsonRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::BAD_REQUEST;
        let error_message = "Invalid JSON payload or missing required fields".to_string();

        let body = Json(ApiErrorResponse {
            error: error_message,
        });

        (status, body).into_response()
    }
}

trait PlanEngine: Send + Sync {
    fn build_daily_plan(&self, metrics: &MorningMetrics, tasks: Vec<Task>) -> DailyPlanResponse;
}

struct DefaultPlanEngine;

impl DefaultPlanEngine {
    fn is_fatigued(metrics: &MorningMetrics) -> bool {
        let threshold = metrics.baseline_rhr as f32 * 1.10;
        metrics.current_rhr as f32 > threshold
    }

    fn is_optimal(metrics: &MorningMetrics) -> bool {
        metrics.current_rhr as u8 <= metrics.baseline_rhr
    }
}

impl PlanEngine for DefaultPlanEngine {
    fn build_daily_plan(&self, metrics: &MorningMetrics, tasks: Vec<Task>) -> DailyPlanResponse {
        if Self::is_fatigued(metrics) {
            let filtered: Vec<Task> = tasks
                .into_iter()
                .filter(|task| task.is_essential || !(task.cognitive_load == TaskLoad::High || task.physical_load == TaskLoad::High))
                .collect();

            let mut essential_high: Vec<Task> = filtered
                .iter()
                .cloned()
                .filter(|task| task.is_essential && (task.cognitive_load == TaskLoad::High || task.physical_load == TaskLoad::High))
                .collect();

            let mut low_load: Vec<Task> = filtered
                .iter()
                .cloned()
                .filter(|task| task.cognitive_load == TaskLoad::Low && task.physical_load == TaskLoad::Low)
                .collect();

            let mut remaining: Vec<Task> = filtered
                .into_iter()
                .filter(|task| {
                    !(task.is_essential && (task.cognitive_load == TaskLoad::High || task.physical_load == TaskLoad::High))
                        && !(task.cognitive_load == TaskLoad::Low && task.physical_load == TaskLoad::Low)
                })
                .collect();

            essential_high.append(&mut low_load);
            essential_high.append(&mut remaining);

            DailyPlanResponse {
                status: "Fatigued".to_string(),
                do_recommendations: vec![
                    "Prioritize recovery and avoid high-strain tasks.".to_string(),
                    "Focus on essential work, short breaks, and hydration.".to_string(),
                ],
                dont_recommendations: vec![
                    "Skip heavy workouts today.".to_string(),
                    "Limit caffeine and avoid late-night deep work.".to_string(),
                ],
                optimized_tasks: essential_high,
            }
        } else if Self::is_optimal(metrics) {
            let mut sorted_tasks = tasks;
            sorted_tasks.sort_by(|a, b| b.cognitive_load.score().cmp(&a.cognitive_load.score()));

            DailyPlanResponse {
                status: "Prime".to_string(),
                do_recommendations: vec![
                    "Tackle the hardest problems early.".to_string(),
                    "Use your recovery window for focused deep work.".to_string(),
                ],
                dont_recommendations: vec![
                    "Don't procrastinate on your top priority tasks.".to_string(),
                ],
                optimized_tasks: sorted_tasks,
            }
        } else {
            let mut sorted_tasks = tasks;
            sorted_tasks.sort_by(|a, b| {
                b.is_essential
                    .cmp(&a.is_essential)
                    .then_with(|| b.cognitive_load.score().cmp(&a.cognitive_load.score()))
            });

            DailyPlanResponse {
                status: "Normal".to_string(),
                do_recommendations: vec![
                    "Keep a balanced workload and listen to your energy levels.".to_string(),
                    "Start with essential tasks and maintain steady pacing.".to_string(),
                ],
                dont_recommendations: vec![
                    "Avoid sudden spikes in physical or mental strain.".to_string(),
                ],
                optimized_tasks: sorted_tasks,
            }
        }
    }
}

async fn plan_day(
    Extension(engine): Extension<Arc<dyn PlanEngine>>,
    payload: Result<Json<PlanRequest>, JsonRejection>,
) -> Result<Json<DailyPlanResponse>, ApiError> {
    let Json(request) = payload.map_err(ApiError::InvalidPayload)?;
    let response = engine.build_daily_plan(&request.morning_metrics, request.tasks);
    Ok(Json(response))
}

#[tokio::main]
async fn main() {
    let engine: Arc<dyn PlanEngine> = Arc::new(DefaultPlanEngine);

    async fn serve_index() -> impl IntoResponse {
        match tokio::fs::read_to_string("static/index.html").await {
            Ok(s) => (StatusCode::OK, [("content-type", "text/html; charset=utf-8")], s).into_response(),
            Err(_) => (StatusCode::NOT_FOUND, "index.html not found").into_response(),
        }
    }

    async fn serve_js() -> impl IntoResponse {
        match tokio::fs::read_to_string("static/app.js").await {
            Ok(s) => (StatusCode::OK, [("content-type", "application/javascript; charset=utf-8")], s).into_response(),
            Err(_) => (StatusCode::NOT_FOUND, "app.js not found").into_response(),
        }
    }

    async fn serve_css() -> impl IntoResponse {
        match tokio::fs::read_to_string("static/styles.css").await {
            Ok(s) => (StatusCode::OK, [("content-type", "text/css; charset=utf-8")], s).into_response(),
            Err(_) => (StatusCode::NOT_FOUND, "styles.css not found").into_response(),
        }
    }

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/app.js", get(serve_js))
        .route("/styles.css", get(serve_css))
        .route("/api/v1/plan-day", post(plan_day))
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .layer(Extension(engine));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind address: {}", e));

    axum::serve(listener, app)
        .await
        .expect("server failed");
}
