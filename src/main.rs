use std::{fmt, net::SocketAddr, sync::Arc};

use axum::{
    extract::rejection::JsonRejection,
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use hello_world::{Contract, ContractClient, Task as ContractTask, TaskLoad as ContractTaskLoad};
use serde::{de, Deserialize, Deserializer, Serialize};
use soroban_sdk::{Env, String as SorobanString, Vec as SorobanVec};
use tokio::net::TcpListener;
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
struct MorningMetrics {
    current_rhr: u32,
    baseline_rhr: u32,
    hrv: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskLoad {
    Low,
    Medium,
    High,
}

impl TaskLoad {
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

#[derive(Debug, Clone, Deserialize)]
struct TaskRequest {
    id: String,
    title: String,
    cognitive_load: TaskLoad,
    physical_load: TaskLoad,
    is_essential: bool,
}

#[derive(Debug, Serialize)]
struct TaskResponse {
    id: String,
    title: String,
    cognitive_load: String,
    physical_load: String,
    is_essential: bool,
}

#[derive(Debug, Deserialize)]
struct PlanRequest {
    morning_metrics: MorningMetrics,
    tasks: Vec<TaskRequest>,
}

#[derive(Debug, Serialize)]
struct DailyPlanResponse {
    status: String,
    do_recommendations: Vec<String>,
    dont_recommendations: Vec<String>,
    optimized_tasks: Vec<TaskResponse>,
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
    fn build_daily_plan(&self, metrics: &MorningMetrics, tasks: Vec<TaskRequest>) -> DailyPlanResponse;
}

struct SorobanPlanEngine;

impl SorobanPlanEngine {
    fn convert_task_load(load: TaskLoad) -> ContractTaskLoad {
        match load {
            TaskLoad::Low => ContractTaskLoad::Low,
            TaskLoad::Medium => ContractTaskLoad::Medium,
            TaskLoad::High => ContractTaskLoad::High,
        }
    }

    fn soroban_string_to_std(value: &SorobanString) -> String {
        value.to_string()
    }

    fn contract_task_to_response(task: &ContractTask) -> TaskResponse {
        TaskResponse {
            id: Self::soroban_string_to_std(&task.id),
            title: Self::soroban_string_to_std(&task.title),
            cognitive_load: match task.cognitive_load {
                ContractTaskLoad::Low => "Low".to_string(),
                ContractTaskLoad::Medium => "Medium".to_string(),
                ContractTaskLoad::High => "High".to_string(),
            },
            physical_load: match task.physical_load {
                ContractTaskLoad::Low => "Low".to_string(),
                ContractTaskLoad::Medium => "Medium".to_string(),
                ContractTaskLoad::High => "High".to_string(),
            },
            is_essential: task.is_essential,
        }
    }

    fn build_contract_tasks(env: &Env, tasks: Vec<TaskRequest>) -> SorobanVec<ContractTask> {
        let mut contract_tasks = SorobanVec::new(env);
        for task in tasks {
            contract_tasks.push_back(ContractTask {
                id: SorobanString::from_str(env, &task.id),
                title: SorobanString::from_str(env, &task.title),
                cognitive_load: Self::convert_task_load(task.cognitive_load),
                physical_load: Self::convert_task_load(task.physical_load),
                is_essential: task.is_essential,
            });
        }
        contract_tasks
    }
}

impl PlanEngine for SorobanPlanEngine {
    fn build_daily_plan(&self, metrics: &MorningMetrics, tasks: Vec<TaskRequest>) -> DailyPlanResponse {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let contract_tasks = Self::build_contract_tasks(&env, tasks);

        let (status, do_recommendations, dont_recommendations, optimized_tasks) =
            client.plan_daily(&metrics.current_rhr, &metrics.baseline_rhr, &contract_tasks);

        DailyPlanResponse {
            status: Self::soroban_string_to_std(&status),
            do_recommendations: do_recommendations
                .iter()
                .map(|s| Self::soroban_string_to_std(&s))
                .collect(),
            dont_recommendations: dont_recommendations
                .iter()
                .map(|s| Self::soroban_string_to_std(&s))
                .collect(),
            optimized_tasks: optimized_tasks
                .iter()
                .map(|task| Self::contract_task_to_response(&task))
                .collect(),
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
    let engine: Arc<dyn PlanEngine> = Arc::new(SorobanPlanEngine);

    async fn serve_index() -> impl IntoResponse {
        match tokio::fs::read_to_string("frontend/index.html").await {
            Ok(s) => (StatusCode::OK, [("content-type", "text/html; charset=utf-8")], s).into_response(),
            Err(_) => (StatusCode::NOT_FOUND, "index.html not found").into_response(),
        }
    }

    async fn serve_js() -> impl IntoResponse {
        match tokio::fs::read_to_string("frontend/app.js").await {
            Ok(s) => (StatusCode::OK, [("content-type", "application/javascript; charset=utf-8")], s).into_response(),
            Err(_) => (StatusCode::NOT_FOUND, "app.js not found").into_response(),
        }
    }

    async fn serve_css() -> impl IntoResponse {
        match tokio::fs::read_to_string("frontend/styles.css").await {
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
