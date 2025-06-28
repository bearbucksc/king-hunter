use reqwest::blocking::get;
use semver::Version;

const GITHUB_REPO: &str = "https://api.github.com/repos/yourusername/king-hunter/releases/latest ";

pub fn check_for_update(current_version: &str) {
    let response = get(GITHUB_REPO).unwrap();
    let release_info: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();

    let latest_version = Version::parse(release_info["tag_name"].as_str().unwrap()).unwrap();
    let current_version = Version::parse(current_version).unwrap();

    if latest_version > current_version {
        println!("New version available: {}", latest_version);
    } else {
        println!("You are running the latest version.");
    }
}
