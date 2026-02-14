use crate::handlers::handle_join;

pub async fn analysis(lines: Vec<String>) {
    for line in &lines {
        let mut split = line.split('|');
        let line_type = split.next().unwrap_or_default();
        match line_type {
            "j" => handle_join(split),
            _ => "".to_string(),
        };
    }
}
