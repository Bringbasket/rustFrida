use common::{Error, Result};

pub fn spawn_target(bundle_id: &str) -> Result<i32> {
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    {
        let _ = bundle_id;
        return Err(Error::Unsupported(
            "spawn path is a stub; wire it to launchctl/frontboard in the jailbreak environment".into(),
        ));
    }

    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    {
        let _ = bundle_id;
        Err(Error::Unsupported("spawn path is only available on Apple targets".into()))
    }
}
