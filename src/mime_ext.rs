use mime::Mime;

pub trait MimeExt {
    fn is_same_essence(&self, other: &Mime) -> bool;
    fn normalized_essence_str(&self) -> &str;
}

impl MimeExt for Mime {
    fn is_same_essence(&self, other: &Mime) -> bool {
        self.normalized_essence_str() == other.normalized_essence_str()
    }

    fn normalized_essence_str(&self) -> &str {
        let essence = self.essence_str();
        if essence.ends_with("+json") {
            return "application/json";
        } else if essence == "image/svg+xml" {
            return "image/svg+xml";
        } else if essence.ends_with("+xml") {
            return "application/xml";
        } else {
            match essence {
                "application/ecmascript" | "application/javascript" | "application/x-ecmascript" |
                "application/x-javascript" | "text/ecmascript" | "text/javascript" |
                "text/javascript1.0" | "text/javascript1.1" | "text/javascript1.2" |
                "text/javascript1.3" | "text/javascript1.4" | "text/javascript1.5" |
                "text/jscript" | "text/livescript" | "text/x-ecmascript" | "text/x-javascript" =>
                    "application/javascript",
                "text/json" => "application/json",
                "text/xml" => "application/xml",
                _ => essence,
            }
        }
    }
}