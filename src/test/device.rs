use super::{Error, Metadata};

/// Platform-independent wrapper around the platform-dependent device types.
/// Note: on iOS, UI automation only works in the iOS Simulator, not on physical
/// devices. As such, it's not necessary to make that distinction here.
#[derive(Clone, Debug)]
pub enum Device {
    /// Wrapper for iOS simulators.
    IOS {
        /// Wrapper around the iOS simulator device itself.
        device: simctl::Device,

        /// Wrapper around the iOS runtime (i.e. version of iOS) that runs on
        /// the simulated device.
        runtime: simctl::list::Runtime,

        /// Device type (i.e. hardware) that is simulated by the device.
        device_type: simctl::list::DeviceType,
    },
}

impl Device {
    /// Opens an URL on this device.
    pub fn open_url(&self, url: &str) -> Result<(), Error> {
        match self {
            Device::IOS { device, .. } => device.open_url(url).map_err(|error| Error::IOS(error)),
        }
    }

    /// Takes a screenshot on this device and returns the PNG-encoded result.
    pub fn screenshot(&self) -> Result<Vec<u8>, Error> {
        match self {
            Device::IOS { device, .. } => device
                .io()
                .screenshot(
                    simctl::io::ImageType::Png,
                    simctl::io::Display::Internal,
                    simctl::io::Mask::Ignored,
                )
                .map_err(|error| Error::IOS(error)),
        }
    }

    /// Summarizes this device into snapshot metadata.
    pub fn metadata(&self) -> Metadata {
        match self {
            Device::IOS {
                device_type,
                runtime,
                ..
            } => Metadata {
                device: Some(device_type.name.clone()),
                os: Some(runtime.name.clone()),
                os_version: Some(runtime.version.clone()),
                ..Default::default()
            },
        }
    }
}
