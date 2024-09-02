use std::{collections::VecDeque, fmt::format};

#[derive(Clone, PartialEq)]
pub struct Card {
    pub name: String,
    pub image: String,
}

pub fn create_deck() -> VecDeque<Card> {
    let cards = [
        ("El Gallo", "gallo"),
        ("El Diablo", "diablo"),
        ("La Dama", "dama"),
        ("El Catrin", "catrin"),
        ("El Paraguas", "paraguas"),
        ("La Sirena", "sirena"),
        ("La Escalera", "escalera"),
        ("La Botella", "botella"),
        ("El Barril", "barril"),
        ("El Arbol", "arbol"),
        ("El Melon", "melon"),
        ("El Valiente", "valiente"),
        ("El Gorrito", "gorrito"),
        ("La Muerte", "muerte"),
        ("La Pera", "pera"),
        ("La Bandera", "bandera"),
        ("El Bandolon", "bandolon"),
        ("El Violoncello", "violoncello"),
        ("La Garza", "garza"),
        ("El Pajaro", "pajaro"),
        ("La Mano", "mano"),
        ("La Bota", "bota"),
        ("La Luna", "luna"),
        ("El Cotorro", "cotorro"),
        ("El Boracho", "boracho"),
        ("El Negrito", "negrito"),
        ("El Corazon", "corazon"),
        ("La Sandia", "sandia"),
        ("El Tambor", "tambor"),
        ("El Camaron", "camaron"),
        ("Las Jaras", "jaras"),
        ("El Musico", "musico"),
        ("La Araña", "araña"),
        ("El Soldado", "soldado"),
        ("La Estrella", "estrella"),
        ("El Cazo", "cazo"),
        ("El Apache", "apache"),
        ("El Nopal", "nopal"),
        ("El Alacran", "alacran"),
        ("La Rosa", "rosa"),
        ("La Calavera", "calavera"),
        ("La Campana", "campana"),
        ("El Cantarito", "cantarito"),
        ("El Venado", "venado"),
        ("El Sol", "sol"),
        ("La Corona", "corona"),
        ("La Chalupa", "chalupa"),
        ("El Venado", "venado"),
        ("El Pino", "pino"),
        ("El Pescado", "pescado"),
        ("La Palma", "palma"),
        ("La Maceta", "maceta"),
        ("El Arpa", "arpa"),
        ("La Rana", "rana"),
    ];

    let deck: VecDeque<Card> = cards.iter()
        .map(|&(name, id)| {
            Card {
                name: name.to_string(),
                // create image URL string
                image: format!("/imgs/{}.jpg", id),
            }
        })
        .collect();

    return deck

}