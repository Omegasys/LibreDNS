use crate::filtering::{blocklist::Blocklist, allowlist::Allowlist};

pub struct Updater;

impl Updater {
    pub fn update_blocklist(blocklist: &mut Blocklist) {
        println!("Updating blocklist...");
        blocklist.insert("malware.example");
        blocklist.insert("phishing.badsite");
    }

    pub fn update_allowlist(allowlist: &mut Allowlist) {
        println!("Updating allowlist...");
        allowlist.insert("safe.example");
    }
}
