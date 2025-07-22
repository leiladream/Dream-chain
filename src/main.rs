use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::prelude::*;
use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    index: u64,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
    event: String,
}

fn calculate_hash(
    index: u64,
    timestamp: &str,
    data: &str,
    previous_hash: &str,
    nonce: u64,
    event: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_string());
    hasher.update(timestamp);
    hasher.update(data);
    hasher.update(previous_hash);
    hasher.update(nonce.to_string());
    hasher.update(event);
    let result = hasher.finalize();
    hex::encode(result)
}

fn generate_event() -> String {
    let events = vec![
        "Une étoile est née 🌟",
        "Un secret s'efface dans le vide 🌫️",
        "Une comète traverse la mémoire ☄️",
        "Un rêve s’enlace à la réalité ✨",
        "Une vibration d’amour se propage 💖",
        "Un mystère éclot dans le silence 🌙",
        "Le temps s’est plié sur lui-même ⏳",
        "Un souvenir perdu s'est allumé 🔥",
    ];
    let mut rng = rand::thread_rng();
    events[rng.gen_range(0..events.len())].to_string()
}

fn create_block(index: u64, data: String, previous_hash: String) -> Block {
    let timestamp = Utc::now().to_rfc3339();
    let event = generate_event();
    let mut nonce = 0;
    let mut hash = calculate_hash(index, &timestamp, &data, &previous_hash, nonce, &event);

    while !hash.starts_with("0000") {
        nonce += 1;
        hash = calculate_hash(index, &timestamp, &data, &previous_hash, nonce, &event);
    }

    Block {
        index,
        timestamp,
        data,
        previous_hash,
        hash,
        nonce,
        event,
    }
}

fn save_chain_to_file(chain: &Vec<Block>) {
    let file = File::create("dream_chain.json").expect("Erreur de création de fichier");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, chain).expect("Erreur d’écriture JSON");
}

fn load_chain_from_file() -> Vec<Block> {
    if let Ok(file) = File::open("dream_chain.json") {
        let reader = BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(chain) => chain,
            Err(_) => {
                println!("⚠️ Fichier JSON corrompu, nouvelle chaîne créée.");
                Vec::new()
            }
        }
    } else {
        Vec::new()
    }
}

fn main() {
    println!("🌙 Bienvenue dans Dream Chain, là où chaque bloc est un rêve codé...");

    let mut chain = load_chain_from_file();
    if chain.is_empty() {
        println!("✨ Création du bloc Genesis...");
        let genesis = create_block(0, "Premier rêve : naissance de la chaîne.".to_string(), "0".to_string());
        chain.push(genesis);
    }

    loop {
        println!("\nMenu :");
        println!("1. Ajouter un nouveau bloc");
        println!("2. Afficher la Dream Chain");
        println!("3. Quitter");

        print!("Choix > ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");

        match choix.trim() {
            "1" => {
                print!("📝 Entre ton message de rêve > ");
                io::stdout().flush().unwrap();
                let mut data = String::new();
                io::stdin().read_line(&mut data).expect("Erreur de lecture");

                let last_block = chain.last().unwrap();
                let new_block = create_block(last_block.index + 1, data.trim().to_string(), last_block.hash.clone());
                println!("✨ Nouveau bloc forgé avec succès !");
                println!("{:#?}", new_block);
                chain.push(new_block);
                save_chain_to_file(&chain);
            }
            "2" => {
                println!("\n🌌 Chaîne actuelle :");
                for block in &chain {
                    println!("{:#?}", block);
                }
            }
            "3" => {
                println!("🌙 À bientôt dans tes rêves, voyageuse onirique...");
                break;
            }
            _ => println!("Option invalide 💔"),
        }
    }
}
