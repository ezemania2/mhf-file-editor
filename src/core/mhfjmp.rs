// Logique spécifique au format mhfjmp.bin 

use std::io::{self, Read, Write, Seek, SeekFrom, Cursor};
use std::fs::File;
use std::path::Path;
use encoding_rs::SHIFT_JIS;

use crate::model::mhfjmp::{MenuEntry, Area, StringEntry, AreaEntry};

fn read_u32_le<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}
fn read_u16_le<R: Read>(r: &mut R) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}
fn read_f32_le<R: Read>(r: &mut R) -> io::Result<f32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

fn read_shift_jis_string<R: Read + Seek>(r: &mut R, ptr: u32) -> io::Result<String> {
    let cur = r.seek(SeekFrom::Current(0))?;
    r.seek(SeekFrom::Start(ptr as u64))?;
    let mut bytes = Vec::new();
    loop {
        let mut b = [0u8; 1];
        match r.read_exact(&mut b) {
            Ok(_) => {
                if b[0] == 0 { break; }
                bytes.push(b[0]);
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    let (cow, _, _) = SHIFT_JIS.decode(&bytes);
    let s = cow.to_string();
    r.seek(SeekFrom::Start(cur))?;
    Ok(s)
}

pub fn load_mhfjmp_bin<P: AsRef<Path>>(path: P) -> io::Result<(Vec<MenuEntry>, Vec<Area>, Vec<StringEntry>)> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();

    let menu_entries_ptr = read_u32_le(&mut file)?;
    let areas_ptr = read_u32_le(&mut file)?;
    let num_areas = read_u32_le(&mut file)?;
    let strings_ptr = read_u32_le(&mut file)?;
    let num_strings = read_u32_le(&mut file)?;

    let menu_entry_count = 24;
    let _menu_entry_size = 56;
    let mut menu_entries = Vec::with_capacity(menu_entry_count);
    
    file.seek(SeekFrom::Start(menu_entries_ptr as u64))?;
    
    for _ in 0..menu_entry_count {
        let jump_id = read_u32_le(&mut file)?;
        let unk0c = read_u32_le(&mut file)?;
        let area_id = read_u16_le(&mut file)?;
        let area_id2 = read_u16_le(&mut file)?;
        let area_id3 = read_u16_le(&mut file)?;
        let area_id4 = read_u16_le(&mut file)?;
        let player_pos_x = read_f32_le(&mut file)?;
        let player_pos_y = read_f32_le(&mut file)?;
        let player_pos_z = read_f32_le(&mut file)?;
        let rotation = read_u32_le(&mut file)?;
        let camera_pos_x = read_f32_le(&mut file)?;
        let camera_pos_y = read_f32_le(&mut file)?;
        let camera_pos_z = read_f32_le(&mut file)?;
        let rotation1 = read_u32_le(&mut file)?;
        let title_ptr = read_u32_le(&mut file)?;
        let desc_ptr = read_u32_le(&mut file)?;
        let title = read_shift_jis_string(&mut file, title_ptr).unwrap_or_default();
        let description = read_shift_jis_string(&mut file, desc_ptr).unwrap_or_default();
        menu_entries.push(MenuEntry {
            jump_id, unk0c, area_id, area_id2, area_id3, area_id4,
            player_pos_x, player_pos_y, player_pos_z, rotation,
            camera_pos_x, camera_pos_y, camera_pos_z, rotation1,
            title, description,
        });
    }

    let mut areas = Vec::with_capacity(num_areas as usize);
    file.seek(SeekFrom::Start(areas_ptr as u64))?;
    for i in 0..num_areas {
        let header_offset = areas_ptr as u64 + (i as u64) * 12;
        file.seek(SeekFrom::Start(header_offset))?;
        let p_entry_data = read_u32_le(&mut file)?;
        let len_entry_data = read_u32_le(&mut file)?;
        let p_stage_ids = read_u32_le(&mut file)?;
        let mut entries = Vec::new();
        if p_entry_data != 0 && len_entry_data > 0 {
            let cur = file.seek(SeekFrom::Current(0))?;
            file.seek(SeekFrom::Start(p_entry_data as u64))?;
            for _ in 0..len_entry_data {
                let index = read_u16_le(&mut file)?;
                let flags = read_u16_le(&mut file)?;
                entries.push(AreaEntry { index, flags });
            }
            file.seek(SeekFrom::Start(cur))?;
        }
        let mut stage_ids = Vec::new();
        if p_stage_ids != 0 {
            let cur = file.seek(SeekFrom::Current(0))?;
            file.seek(SeekFrom::Start(p_stage_ids as u64))?;
            loop {
                let id = read_u16_le(&mut file)?;
                if id == 0 { break; }
                stage_ids.push(id);
            }
            file.seek(SeekFrom::Start(cur))?;
        }
        areas.push(Area {
            p_entry_data,
            len_entry_data,
            p_stage_ids,
            entries,
            stage_ids,
        });
    }

    let mut string_entries = Vec::with_capacity(num_strings as usize);
    if strings_ptr != 0 && num_strings > 0 {
        file.seek(SeekFrom::Start(strings_ptr as u64))?;
        let mut string_offsets = Vec::with_capacity(num_strings as usize);
        for _ in 0..num_strings {
            let ptr = read_u32_le(&mut file)?;
            string_offsets.push(ptr);
        }
        for (i, &ptr) in string_offsets.iter().enumerate() {
            if ptr == 0 || u64::from(ptr) >= file_size {
                string_entries.push(StringEntry { id: i as i32, text: String::new() });
                continue;
            }
            let text = read_shift_jis_string(&mut file, ptr).unwrap_or_default();
            string_entries.push(StringEntry { id: i as i32, text });
        }
    }
    Ok((menu_entries, areas, string_entries))
}

fn write_u32_le<W: Write>(w: &mut W, value: u32) -> io::Result<()> {
    w.write_all(&value.to_le_bytes())
}

fn write_u16_le<W: Write>(w: &mut W, value: u16) -> io::Result<()> {
    w.write_all(&value.to_le_bytes())
}

fn write_f32_le<W: Write>(w: &mut W, value: f32) -> io::Result<()> {
    w.write_all(&value.to_le_bytes())
}

pub fn save_mhfjmp_bin<P: AsRef<Path>>(path: P, menu_entries: &[MenuEntry], areas: &[Area], strings: &[StringEntry]) -> io::Result<()> {
    let mut file = File::create(path)?;
    let header_size = 20;
    let menu_entries_size = menu_entries.len() * 56;
    let areas_headers_size = areas.len() * 12;
    let mut area_entries_size = 0;
    let mut stage_ids_size = 0;
    for area in areas {
        area_entries_size += area.entries.len() * 4;
        stage_ids_size += (area.stage_ids.len() + 1) * 2;
    }
    let menu_entries_offset = header_size;
    let menu_strings_offset = menu_entries_offset + menu_entries_size;
    write_u32_le(&mut file, menu_entries_offset as u32)?;
    write_u32_le(&mut file, 0)?;
    write_u32_le(&mut file, areas.len() as u32)?;
    write_u32_le(&mut file, 0)?;
    write_u32_le(&mut file, strings.len() as u32)?;
    file.seek(SeekFrom::Start(menu_entries_offset as u64))?;
    for _ in menu_entries {
        file.write_all(&[0; 56])?;
    }
    let mut title_offsets = Vec::with_capacity(menu_entries.len());
    let mut desc_offsets = Vec::with_capacity(menu_entries.len());
    let mut cur_offset = menu_strings_offset as u32;
    file.seek(SeekFrom::Start(cur_offset as u64))?;
    for entry in menu_entries {
        title_offsets.push(cur_offset);
        let (title_bytes, _, _) = SHIFT_JIS.encode(&entry.title);
        file.write_all(&title_bytes)?;
        file.write_all(&[0])?;
        cur_offset += title_bytes.len() as u32 + 1;
    }
    for entry in menu_entries {
        desc_offsets.push(cur_offset);
        let (desc_bytes, _, _) = SHIFT_JIS.encode(&entry.description);
        file.write_all(&desc_bytes)?;
        file.write_all(&[0])?;
        cur_offset += desc_bytes.len() as u32 + 1;
    }
    let areas_headers_offset = cur_offset;
    let areas_data_offset = areas_headers_offset + areas_headers_size as u32;
    let strings_offset = areas_data_offset + (area_entries_size + stage_ids_size) as u32;
    file.seek(SeekFrom::Start(4))?;
    write_u32_le(&mut file, areas_headers_offset)?;
    file.seek(SeekFrom::Start(12))?;
    write_u32_le(&mut file, strings_offset)?;
    file.seek(SeekFrom::Start(menu_entries_offset as u64))?;
    for (i, entry) in menu_entries.iter().enumerate() {
        write_u32_le(&mut file, entry.jump_id)?;
        write_u32_le(&mut file, entry.unk0c)?;
        write_u16_le(&mut file, entry.area_id)?;
        write_u16_le(&mut file, entry.area_id2)?;
        write_u16_le(&mut file, entry.area_id3)?;
        write_u16_le(&mut file, entry.area_id4)?;
        write_f32_le(&mut file, entry.player_pos_x)?;
        write_f32_le(&mut file, entry.player_pos_y)?;
        write_f32_le(&mut file, entry.player_pos_z)?;
        write_u32_le(&mut file, entry.rotation)?;
        write_f32_le(&mut file, entry.camera_pos_x)?;
        write_f32_le(&mut file, entry.camera_pos_y)?;
        write_f32_le(&mut file, entry.camera_pos_z)?;
        write_u32_le(&mut file, entry.rotation1)?;
        write_u32_le(&mut file, title_offsets[i])?;
        write_u32_le(&mut file, desc_offsets[i])?;
    }
    file.seek(SeekFrom::Start(areas_headers_offset as u64))?;
    let mut current_data_offset = areas_data_offset;
    for area in areas {
        let entry_data_ptr = if !area.entries.is_empty() { current_data_offset as u32 } else { 0 };
        let stage_ids_ptr = if !area.stage_ids.is_empty() { (current_data_offset + (area.entries.len() as u32) * 4) as u32 } else { 0 };
        write_u32_le(&mut file, entry_data_ptr)?;
        write_u32_le(&mut file, area.len_entry_data)?;
        write_u32_le(&mut file, stage_ids_ptr)?;
        if !area.entries.is_empty() {
            let cur_pos = file.seek(SeekFrom::Current(0))?;
            file.seek(SeekFrom::Start(entry_data_ptr as u64))?;
            for entry in &area.entries {
                write_u16_le(&mut file, entry.index)?;
                write_u16_le(&mut file, entry.flags)?;
            }
            file.seek(SeekFrom::Start(cur_pos))?;
        }
        if !area.stage_ids.is_empty() {
            let cur_pos = file.seek(SeekFrom::Current(0))?;
            file.seek(SeekFrom::Start(stage_ids_ptr as u64))?;
            for &id in &area.stage_ids {
                write_u16_le(&mut file, id)?;
            }
            write_u16_le(&mut file, 0)?;
            file.seek(SeekFrom::Start(cur_pos))?;
        }
        current_data_offset += (area.entries.len() as u32) * 4 + ((area.stage_ids.len() as u32) + 1) * 2;
    }
    file.seek(SeekFrom::Start(strings_offset as u64))?;
    for _ in 0..strings.len() {
        write_u32_le(&mut file, 0)?;
    }
    let mut string_offsets = Vec::with_capacity(strings.len());
    let mut cur_str_offset = strings_offset + (strings.len() as u32) * 4;
    for entry in strings {
        string_offsets.push(cur_str_offset);
        file.seek(SeekFrom::Start(cur_str_offset as u64))?;
        let (sjis_bytes, _, _) = SHIFT_JIS.encode(&entry.text);
        file.write_all(&sjis_bytes)?;
        file.write_all(&[0])?;
        cur_str_offset += sjis_bytes.len() as u32 + 1;
    }
    file.seek(SeekFrom::Start(strings_offset as u64))?;
    for &ptr in &string_offsets {
        write_u32_le(&mut file, ptr)?;
    }
    Ok(())
}

pub fn load_mhfjmp_bin_from_buffer(buffer: &[u8]) -> io::Result<(Vec<MenuEntry>, Vec<Area>, Vec<StringEntry>)> {
    let mut cursor = Cursor::new(buffer);
    load_mhfjmp_bin_from_reader_with_log(&mut cursor)
}

pub fn load_mhfjmp_bin_from_reader_with_log<R: Read + Seek>(reader: &mut R) -> io::Result<(Vec<MenuEntry>, Vec<Area>, Vec<StringEntry>)> {
    let file_size = reader.seek(SeekFrom::End(0))?;
    reader.seek(SeekFrom::Start(0))?;

    let menu_entries_ptr = read_u32_le(reader)?;
    let areas_ptr = read_u32_le(reader)?;
    let num_areas = read_u32_le(reader)?;
    let strings_ptr = read_u32_le(reader)?;
    let num_strings = read_u32_le(reader)?;

    eprintln!("[DEBUG] file_size={file_size} menu_entries_ptr={menu_entries_ptr} areas_ptr={areas_ptr} num_areas={num_areas} strings_ptr={strings_ptr} num_strings={num_strings}");

    let menu_entry_count = 24;
    let _menu_entry_size = 56;
    let mut menu_entries = Vec::with_capacity(menu_entry_count);

    if menu_entries_ptr as u64 >= file_size {
        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("menu_entries_ptr out of bounds: {menu_entries_ptr} >= {file_size}")));
    }
    reader.seek(SeekFrom::Start(menu_entries_ptr as u64))?;

    for _ in 0..menu_entry_count {
        let jump_id = read_u32_le(reader)?;
        let unk0c = read_u32_le(reader)?;
        let area_id = read_u16_le(reader)?;
        let area_id2 = read_u16_le(reader)?;
        let area_id3 = read_u16_le(reader)?;
        let area_id4 = read_u16_le(reader)?;
        let player_pos_x = read_f32_le(reader)?;
        let player_pos_y = read_f32_le(reader)?;
        let player_pos_z = read_f32_le(reader)?;
        let rotation = read_u32_le(reader)?;
        let camera_pos_x = read_f32_le(reader)?;
        let camera_pos_y = read_f32_le(reader)?;
        let camera_pos_z = read_f32_le(reader)?;
        let rotation1 = read_u32_le(reader)?;
        let title_ptr = read_u32_le(reader)?;
        let desc_ptr = read_u32_le(reader)?;
        let title = read_shift_jis_string(reader, title_ptr).unwrap_or_default();
        let description = read_shift_jis_string(reader, desc_ptr).unwrap_or_default();
        menu_entries.push(MenuEntry {
            jump_id, unk0c, area_id, area_id2, area_id3, area_id4,
            player_pos_x, player_pos_y, player_pos_z, rotation,
            camera_pos_x, camera_pos_y, camera_pos_z, rotation1,
            title, description,
        });
    }

    let mut areas = Vec::with_capacity(num_areas as usize);
    if areas_ptr as u64 >= file_size {
        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("areas_ptr out of bounds: {areas_ptr} >= {file_size}")));
    }
    reader.seek(SeekFrom::Start(areas_ptr as u64))?;
    for i in 0..num_areas {
        let header_offset = areas_ptr as u64 + (i as u64) * 12;
        if header_offset >= file_size {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("area header out of bounds: {header_offset} >= {file_size}")));
        }
        reader.seek(SeekFrom::Start(header_offset))?;
        let p_entry_data = read_u32_le(reader)?;
        let len_entry_data = read_u32_le(reader)?;
        let p_stage_ids = read_u32_le(reader)?;
        let mut entries = Vec::new();
        if p_entry_data != 0 && len_entry_data > 0 {
            let cur = reader.seek(SeekFrom::Current(0))?;
            if p_entry_data as u64 >= file_size {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("p_entry_data out of bounds: {p_entry_data} >= {file_size}")));
            }
            reader.seek(SeekFrom::Start(p_entry_data as u64))?;
            for _ in 0..len_entry_data {
                let index = read_u16_le(reader)?;
                let flags = read_u16_le(reader)?;
                entries.push(AreaEntry { index, flags });
            }
            reader.seek(SeekFrom::Start(cur))?;
        }
        let mut stage_ids = Vec::new();
        if p_stage_ids != 0 {
            let cur = reader.seek(SeekFrom::Current(0))?;
            if p_stage_ids as u64 >= file_size {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("p_stage_ids out of bounds: {p_stage_ids} >= {file_size}")));
            }
            reader.seek(SeekFrom::Start(p_stage_ids as u64))?;
            loop {
                let id = read_u16_le(reader)?;
                if id == 0 { break; }
                stage_ids.push(id);
            }
            reader.seek(SeekFrom::Start(cur))?;
        }
        areas.push(Area {
            p_entry_data,
            len_entry_data,
            p_stage_ids,
            entries,
            stage_ids,
        });
    }

    let mut string_entries = Vec::with_capacity(num_strings as usize);
    if strings_ptr != 0 && num_strings > 0 {
        if strings_ptr as u64 >= file_size {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("strings_ptr out of bounds: {strings_ptr} >= {file_size}")));
        }
        reader.seek(SeekFrom::Start(strings_ptr as u64))?;
        let mut string_offsets = Vec::with_capacity(num_strings as usize);
        for _ in 0..num_strings {
            let ptr = read_u32_le(reader)?;
            string_offsets.push(ptr);
        }
        for (i, &ptr) in string_offsets.iter().enumerate() {
            if ptr == 0 || u64::from(ptr) >= file_size {
                string_entries.push(StringEntry { id: i as i32, text: String::new() });
                continue;
            }
            let text = read_shift_jis_string(reader, ptr).unwrap_or_default();
            string_entries.push(StringEntry { id: i as i32, text });
        }
    }
    Ok((menu_entries, areas, string_entries))
}

