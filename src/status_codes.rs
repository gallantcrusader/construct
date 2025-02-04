pub enum HttpStatus {
    // 1xx: Informational
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,

    // 2xx: Success
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    ImUsed,

    // 3xx: Redirection
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // 4xx: Client Error
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // 5xx: Server Error
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl HttpStatus {
    pub fn as_bytes(&self) -> Vec<u8> {
        let status_line = match self {
            // 1xx: Informational
            HttpStatus::Continue => "100 Continue",
            HttpStatus::SwitchingProtocols => "101 Switching Protocols",
            HttpStatus::Processing => "102 Processing",
            HttpStatus::EarlyHints => "103 Early Hints",

            // 2xx: Success
            HttpStatus::Ok => "200 OK",
            HttpStatus::Created => "201 Created",
            HttpStatus::Accepted => "202 Accepted",
            HttpStatus::NonAuthoritativeInformation => "203 Non-Authoritative Information",
            HttpStatus::NoContent => "204 No Content",
            HttpStatus::ResetContent => "205 Reset Content",
            HttpStatus::PartialContent => "206 Partial Content",
            HttpStatus::MultiStatus => "207 Multi-Status",
            HttpStatus::AlreadyReported => "208 Already Reported",
            HttpStatus::ImUsed => "226 IM Used",

            // 3xx: Redirection
            HttpStatus::MultipleChoices => "300 Multiple Choices",
            HttpStatus::MovedPermanently => "301 Moved Permanently",
            HttpStatus::Found => "302 Found",
            HttpStatus::SeeOther => "303 See Other",
            HttpStatus::NotModified => "304 Not Modified",
            HttpStatus::UseProxy => "305 Use Proxy",
            HttpStatus::TemporaryRedirect => "307 Temporary Redirect",
            HttpStatus::PermanentRedirect => "308 Permanent Redirect",

            // 4xx: Client Error
            HttpStatus::BadRequest => "400 Bad Request",
            HttpStatus::Unauthorized => "401 Unauthorized",
            HttpStatus::PaymentRequired => "402 Payment Required",
            HttpStatus::Forbidden => "403 Forbidden",
            HttpStatus::NotFound => "404 Not Found",
            HttpStatus::MethodNotAllowed => "405 Method Not Allowed",
            HttpStatus::NotAcceptable => "406 Not Acceptable",
            HttpStatus::ProxyAuthenticationRequired => "407 Proxy Authentication Required",
            HttpStatus::RequestTimeout => "408 Request Timeout",
            HttpStatus::Conflict => "409 Conflict",
            HttpStatus::Gone => "410 Gone",
            HttpStatus::LengthRequired => "411 Length Required",
            HttpStatus::PreconditionFailed => "412 Precondition Failed",
            HttpStatus::PayloadTooLarge => "413 Payload Too Large",
            HttpStatus::UriTooLong => "414 URI Too Long",
            HttpStatus::UnsupportedMediaType => "415 Unsupported Media Type",
            HttpStatus::RangeNotSatisfiable => "416 Range Not Satisfiable",
            HttpStatus::ExpectationFailed => "417 Expectation Failed",
            HttpStatus::ImATeapot => "418 I'm a Teapot",
            HttpStatus::MisdirectedRequest => "421 Misdirected Request",
            HttpStatus::UnprocessableEntity => "422 Unprocessable Entity",
            HttpStatus::Locked => "423 Locked",
            HttpStatus::FailedDependency => "424 Failed Dependency",
            HttpStatus::TooEarly => "425 Too Early",
            HttpStatus::UpgradeRequired => "426 Upgrade Required",
            HttpStatus::PreconditionRequired => "428 Precondition Required",
            HttpStatus::TooManyRequests => "429 Too Many Requests",
            HttpStatus::RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large",
            HttpStatus::UnavailableForLegalReasons => "451 Unavailable For Legal Reasons",

            // 5xx: Server Error
            HttpStatus::InternalServerError => "500 Internal Server Error",
            HttpStatus::NotImplemented => "501 Not Implemented",
            HttpStatus::BadGateway => "502 Bad Gateway",
            HttpStatus::ServiceUnavailable => "503 Service Unavailable",
            HttpStatus::GatewayTimeout => "504 Gateway Timeout",
            HttpStatus::HttpVersionNotSupported => "505 HTTP Version Not Supported",
            HttpStatus::VariantAlsoNegotiates => "506 Variant Also Negotiates",
            HttpStatus::InsufficientStorage => "507 Insufficient Storage",
            HttpStatus::LoopDetected => "508 Loop Detected",
            HttpStatus::NotExtended => "510 Not Extended",
            HttpStatus::NetworkAuthenticationRequired => "511 Network Authentication Required",
        };
        let mut bytes = status_line.as_bytes().to_vec();
        bytes.extend_from_slice(b"\r\n");
        bytes
    }
}
