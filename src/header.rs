#[derive(Eq, Hash, PartialEq)]
pub enum CommonHeader {
    // Authentication
    WwwAuthenticate,
    Authorization,
    ProxyAuthenticate,
    ProxyAuthorization,

    // Caching
    Age,
    CacheControl,
    ClearSiteData,
    Expires,
    Pragma,
    Warning,

    // Client Hints
    AcceptCH,

    // Network Client Hints
    SaveData,

    // Conditionals
    LastModified,
    ETag,
    IfMatch,
    IfNoneMatch,
    IfModifiedSince,
    IfUnmodifiedSince,
    Vary,
    DeltaBase,

    // Connection Management
    Connection,
    KeepAlive,

    // Content Negotiation
    Accept,
    AcceptEncoding,
    AcceptLanguage,
    AIM,
    IM,

    // Controls
    Expect,
    MaxForwards,

    // Cookies
    Cookie,
    SetCookie,

    // CORS
    AccessControlAllowOrigin,
    AccessControlAllowCredentials,
    AccessControlAllowHeaders,
    AccessControlExposeHeaders,
    AccessControlMaxAge,
    AccessControlRequestHeaders,
    AccessControlRequestMethod,
    TimingAllowOrigin,

    // Downloads
    ContentDisposition,

    // Message Body Information
    ContentLength,
    ContentType,
    ContentEncoding,
    ContentLanguage,
    ContentLocation,

    // Proxies
    Forwarded,
    XForwardedFor,
    XForwardedHost,
    XForwardedProto,
    Via,

    // Redirects
    Location,

    // Request Context
    From,
    Host,
    Referer,
    ReferrerPolicy,
    UserAgent,

    // Response Context
    Allow,
    Server,

    // Range Requests
    AcceptRanges,
    Range,
    IfRange,
    ContentRange,

    // Security
    CrossOriginEmbedderPolicy,
    CrossOriginOpenerPolicy,
    CrossOriginResourcePolicy,
    ContentSecurityPolicy,
    ContentSecurityPolicyReportOnly,
    ExpectCT,
    StrictTransportSecurity,
    UpgradeInsecureRequests,
    XContentTypeOptions,
    XFrameOptions,
    XPoweredBy,
    XXSSProtection,

    // Fetch Metadata Request Headers
    SecFetchSite,
    SecFetchMode,
    SecFetchUser,
    SecFetchDest,

    // Server-Sent Events
    NEL,

    // Transfer Coding
    TransferEncoding,
    TE,
    Trailer,

    // WebSockets
    SecWebsocketAccept,

    // Other
    AltSvc,
    Date,
    Link,
    RetryAfter,
    ServerTiming,
    Sourcemap,
    Upgrade,
    XDNSPrefetchControl,
}

