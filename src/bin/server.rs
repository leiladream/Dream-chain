use dream_chain::blockchain::Blockchain;
use tiny_http::{Server, Response, Method, Header};
use std::io::Read;
use serde_json::json;
use std::fs;

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    let mut blockchain = Blockchain::new();

    println!("🚀 Serveur HTTP lancé sur http://localhost:8000");

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().clone();

        match (method, url.as_str()) {
            // Route pour ajouter un bloc
            (Method::Post, "/add_block") => {
                let mut body = String::new();
                request.as_reader().read_to_string(&mut body).unwrap();

                let json_data: serde_json::Value = serde_json::from_str(&body).unwrap();
                let event = json_data["event"].as_str().unwrap_or("Inconnu");
                let data = json_data["data"].as_str().unwrap_or("");

                let block = blockchain.add_block(event.to_string(), data.to_string());
                println!("Nouveau bloc ajouté : {:?}", block);

                let response_json = json!({ "status": "OK", "block": block });
                let response = Response::from_string(response_json.to_string())
                    .with_status_code(200);
                request.respond(response).unwrap();
            }

            // Route pour obtenir la blockchain complète
            (Method::Get, "/chain") => {
                let response_json = json!(blockchain.chain);
                let response = Response::from_string(response_json.to_string())
                    .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                    .with_status_code(200);
                request.respond(response).unwrap();
            }

            // Route pour la page HTML ("/")
            (Method::Get, "/") => {
                let content = fs::read_to_string("src/web/index.html")
                    .unwrap_or_else(|_| "<h1>Erreur : Fichier index.html introuvable</h1>".to_string());
                let response = Response::from_string(content)
                    .with_header(Header::from_bytes("Content-Type", "text/html").unwrap());
                request.respond(response).unwrap();
            }

            // Tout le reste -> 404
            _ => {
                let response = Response::from_string("404 Not Found")
                    .with_status_code(404);
                request.respond(response).unwrap();
            }
        }
    }
}
