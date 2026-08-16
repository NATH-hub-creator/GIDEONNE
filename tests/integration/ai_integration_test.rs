// tests/integration/ai_integration_test.rs — Tests d'intégration IA
// Nécessite Ollama en cours d'exécution (`ollama serve`)
#[cfg(test)]
mod tests_integration {
    use reqwest::Client;
    use serde_json::json;

    const OLLAMA_URL: &str = "http://localhost:11434";
    const MODELE_TEST: &str = "llama3";

    async fn ollama_disponible() -> bool {
        Client::new().get(OLLAMA_URL).timeout(std::time::Duration::from_secs(3)).send().await
            .map(|r| r.status().is_success()).unwrap_or(false)
    }

    #[tokio::test]
    async fn test_verifier_ollama_disponible() {
        if !ollama_disponible().await { println!("SKIP : Ollama non disponible"); return; }
        let rep = Client::new().get(OLLAMA_URL).send().await.expect("Impossible de joindre Ollama");
        assert!(rep.status().is_success());
    }

    #[tokio::test]
    async fn test_lister_modeles_ollama() {
        if !ollama_disponible().await { println!("SKIP : Ollama non disponible"); return; }
        let rep = Client::new().get(format!("{}/api/tags", OLLAMA_URL)).send().await.expect("Err");
        assert!(rep.status().is_success());
        let corps: serde_json::Value = rep.json().await.expect("JSON invalide");
        assert!(corps.get("models").is_some(), "Champ 'models' manquant");
    }

    #[tokio::test]
    async fn test_envoyer_message_simple() {
        if !ollama_disponible().await { println!("SKIP : Ollama non disponible"); return; }
        let corps = json!({ "model": MODELE_TEST, "messages": [{"role": "user", "content": "R\u00e9ponds juste : 'OK'"}], "stream": false });
        let rep = Client::new().post(format!("{}/api/chat", OLLAMA_URL)).json(&corps)
            .timeout(std::time::Duration::from_secs(120)).send().await;
        match rep {
            Ok(r) if r.status().is_success() => {
                let json: serde_json::Value = r.json().await.expect("JSON invalide");
                let contenu = json["message"]["content"].as_str().unwrap_or("");
                assert!(!contenu.is_empty(), "R\u00e9ponse vide");
                println!("LLM: {}", contenu);
            }
            Ok(r) => println!("Statut inattendu : {}", r.status()),
            Err(e) => println!("Erreur r\u00e9seau : {}", e),
        }
    }

    #[tokio::test]
    async fn test_structure_reponse_ollama() {
        if !ollama_disponible().await { println!("SKIP : Ollama non disponible"); return; }
        let corps = json!({ "model": MODELE_TEST, "messages": [{"role": "user", "content": "1+1=?"}], "stream": false });
        let rep = Client::new().post(format!("{}/api/chat", OLLAMA_URL)).json(&corps)
            .timeout(std::time::Duration::from_secs(60)).send().await;
        if let Ok(r) = rep {
            if r.status().is_success() {
                let json: serde_json::Value = r.json().await.expect("JSON invalide");
                assert!(json.get("model").is_some());
                assert!(json.get("message").is_some());
                assert!(json.get("done").is_some());
                assert!(json["message"].get("content").is_some());
            }
        }
    }
}
