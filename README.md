# MIME Classifier / Media Type Sniffer

This crate exposes the [MIME Classifier from the Servo web engine](https://github.com/servo/servo/blob/master/components/net/mime_classifier.rs)
as a standalone library.

It implements the [WHATWG MIME Sniffing](https://mimesniff.spec.whatwg.org/)
standard to guess the [Media Type](https://www.iana.org/assignments/media-types/media-types.xhtml)
(also known as MIME type) of a resource from its content. It enables browsers
to properly interpret a server response even when the `Content-Type` header is
missing or invalid.

The current version of the library was extracted at the date 2022-04-11 from
the commit [`8d684eff7d6f8815422cb4c30b43df0035c5069a`](https://github.com/servo/servo/tree/8d684eff7d6f8815422cb4c30b43df0035c5069a).
If you are a member of the Servo and wish to maintain this library yourself,
I'd be glad to transfer ownership of the crate: please open a GitHub issue or
send me an email.

## Usage

```rust
use mime_classifier::{ApacheBugFlag, LoadContext, MimeClassifier, NoSniffFlag};

pub fn main() {
    // Create a classifier using default configuration
    let classifier = MimeClassifier::new();
    // Select the context, this is used to help the classifier based on where
    // the resource is loaded from. `Browsing` corresponds to simply typing
    // the URL in the address bar.
    let context = LoadContext::Browsing;
    // Flag indicating that sniffing should be avoided. This usually corresponds
    // to the server sending the header value `X-Content-Type-Options = "nosniff"`
    // but may also be applied automatically by the browser (e.g. `fetch` API)
    let no_sniff_flag = NoSniffFlag::Off;
    // Enable workaround for an Apache bug when server incorrectly sends a
    // `text/plain` or similar `Content-Type`.
    // See <https://mimesniff.spec.whatwg.org/#ref-for-check-for-apache-bug-flag>
    let apache_bug_flag = ApacheBugFlag::Off;
    // `Content-Type` set by the server, if any
    let supplied_type: Option<mime::Mime> = None;
    // Response body to classify
    let body: &[u8] = include_bytes!("../servo_logo.png");

    let computed_type = classifier.classify(context, no_sniff_flag, apache_bug_flag, &supplied_type, body);
    assert_eq!(computed_type, mime::IMAGE_PNG);
}
```

## License

Code in this crate retains its original [Mozilla Public License, version 2.0 (MPL-2.0)](https://mozilla.org/MPL/2.0/)
license from the Servo project.
