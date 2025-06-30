use base64::{self, engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_ctoken(
    token_type: &str,
    ticket_collection_t: u64,
    time_offset: i64,
    stay_time: u64,
) -> String {
    let mut rng = rand::rng();
    
    let touch_event: u16;
    let visibility_change: u16 = 2;
    let page_unload: u16;
    let timer: u16;
    let time_difference: u16;
    let scroll_x: u16 = 0;
    let scroll_y: u16 = 0;
    let inner_width: u16 = 255;
    let inner_height: u16 = 255;
    let outer_width: u16 = 255;
    let outer_height: u16 = 255;
    let screen_x: u16 = 0;
    let screen_y: u16 = 0;
    let screen_width: u16 = 255;
    let screen_height: u16 = rng.random_range(1000..=3000);
    let screen_avail_width: u16 = rng.random_range(1..=100);

    if token_type == "createV2" {
        touch_event = 255;
        page_unload = 25;
        
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        time_difference = ((current_time + time_offset - ticket_collection_t as i64) as u16)
            .min(65535);
        timer = (time_difference as u64 + stay_time).min(65535) as u16;
    } else {
        touch_event = rng.random_range(3..=10);
        page_unload = 2;
        time_difference = 0;
        timer = stay_time.min(65535) as u16;
    }

    let mut buffer = [0u8; 16];
    
    buffer[0] = (touch_event.min(255) & 0xFF) as u8;
    buffer[1] = (scroll_x.min(255) & 0xFF) as u8;
    buffer[2] = (visibility_change.min(255) & 0xFF) as u8;
    buffer[3] = (scroll_y.min(255) & 0xFF) as u8;
    buffer[4] = (inner_width.min(255) & 0xFF) as u8;
    buffer[5] = (page_unload.min(255) & 0xFF) as u8;
    buffer[6] = (inner_height.min(255) & 0xFF) as u8;
    buffer[7] = (outer_width.min(255) & 0xFF) as u8;
    
    let timer_val = timer.min(65535);
    buffer[8] = ((timer_val >> 8) & 0xFF) as u8;
    buffer[9] = (timer_val & 0xFF) as u8;
    
    let time_diff_val = time_difference.min(65535);
    buffer[10] = ((time_diff_val >> 8) & 0xFF) as u8;
    buffer[11] = (time_diff_val & 0xFF) as u8;
    
    buffer[12] = (outer_height.min(255) & 0xFF) as u8;
    buffer[13] = (screen_x.min(255) & 0xFF) as u8;
    buffer[14] = (screen_y.min(255) & 0xFF) as u8;
    buffer[15] = (screen_width.min(255) & 0xFF) as u8;

    for i in 0..16 {
        if i == 9 || i == 11 {
            continue;
        }
        if ![0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 13, 14, 15].contains(&i) {
            let condition_value = if (4 & screen_height) != 0 {
                scroll_y
            } else {
                screen_avail_width
            };
            buffer[i] = (condition_value & 0xFF) as u8;
        }
    }

    to_binary(&buffer)
}

fn to_binary(buffer: &[u8]) -> String {
    let mut uint16_data = Vec::new();
    for &byte in buffer {
        uint16_data.push(byte as u16);
    }
    
    let mut uint8_data = Vec::new();
    for val in uint16_data {
        uint8_data.push((val & 0xFF) as u8);
        uint8_data.push(((val >> 8) & 0xFF) as u8);
    }
    
    BASE64_STANDARD.encode(&uint8_data)
}