impl CommonHeader {
    /// Converts the enum variant into a byte array of the header name with a newline (`\n`) at the end.
    pub fn as_bytes(&self, body: &[u8]) -> Vec<u8> {
        let header_name = match self {
            CommonHeader::WwwAuthenticate => "WWW-Authenticate",
            CommonHeader::Authorization => "Authorization",
            CommonHeader::ProxyAuthenticate => "Proxy-Authenticate",
            CommonHeader::ProxyAuthorization => "Proxy-Authorization",
            CommonHeader::Age => "Age",
            CommonHeader::CacheControl => "Cache-Control",
            CommonHeader::ClearSiteData => "Clear-Site-Data",
            CommonHeader::Expires => "Expires",
            CommonHeader::Pragma => "Pragma",
            CommonHeader::Warning => "Warning",
            CommonHeader::AcceptCH => "Accept-CH",
            CommonHeader::SaveData => "Save-Data",
            CommonHeader::LastModified => "Last-Modified",
            CommonHeader::ETag => "ETag",
            CommonHeader::IfMatch => "If-Match",
            CommonHeader::IfNoneMatch => "If-None-Match",
            CommonHeader::IfModifiedSince => "If-Modified-Since",
            CommonHeader::IfUnmodifiedSince => "If-Unmodified-Since",
            CommonHeader::Vary => "Vary",
            CommonHeader::DeltaBase => "Delta-Base",
            CommonHeader::Connection => "Connection",
            CommonHeader::KeepAlive => "Keep-Alive",
            CommonHeader::Accept => "Accept",
            CommonHeader::AcceptEncoding => "Accept-Encoding",
            CommonHeader::AcceptLanguage => "Accept-Language",
            CommonHeader::AIM => "A-IM",
            CommonHeader::IM => "IM",
            CommonHeader::Expect => "Expect",
            CommonHeader::MaxForwards => "Max-Forwards",
            CommonHeader::Cookie => "Cookie",
            CommonHeader::SetCookie => "Set-Cookie",
            CommonHeader::AccessControlAllowOrigin => "Access-Control-Allow-Origin",
            CommonHeader::AccessControlAllowCredentials => "Access-Control-Allow-Credentials",
            CommonHeader::AccessControlAllowHeaders => "Access-Control-Allow-Headers",
            CommonHeader::AccessControlExposeHeaders => "Access-Control-Expose-Headers",
            CommonHeader::AccessControlMaxAge => "Access-Control-Max-Age",
            CommonHeader::AccessControlRequestHeaders => "Access-Control-Request-Headers",
            CommonHeader::AccessControlRequestMethod => "Access-Control-Request-Method",
            CommonHeader::TimingAllowOrigin => "Timing-Allow-Origin",
            CommonHeader::ContentDisposition => "Content-Disposition",
            CommonHeader::ContentLength => "Content-Length",
            CommonHeader::ContentType => "Content-Type",
            CommonHeader::ContentEncoding => "Content-Encoding",
            CommonHeader::ContentLanguage => "Content-Language",
            CommonHeader::ContentLocation => "Content-Location",
            CommonHeader::Forwarded => "Forwarded",
            CommonHeader::XForwardedFor => "X-Forwarded-For",
            CommonHeader::XForwardedHost => "X-Forwarded-Host",
            CommonHeader::XForwardedProto => "X-Forwarded-Proto",
            CommonHeader::Via => "Via",
            CommonHeader::Location => "Location",
            CommonHeader::From => "From",
            CommonHeader::Host => "Host",
            CommonHeader::Referer => "Referer",
            CommonHeader::ReferrerPolicy => "Referrer-Policy",
            CommonHeader::UserAgent => "User-Agent",
            CommonHeader::Allow => "Allow",
            CommonHeader::Server => "Server",
            CommonHeader::AcceptRanges => "Accept-Ranges",
            CommonHeader::Range => "Range",
            CommonHeader::IfRange => "If-Range",
            CommonHeader::ContentRange => "Content-Range",
            CommonHeader::CrossOriginEmbedderPolicy => "Cross-Origin-Embedder-Policy",
            CommonHeader::CrossOriginOpenerPolicy => "Cross-Origin-Opener-Policy",
            CommonHeader::CrossOriginResourcePolicy => "Cross-Origin-Resource-Policy",
            CommonHeader::ContentSecurityPolicy => "Content-Security-Policy",
            CommonHeader::ContentSecurityPolicyReportOnly => "Content-Security-Policy-Report-Only",
            CommonHeader::ExpectCT => "Expect-CT",
            CommonHeader::StrictTransportSecurity => "Strict-Transport-Security",
            CommonHeader::UpgradeInsecureRequests => "Upgrade-Insecure-Requests",
            CommonHeader::XContentTypeOptions => "X-Content-Type-Options",
            CommonHeader::XFrameOptions => "X-Frame-Options",
            CommonHeader::XPoweredBy => "X-Powered-By",
            CommonHeader::XXSSProtection => "X-XSS-Protection",
            CommonHeader::SecFetchSite => "Sec-Fetch-Site",
            CommonHeader::SecFetchMode => "Sec-Fetch-Mode",
            CommonHeader::SecFetchUser => "Sec-Fetch-User",
            CommonHeader::SecFetchDest => "Sec-Fetch-Dest",
            CommonHeader::NEL => "NEL",
            CommonHeader::TransferEncoding => "Transfer-Encoding",
            CommonHeader::TE => "TE",
            CommonHeader::Trailer => "Trailer",
            CommonHeader::SecWebsocketAccept => "Sec-WebSocket-Accept",
            CommonHeader::AltSvc => "Alt-Svc",
            CommonHeader::Date => "Date",
            CommonHeader::Link => "Link",
            CommonHeader::RetryAfter => "Retry-After",
            CommonHeader::ServerTiming => "Server-Timing",
            CommonHeader::Sourcemap => "Sourcemap",
            CommonHeader::Upgrade => "Upgrade",
            CommonHeader::XDNSPrefetchControl => "X-DNS-Prefetch-Control",
        };
        let mut bytes = header_name.as_bytes().to_vec();
        bytes.extend_from_slice(b": ");
        bytes.extend_from_slice(body);
        bytes.extend_from_slice(b"\r\n");
        bytes
    }
}
