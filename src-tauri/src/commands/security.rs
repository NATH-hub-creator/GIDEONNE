// commands/security.rs — Chiffrement AES-256-GCM
use serde::{Deserialize, Serialize};
use tauri::command;
use aes_gcm::{aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Key, Nonce};

#[derive(Debug, Serialize, Deserialize)]
pub struct DonneesChiffrees {
    pub succes: bool,
    pub message: String,
    pub donnees: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DonneesDechiffrees {
    pub succes: bool,
    pub message: String,
    pub donnees: Option<String>,
}

/// Chiffre des données avec AES-256-GCM
#[command]
pub async fn chiffrer_donnees(donnees: String, cle_hex: String) -> DonneesChiffrees {
    match chiffrer_interne(donnees.as_bytes(), &cle_hex) {
        Ok(chiffre) => DonneesChiffrees { succes: true, message: "Chiffrement réussi.".to_string(), donnees: Some(chiffre) },
        Err(e) => DonneesChiffrees { succes: false, message: format!("Erreur : {}", e), donnees: None },
    }
}

/// Déchiffre des données AES-256-GCM
#[command]
pub async fn dechiffrer_donnees(donnees_b64: String, cle_hex: String) -> DonneesDechiffrees {
    match dechiffrer_interne(&donnees_b64, &cle_hex) {
        Ok(clair) => DonneesDechiffrees { succes: true, message: "Déchiffrement réussi.".to_string(), donnees: Some(clair) },
        Err(e) => DonneesDechiffrees { succes: false, message: format!("Erreur : {}", e), donnees: None },
    }
}

fn chiffrer_interne(donnees: &[u8], cle_hex: &str) -> anyhow::Result<String> {
    let cle_bytes = hex::decode(cle_hex).map_err(|_| anyhow::anyhow!("Clé hex invalide"))?;
    let cle = Key::<Aes256Gcm>::from_slice(&cle_bytes);
    let chiffre_algo = Aes256Gcm::new(cle);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = chiffre_algo.encrypt(&nonce, donnees).map_err(|_| anyhow::anyhow!("Echec chiffrement"))?;
    let mut resultat = nonce.to_vec();
    resultat.extend_from_slice(&ciphertext);
    Ok(base64_encode(&resultat))
}

fn dechiffrer_interne(donnees_b64: &str, cle_hex: &str) -> anyhow::Result<String> {
    let donnees = base64_decode(donnees_b64).map_err(|_| anyhow::anyhow!("Base64 invalide"))?;
    let cle_bytes = hex::decode(cle_hex).map_err(|_| anyhow::anyhow!("Clé hex invalide"))?;
    let cle = Key::<Aes256Gcm>::from_slice(&cle_bytes);
    let chiffre_algo = Aes256Gcm::new(cle);
    if donnees.len() < 12 { return Err(anyhow::anyhow!("Données trop courtes")); }
    let (nonce_bytes, ciphertext) = donnees.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let clair = chiffre_algo.decrypt(nonce, ciphertext).map_err(|_| anyhow::anyhow!("Echec déchiffrement"))?;
    String::from_utf8(clair).map_err(|e| anyhow::anyhow!("UTF-8 invalide : {}", e))
}

fn base64_encode(donnees: &[u8]) -> String {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut r = Vec::new();
    for c in donnees.chunks(3) {
        let b0 = c[0] as usize; let b1 = if c.len()>1{c[1] as usize}else{0}; let b2 = if c.len()>2{c[2] as usize}else{0};
        r.push(alphabet[b0>>2]); r.push(alphabet[((b0&3)<<4)|(b1>>4)]);
        if c.len()>1{r.push(alphabet[((b1&15)<<2)|(b2>>6)])}else{r.push(b'=')};
        if c.len()>2{r.push(alphabet[b2&63])}else{r.push(b'=')};
    }
    String::from_utf8(r).unwrap_or_default()
}

fn base64_decode(s: &str) -> Result<Vec<u8>, ()> {
    let s: Vec<u8> = s.bytes().filter(|&b| b!=b'\n'&&b!=b'\r').collect();
    let mut out = Vec::new();
    let lk = |c: u8| -> Result<u8,()> { match c { b'A'..=b'Z'=>Ok(c-b'A'), b'a'..=b'z'=>Ok(c-b'a'+26), b'0'..=b'9'=>Ok(c-b'0'+52), b'+'=>Ok(62), b'/'=>Ok(63), b'='=>Ok(0), _=>Err(()) } };
    for c in s.chunks(4) { if c.len()<4{break;} let (a,b,cc,d)=(lk(c[0])?,lk(c[1])?,lk(c[2])?,lk(c[3])?);
        out.push((a<<2)|(b>>4)); if c[2]!=b'='{out.push((b<<4)|(cc>>2));} if c[3]!=b'='{out.push((cc<<6)|d);} }
    Ok(out)
}
