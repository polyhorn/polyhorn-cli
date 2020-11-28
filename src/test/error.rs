/// Error that is returned when an exception occurs during Polyhorn testing.
#[derive(Debug)]
pub enum Error {
    /// Represents an error that occurs when interacting with `simctl`, the
    /// Apple-provided CLI that manages the iOS Simulators on Mac.
    IOS(simctl::Error),
}
