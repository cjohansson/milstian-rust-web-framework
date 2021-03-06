//! # Handling MIME types

/// Determine MIME based on filename.
// # Example
/// ```rust
/// use milstian_internet_framework::mime;
/// let mime_type = mime::from_filename("random.aac");
/// assert_eq!("audio/aac", mime_type);
/// ```
// @see https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Complete_list_of_MIME_types
pub fn from_filename(filename: &str) -> String {
    let mut mime = "application/octet-stream";
    let parts: Vec<&str> = filename.rsplitn(2, ".").collect();
    if !parts.is_empty() {
        if let Some(extension) = parts.get(0) {
            let extension: String = extension.to_lowercase();
            mime = match extension.as_ref() {
                "aac" => "audio/aac",
                "abw" => "application/x-abiword",
                "avi" => "video/x-msvideo",
                "azw" => "application/vnd.amazon.ebook",
                "bmp" => "image/bmp",
                "bz" => "application/x-bzip",
                "bz2" => "application/x-bzip2",
                "csh" => "application/x-csh",
                "css" => "text/css",
                "csv" => "text/csv",
                "doc" => "application/msword",
                "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                "eot" => "application/vnd.ms-fontobject",
                "epub" => "application/epub+zip",
                "es" => "application/ecmascript",
                "gif" => "image/gif",
                "html" => "text/html",
                "htm" => "text/html",
                "ico" => "image/x-icon",
                "ics" => "text/calendar",
                "jar" => "application/java-archive",
                "jpeg" => "image/jpeg",
                "jpg" => "image/jpg",
                "js" => "application/javascript",
                "json" => "application/json",
                "mid" => "audo/midi",
                "midi" => "audo/midi",
                "mpeg" => "video/mpeg",
                "mpkg" => "application/vnd.apple.installer+xml",
                "odp" => "application/vnd.oasis.opendocument.presentation",
                "ods" => "application/vnd.oasis.opendocument.spreadsheet",
                "odt" => "application/vnd.oasis.opendocument.text",
                "oga" => "audio/ogg",
                "ogv" => "video/ogg",
                "ogx" => "application/ogg",
                "otf" => "font/otf",
                "png" => "image/png",
                "pdf" => "application/pdf",
                "ppt" => "application/vnd.ms-powerpoint",
                "pptx" => {
                    "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                }
                "rar" => "application/x-rar-compressed",
                "rtf" => "application/rtf",
                "sh" => "application/x-sh",
                "svg" => "image/svg+xml",
                "swf" => "application/x-shockwave-flash",
                "tar" => "application/x-tar",
                "tif" => "image/tiff",
                "tiff" => "image/tiff",
                "ts" => "application/typescript",
                "ttf" => "font/ttf",
                "txt" => "text/plain",
                "vsd" => "application/vnd.visio",
                "wav" => "audio/wav",
                "weba" => "audio/webm",
                "webm" => "video/webm",
                "webp" => "image/webp",
                "woff" => "font/woff",
                "woff2" => "font/woff2",
                "xhtml" => "application/xhtml+xml",
                "xls" => "application/vnd.ms-excel",
                "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                "xml" => "application/xml",
                "xul" => "application/vnd.mozilla.xul+xml",
                "zip" => "application/zip",
                "3gp" => "video/3gpp",
                "3g2" => "video/3gpp2",
                "7z" => "application/x-7z-compressed",
                _ => "application/octet-stream",
            };
        }
    }
    mime.to_string()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_from_filename()
    {
        assert_eq!(
            from_filename("random.webp"),
            "image/webp"
        );
        assert_eq!(
            from_filename("random.random"),
            "application/octet-stream"
        );
    }
}
