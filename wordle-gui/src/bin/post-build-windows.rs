#[cfg(target_os = "windows")]

fn main() {
    use codesign::{SignParams, SignTool};
    use std::env;

    let thumbprint = env::var("THUMBPRINT").expect("$Env:THUMBPRINT is not set");

    let signtool = SignTool::locate_latest().unwrap();

    let sign_params = SignParams {
        digest_algorithm: "sha256".to_owned(),
        certificate_thumbprint: thumbprint,
        timestamp_url: Some("http://timestamp.digicert.com".to_owned()),
    };

    signtool
        .sign("target\\release\\wordle-gui.exe", &sign_params)
        .unwrap();
}
