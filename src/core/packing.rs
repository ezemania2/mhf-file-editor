use std::path::Path;
use std::io;
use rsfrontier_core::{pack_buffer as rsfrontier_pack_buffer, pack_folder, unpack_buffer, FolderPackType};

// Re-export PackType
pub use rsfrontier_core::PackType;

/// Décompresse et déchiffre un fichier si nécessaire
pub fn unpack_file(input_path: &Path) -> io::Result<Vec<u8>> {
    let file_buf = std::fs::read(input_path)?;
    let output_path = input_path.to_string_lossy();
    let unpacked_files = unpack_buffer(&output_path, &file_buf, None);
    
    // Pour un fichier unique, on retourne juste le contenu
    if unpacked_files.len() == 1 {
        Ok(unpacked_files[0].1.clone())
    } else {
        Ok(file_buf) // Si pas de décompression/déchiffrement, on retourne le fichier original
    }
}

/// Compresse et chiffre un fichier
pub fn pack_file(input_path: &Path, output_path: &Path, encrypt: bool) -> std::io::Result<()> {
    let file_buf = std::fs::read(input_path)?;
    
    // On commence par compresser avec JPK type 4 (Huffman + LZ)
    let packed_data = rsfrontier_pack_buffer(&file_buf, PackType::Jpk(4));
    
    // Si demandé, on chiffre avec ECD
    let final_data = if encrypt {
        rsfrontier_pack_buffer(&packed_data, PackType::Ecd)
    } else {
        packed_data
    };
    
    // On écrit le résultat
    std::fs::write(output_path, final_data)
}

/// Compresse et chiffre un dossier
pub fn pack_directory(input_path: &Path, output_path: &Path, encrypt: bool) -> std::io::Result<()> {
    // On commence par créer une archive simple
    let packed_data = pack_folder(input_path, FolderPackType::Simple);
    
    // Si demandé, on chiffre avec ECD
    let final_data = if encrypt {
        rsfrontier_pack_buffer(&packed_data, PackType::Ecd)
    } else {
        packed_data
    };
    
    // On écrit le résultat
    std::fs::write(output_path, final_data)
}

/// Ouvre un fichier binaire en essayant d'abord RsFrontier, puis en fallback sur le fichier original si besoin
pub fn open_bin_with_unpack_fallback<T, F>(path: &Path, parse: F) -> Result<T, String>
where
    F: Fn(&[u8]) -> Result<T, io::Error>,
{
    match unpack_file(path) {
        Ok(data) => {
            match parse(&data) {
                Ok(res) => Ok(res),
                Err(_) => {
                    // Fallback: essayer de lire le fichier original
                    match std::fs::read(path).and_then(|buf| parse(&buf)) {
                        Ok(res) => Ok(res),
                        Err(e) => Err(format!("Failed to parse file (packed and direct): {e}")),
                    }
                }
            }
        }
        Err(e) => Err(format!("Failed to open file: {e}")),
    }
}

/// Compresse un fichier avec JPK Type 4 (sans encryption)
pub fn compress_file(input_path: &Path, output_path: &Path) -> std::io::Result<()> {
    let file_buf = std::fs::read(input_path)?;
    
    // Compression JPK Type 4 (Huffman + LZ)
    let compressed_data = rsfrontier_pack_buffer(&file_buf, PackType::Jpk(4));
    
    // Écrire le fichier compressé
    std::fs::write(output_path, compressed_data)
}

/// Chiffre un fichier avec ECD (sans compression)
pub fn encrypt_file(input_path: &Path, output_path: &Path) -> std::io::Result<()> {
    let file_buf = std::fs::read(input_path)?;
    
    // Chiffrement ECD
    let encrypted_data = rsfrontier_pack_buffer(&file_buf, PackType::Ecd);
    
    // Écrire le fichier chiffré
    std::fs::write(output_path, encrypted_data)
}

/// Compresse et chiffre un buffer
pub fn pack_buffer(buf: &[u8], pack_type: PackType) -> Vec<u8> {
    rsfrontier_pack_buffer(buf, pack_type)
} 