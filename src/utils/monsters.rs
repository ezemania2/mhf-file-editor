// Monster id -> name, in the order of the game's monster ids (the Hunting Road table and the quests use the id).
// The list comes from BlazeMH/Blaze-Hunting-Road-Editor (core/constants.py MONSTERS), credited in the README.

pub const MONSTER_NAMES: [&str; 177] = [
    "None", "Rathian", "Fatalis", "Kelbi", "Mosswine", "Bullfango", "Yian Kut Ku", "Lao Shan Lung", "Cephadrome",
    "Felyne 1", "Veggie Elder", "Rathalos", "Aptonoth", "Genprey", "Diablos", "Khezu", "Velociprey", "Gravios",
    "Felyne 2", "Vespoid", "Gypceros", "Plesioth", "Basarios", "Melynx", "Hornetaur", "Apceros", "Monoblos",
    "Velocidrome", "Gendrome", "Rocks 0", "Ioprey", "Iodrome", "Pugis", "Kirin", "Cephalos", "Giaprey",
    "Crimson Fatalis", "Pink Rathian", "Blue Yian Kut Ku", "Purple Gypceros", "Yian Garuga", "Silver Rathalos",
    "Gold Rathian", "Black Diablos", "White Monoblos", "Red Khezu", "Green Plesioth", "Black Gravios",
    "Daimyo Hermitaur", "Azure Rathalos", "Ashen Lao Shan Lung", "Blangonga", "Congalala", "Rajang", "Kushala Daora",
    "Shen Gaoren", "Great Thunderbug", "Shakalaka", "Yama Tsukami 1", "Chameleos", "Rusted Kushala Daora", "Blango",
    "Conga", "Remobra", "Lunastra", "Teostra", "Hermitaur", "Shogun Ceanataur", "Bulldrome", "Anteka", "Popo",
    "White Fatalis", "Yama Tsukami 2", "Ceanataur", "Hypnocatrice", "Lavasioth", "Tigrex", "Akantor", "Bright Hypnoc",
    "Lavasioth Subspecies", "Espinas", "Orange Espinas", "White Hypnoc", "Akura Vashimu", "Akura Jebia",
    "Berukyurosu", "Cactus 01", "Gorge Objects", "Gorge Rocks", "Pariapuria", "White Espinas", "Kamu Orugaron",
    "Nono Orugaron", "Raviente", "Dyuragaua", "Doragyurosu", "Gurenzeburu", "Burukku", "Erupe", "Rukodiora",
    "Unknown", "Gogomoa", "Kokomoa", "Taikun Zamuza", "Abiorugu", "Kuarusepusu", "Odibatorasu", "Disufiroa",
    "Rebidiora", "Anorupatisu", "Hyujikiki", "Midogaron", "Giaorugu", "Mi Ru", "Farunokku", "Pokaradon", "Shantien",
    "Pokara", "Dummy", "Goruganosu", "Aruganosu", "Baruragaru", "Zerureusu", "Gougarf", "Uruki", "Forokururu",
    "Meraginasu", "Diorekkusu", "Garuba Daora", "Inagami", "Varusaburosu", "Poborubarumu", "Duremudira", "UNK 0",
    "Felyne", "Blue NPC", "UNK 1", "Cactus Varusa", "Veggie Elders", "Gureadomosu", "Harudomerugu", "Toridcless",
    "Gasurabazura", "Kusubami", "Yama Kurai", "Dure 2nd District", "Zinogre", "Deviljho", "Brachydios",
    "Berserk Laviente", "Toa Tesukatora", "Barioth", "Uragaan", "Stygian Zinogre", "Guanzorumu", "Starving Deviljho",
    "UNK", "Egyurasu", "Voljang", "Nargacuga", "Keoaruboru", "Zenaserisu", "Gore Magala", "Blinking Nargacuga",
    "Shagaru Magala", "Amatsu", "Elzelion", "Musou Dure", "Rocks 1", "Seregios", "Bogabadorumu",
    "Unknown Blue Barrel", "Musou Bogabadorumu", "Costumed Uruki", "Musou Zerureusu", "PSO2 Rappy", "King Shakalaka",
];

/// Name of a monster id, or "?" when the id is not in the list.
pub fn monster_name(id: u16) -> &'static str {
    MONSTER_NAMES.get(id as usize).copied().unwrap_or("?")
}
