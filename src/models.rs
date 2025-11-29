#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pokemon {
    pub id: u8,
    pub nom: String,
    pub types: Vec<String>,
    pub total: u32,
    pub hp: u32,
    pub att: u32,
    pub def: u32,
    pub vitesse: u32,
    pub id_evolution: u8,
}