pub fn save_mhfjmp_bin_to_writer<W: Write + Seek>(writer: &mut W, menu_entries: &[MenuEntry], areas: &[Area], strings: &[StringEntry]) -> io::Result<()> {
    let header_size = 20;
    let menu_entries_size = menu_entries.len() * 56;
    let areas_headers_size = areas.len() * 12;
    let mut area_entries_size = 0;
    let mut stage_ids_size = 0;
    for area in areas {
        area_entries_size += area.entries.len() * 4;
        stage_ids_size += (area.stage_ids.len() + 1) * 2;
    }
    let menu_entries_offset = header_size;
    let menu_strings_offset = menu_entries_offset + menu_entries_size;
    write_u32_le(writer, menu_entries_offset as u32)?;
    write_u32_le(writer, 0)?;
    write_u32_le(writer, areas.len() as u32)?;
    write_u32_le(writer, 0)?;
    write_u32_le(writer, strings.len() as u32)?;
    writer.seek(SeekFrom::Start(menu_entries_offset as u64))?;
    for _ in menu_entries {
        writer.write_all(&[0; 56])?;
    }
    let mut title_offsets = Vec::with_capacity(menu_entries.len());
    let mut desc_offsets = Vec::with_capacity(menu_entries.len());
    let mut cur_offset = menu_strings_offset as u32;
    writer.seek(SeekFrom::Start(cur_offset as u64))?;
    for entry in menu_entries {
        title_offsets.push(cur_offset);
        let (title_bytes, _, _) = SHIFT_JIS.encode(&entry.title);
        writer.write_all(&title_bytes)?;
        writer.write_all(&[0])?;
        cur_offset += title_bytes.len() as u32 + 1;
    }
    for entry in menu_entries {
        desc_offsets.push(cur_offset);
        let (desc_bytes, _, _) = SHIFT_JIS.encode(&entry.description);
        writer.write_all(&desc_bytes)?;
        writer.write_all(&[0])?;
        cur_offset += desc_bytes.len() as u32 + 1;
    }
    let areas_headers_offset = cur_offset;
    let areas_data_offset = areas_headers_offset + areas_headers_size as u32;
    let strings_offset = areas_data_offset + (area_entries_size + stage_ids_size) as u32;
    writer.seek(SeekFrom::Start(4))?;
    write_u32_le(writer, areas_headers_offset)?;
    writer.seek(SeekFrom::Start(12))?;
    write_u32_le(writer, strings_offset)?;
    writer.seek(SeekFrom::Start(menu_entries_offset as u64))?;
    for (i, entry) in menu_entries.iter().enumerate() {
        write_u32_le(writer, entry.jump_id)?;
        write_u32_le(writer, entry.unk0c)?;
        write_u16_le(writer, entry.area_id)?;
        write_u16_le(writer, entry.area_id2)?;
        write_u16_le(writer, entry.area_id3)?;
        write_u16_le(writer, entry.area_id4)?;
        write_f32_le(writer, entry.player_pos_x)?;
        write_f32_le(writer, entry.player_pos_y)?;
        write_f32_le(writer, entry.player_pos_z)?;
        write_u32_le(writer, entry.rotation)?;
        write_f32_le(writer, entry.camera_pos_x)?;
        write_f32_le(writer, entry.camera_pos_y)?;
        write_f32_le(writer, entry.camera_pos_z)?;
        write_u32_le(writer, entry.rotation1)?;
        write_u32_le(writer, title_offsets[i])?;
        write_u32_le(writer, desc_offsets[i])?;
    }
    writer.seek(SeekFrom::Start(areas_headers_offset as u64))?;
    let mut current_data_offset = areas_data_offset;
    for area in areas {
        let entry_data_ptr = if !area.entries.is_empty() { current_data_offset as u32 } else { 0 };
        let stage_ids_ptr = if !area.stage_ids.is_empty() { (current_data_offset + (area.entries.len() as u32) * 4) as u32 } else { 0 };
        write_u32_le(writer, entry_data_ptr)?;
        write_u32_le(writer, area.len_entry_data)?;
        write_u32_le(writer, stage_ids_ptr)?;
        if !area.entries.is_empty() {
            let cur_pos = writer.seek(SeekFrom::Current(0))?;
            writer.seek(SeekFrom::Start(entry_data_ptr as u64))?;
            for entry in &area.entries {
                write_u16_le(writer, entry.index)?;
                write_u16_le(writer, entry.flags)?;
            }
            writer.seek(SeekFrom::Start(cur_pos))?;
        }
        if !area.stage_ids.is_empty() {
            let cur_pos = writer.seek(SeekFrom::Current(0))?;
            writer.seek(SeekFrom::Start(stage_ids_ptr as u64))?;
            for &id in &area.stage_ids {
                write_u16_le(writer, id)?;
            }
            write_u16_le(writer, 0)?;
            writer.seek(SeekFrom::Start(cur_pos))?;
        }
        current_data_offset += (area.entries.len() as u32) * 4 + ((area.stage_ids.len() as u32) + 1) * 2;
    }
    writer.seek(SeekFrom::Start(strings_offset as u64))?;
    for _ in 0..strings.len() {
        write_u32_le(writer, 0)?;
    }
    let mut string_offsets = Vec::with_capacity(strings.len());
    let mut cur_str_offset = strings_offset + (strings.len() as u32) * 4;
    for entry in strings {
        string_offsets.push(cur_str_offset);
        writer.seek(SeekFrom::Start(cur_str_offset as u64))?;
        let (sjis_bytes, _, _) = SHIFT_JIS.encode(&entry.text);
        writer.write_all(&sjis_bytes)?;
        writer.write_all(&[0])?;
        cur_str_offset += sjis_bytes.len() as u32 + 1;
    }
    writer.seek(SeekFrom::Start(strings_offset as u64))?;
    for &ptr in &string_offsets {
        write_u32_le(writer, ptr)?;
    }
    Ok(())
} 