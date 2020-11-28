//! Types and implementations to write snapshots to disk.

use serde::Serialize;
use sha1::{Digest, Sha1};

trait Sha1Ext {
    fn tag(&mut self, prefix: &str, value: Option<&String>);
}

impl Sha1Ext for Sha1 {
    fn tag(&mut self, prefix: &str, value: Option<&String>) {
        if let Some(value) = value {
            self.update(prefix.as_bytes());
            self.update(format!("{}", value.len()).as_bytes());
            self.update(value.as_bytes());
        }
    }
}

/// Represents the metadata of a snapshot.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Metadata {
    /// This is the device type. For example: "iPhone Xs".
    pub device: Option<String>,

    /// This is the operating system. For example: "iOS" or "Android".
    pub os: Option<String>,

    /// This is the version of the operating system. For example: "12.0". This
    /// version does not have to be semver compatible.
    pub os_version: Option<String>,

    /// This is the appearance of the operating system. For example: "dark" or
    /// "light".
    pub os_appearance: Option<String>,

    /// This is the name of the test itself.
    pub test_name: Option<String>,

    /// This is the name of the snapshot.
    pub snapshot_name: Option<String>,
}

impl Metadata {
    /// Computes a sha-1 digest for this metadata.
    pub fn digest(&self) -> Digest {
        let mut sha1 = Sha1::new();
        sha1.tag("device", self.device.as_ref());
        sha1.tag("os", self.os.as_ref());
        sha1.tag("os_version", self.os_version.as_ref());
        sha1.tag("os_appearance", self.os_appearance.as_ref());
        sha1.tag("test_name", self.test_name.as_ref());
        sha1.tag("snapshot_name", self.snapshot_name.as_ref());
        sha1.digest()
    }
}
