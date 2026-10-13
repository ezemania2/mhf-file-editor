use std::path::Path;
use std::io;
use rsfrontier_core::{pack_buffer as rsfrontier_pack_buffer, pack_folder, FolderPackType};

// Re-export PackType
pub use rsfrontier_core::PackType;

/// Hex SHA-256 of `data`. Quoted in every "refused file" message so a bug report can be matched
/// to the exact file.
pub fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(data).iter().map(|b| format!("{:02x}", b)).collect()
}

const ECD_MAGIC: u32 = 0x1A64_6365; // "ecd", rsfrontier-core ecd::is_buf_ecd
const JPK_MAGIC: u32 = 0x1A52_4B4A; // "JKR", rsfrontier-core jpk::is_buf_jpk
const WRAPPER_HEADER_LEN: usize = 16;
/// Upper bound for a JPK `out_size`; the biggest known file (mhfdat.bin) is ~29 MB. Stops a corrupt
/// header from making the decoder allocate gigabytes.
const MAX_UNPACKED_SIZE: usize = 512 * 1024 * 1024;
/// ECD -> JPK -> (ECD ...) never nests deeper than this in real files.
const MAX_WRAPPER_LAYERS: usize = 4;

fn le_u32(buf: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(buf.get(at..at.checked_add(4)?)?.try_into().ok()?))
}

fn le_u16(buf: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(buf.get(at..at.checked_add(2)?)?.try_into().ok()?))
}

/// Strips the ECD / JPK wrappers (and nothing else) so the editor always works on the clear file.
///
/// The editor used to hand every opened file to `rsfrontier_core::unpack_buffer`, which since
/// rsfrontier `88dea92` also probes for NSO / link / momo / simple / MHA archives. Those probes are
/// heuristics with no magic number; a clear `mhfjmp.bin` can satisfy `is_buf_nso_pac`, after which
/// `decode_nso_pac` slices past the end of the buffer and panics (issue #1). A clear file is now
/// returned untouched, and a wrapper whose header lies about its size is refused with the offset,
/// the sizes and the SHA-256 of the file on disk instead of reaching the decoder.
pub fn open_clear_buffer(raw: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut current: Option<Vec<u8>> = None;
    for layer in 1..=MAX_WRAPPER_LAYERS {
        let buf: &[u8] = current.as_deref().unwrap_or(&raw);
        let step = match le_u32(buf, 0) {
            Some(ECD_MAGIC) => decrypt_ecd_checked(buf),
            Some(JPK_MAGIC) => decode_jpk_checked(buf),
            _ => return Ok(current.unwrap_or(raw)),
        };
        match step {
            Ok(next) => current = Some(next),
            Err(detail) => return Err(refused(&detail, layer, &raw)),
        }
    }
    Err(refused(
        &format!("more than {} nested ECD/JPK wrappers", MAX_WRAPPER_LAYERS),
        MAX_WRAPPER_LAYERS,
        &raw,
    ))
}

fn refused(detail: &str, layer: usize, original: &[u8]) -> String {
    format!(
        "refusing file: {} (wrapper layer {}; file on disk: {} bytes, SHA-256 {})",
        detail, layer, original.len(), sha256_hex(original)
    )
}

fn short_buffer(what: &str, offset: usize, need: u64, have: usize) -> String {
    format!("{} at offset 0x{:X} needs {} bytes but only {} are left", what, offset, need, have)
}

fn decrypt_ecd_checked(buf: &[u8]) -> Result<Vec<u8>, String> {
    let len = buf.len();
    if len < WRAPPER_HEADER_LEN {
        return Err(short_buffer("ECD header", 0, WRAPPER_HEADER_LEN as u64, len));
    }
    // rsfrontier's key table has 6 rows; a larger index panics inside decrypt_ecd.
    let index = le_u16(buf, 4).unwrap_or(u16::MAX);
    if index > 5 {
        return Err(format!("ECD key index {} at offset 0x4 is outside 0..=5", index));
    }
    let declared = le_u32(buf, 8).unwrap_or(0) as u64;
    if WRAPPER_HEADER_LEN as u64 + declared > len as u64 {
        return Err(short_buffer("ECD payload (size field at 0x8)", WRAPPER_HEADER_LEN, declared, len - WRAPPER_HEADER_LEN));
    }
    guarded("ECD decrypt", || rsfrontier_core::ecd::decrypt_ecd(buf))
}

fn decode_jpk_checked(buf: &[u8]) -> Result<Vec<u8>, String> {
    let len = buf.len();
    if len < WRAPPER_HEADER_LEN {
        return Err(short_buffer("JPK header", 0, WRAPPER_HEADER_LEN as u64, len));
    }
    // rsfrontier JpkType: 0 raw, 2 huffman, 3 lz, 4 huffman+lz
    let comp_type = le_u16(buf, 6).unwrap_or(u16::MAX);
    if !matches!(comp_type, 0 | 2 | 3 | 4) {
        return Err(format!("JPK compression type {} at offset 0x6 is unknown", comp_type));
    }
    let start = le_u32(buf, 8).unwrap_or(u32::MAX) as usize;
    if start < WRAPPER_HEADER_LEN || start > len {
        return Err(short_buffer("JPK data (start offset field at 0x8)", start, 1, len.saturating_sub(start)));
    }
    let out_size = le_u32(buf, 12).unwrap_or(u32::MAX) as usize;
    if out_size > MAX_UNPACKED_SIZE {
        return Err(format!("JPK output size {} at offset 0xC exceeds the {} byte limit", out_size, MAX_UNPACKED_SIZE));
    }
    if comp_type == 0 && start as u64 + out_size as u64 > len as u64 {
        return Err(short_buffer("raw JPK payload", start, out_size as u64, len - start));
    }
    guarded("JPK decode", || rsfrontier_core::jpk::decode_jpk(buf))
}

/// Runs an rsfrontier decoder and turns a panic inside it into an error that quotes the panic text.
/// The header checks above cannot see a truncated or corrupt JPK *body* (e.g. `decode.rs:22
/// read_u8().unwrap()` on a cut LZ stream), so this is the backstop. It needs `panic = "unwind"` in
/// the release profile (Cargo.toml); with `abort` a panic would still kill the process.
fn guarded(what: &str, f: impl FnOnce() -> Vec<u8>) -> Result<Vec<u8>, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|payload| {
        let msg = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&str>().copied())
            .unwrap_or("unknown panic");
        format!("{} failed inside rsfrontier-core: {}", what, msg)
    })
}

/// Reads `input_path` and returns the clear file. See [`open_clear_buffer`].
pub fn unpack_file(input_path: &Path) -> io::Result<Vec<u8>> {
    let file_buf = std::fs::read(input_path)?;
    open_clear_buffer(file_buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
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
