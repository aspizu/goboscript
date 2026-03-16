use base64::{
    engine::general_purpose::STANDARD as BASE64,
    Engine as _,
};

/// Convert a bitmap image (png, webp, jpg, etc.) to an SVG wrapper
/// that embeds the image as a base64 data URI. This preserves full
/// image quality by avoiding Scratch's internal bitmap re-encoding.
pub fn costume_to_svg(data: &[u8], extension: &str) -> Vec<u8> {
    let mime = match extension {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "bmp" => "image/bmp",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "image/png",
    };
    let b64 = BASE64.encode(data);
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><image href="data:{mime};base64,{b64}"/></svg>"#
    ).into_bytes()
}
