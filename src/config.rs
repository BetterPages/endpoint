use std::env::var;
use std::sync::LazyLock;

/// Default values are inlined.

/// The brand of the server in the res header "Server"
pub static BRANDING: LazyLock<String> = LazyLock::new(|| get_branding());
/// Port to be binded to
pub static PORT: LazyLock<u16> = LazyLock::new(|| get_port());
/// Address of the root server
pub static ROOT_IP: LazyLock<String> = LazyLock::new(|| get_root_ip());

fn get_branding() -> String {
    if let Ok(branding) = var("BRANDING") {
        branding
    } else {
        "Pages".into()
    }
}

fn get_port() -> u16 {
    if let Ok(port) = var("PORT") {
        port.parse().unwrap()
    } else {
        3000
    }
}

fn get_root_ip() -> String {
    if let Ok(ip) = var("ROOT") {
        ip
    } else {
        "http://[::1]:50051".into()
    }
}
