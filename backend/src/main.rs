use axum::{routing::{get, post}, Router, Json, extract::State, http::{Method, header}};
use serde::{Serialize, Deserialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    db: SqlitePool,
    api_key: String,
}

#[derive(Deserialize)]
struct MensagemEntrada {
    usuario_id: String,
    texto: String,
}

#[derive(Serialize)]
struct MensagemSaida {
    resposta: String,
}

#[tokio::main]
async fn main() {
    // Busca a chave injetada pelo Systemd ou Ambiente
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_else(|_| {
        println!("⚠️  CHAVE NÃO ENCONTRADA! Verifique o teologia.service");
        "FALTA_CHAVE".to_string()
    });

    let db_connection = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:teologia.db?mode=rwc")
        .await
        .expect("Erro ao abrir banco");

    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS historico (id INTEGER PRIMARY KEY, usuario_id TEXT, papel TEXT, conteudo TEXT, data_envio DATETIME DEFAULT CURRENT_TIMESTAMP)")
        .execute(&db_connection).await;

    // CORS: Essencial para o seu futuro site em Vue.js
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([header::CONTENT_TYPE]);

    let shared_state = Arc::new(AppState { db: db_connection, api_key });

    let app = Router::new()
        .route("/", get(|| async { "Motor TeologIA Online" }))
        .route("/api/chat", post(receber_chat))
        .layer(cors)
        .with_state(shared_state);

    println!("🚀 TeologIA subindo na porta 3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn receber_chat(State(state): State<Arc<AppState>>, Json(payload): Json<MensagemEntrada>) -> Json<MensagemSaida> {
    println!("📩 Recebido de {}: {}", payload.usuario_id, payload.texto);

    let client = reqwest::Client::new();
    // Usando gemini-2.5-flash que é o modelo atual estável
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", state.api_key);

    let corpo = serde_json::json!({
        "contents": [{
            "parts": [{ "text": format!("Persona: Mestre Teólogo Sábio.\n\nUsuário: {}", payload.texto) }]
        }]
    });

    let res = client.post(url).json(&corpo).send().await;

    let resposta_ia = match res {
        Ok(response) => {
            let status = response.status();
            let texto_puro = response.text().await.unwrap_or_default();
            
            if !status.is_success() {
                println!("❌ Erro Google API: {} - {}", status, texto_puro);
                "Amado, não consegui meditar agora. Tente novamente.".to_string()
            } else {
                let json: serde_json::Value = serde_json::from_str(&texto_puro).unwrap_or_default();
                json["candidates"][0]["content"]["parts"][0]["text"]
                    .as_str()
                    .unwrap_or("Resposta vazia do Mestre.")
                    .to_string()
            }
        },
        Err(e) => format!("Erro de rede: {}", e),
    };

    // Salva histórico
    let _ = sqlx::query("INSERT INTO historico (usuario_id, papel, conteudo) VALUES (?, 'user', ?)")
        .bind(&payload.usuario_id).bind(&payload.texto).execute(&state.db).await;
    let _ = sqlx::query("INSERT INTO historico (usuario_id, papel, conteudo) VALUES (?, 'assistant', ?)")
        .bind(&payload.usuario_id).bind(&resposta_ia).execute(&state.db).await;

    Json(MensagemSaida { resposta: resposta_ia })
}