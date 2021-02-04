use std::process::Command;

#[derive(Debug)]
pub struct UbuntuVersion {
    pub description: String,
    pub release: String,
    pub codename: String,
}

pub fn ubuntu_version() -> Option<UbuntuVersion> {
    let output = Command::new("lsb_release").args(&["-a"]).output();
    let output = match output {
        Ok(o) => o.stdout,
        Err(_) => {
            return None;
        }
    };

    let output = match String::from_utf8(output) {
        Ok(str) => str,
        Err(_) => {
            return None;
        }
    };

    let mut distributor_found = true;
    let mut description = "";
    let mut release = "";
    let mut codename = "";

    for line in output.lines() {
        if !distributor_found && line.starts_with("Distributor ID:") {
            let os = line.split(":").last().unwrap();
            if os.trim() != "Ubuntu" {
                return None;
            }

            distributor_found = true;
        } else if line.starts_with("Description:") {
            description = line.split(":").last().unwrap().trim();
        } else if line.starts_with("Release:") {
            release = line.split(":").last().unwrap().trim();
        } else if line.starts_with("Codename:") {
            codename = line.split(":").last().unwrap().trim();
        }
    }

    if !distributor_found || description.is_empty() || release.is_empty() || codename.is_empty() {
        None
    } else {
        Some(UbuntuVersion {
            description: description.to_string(),
            release: release.to_string(),
            codename: codename.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ubuntu_version;

    #[test]
    fn test() {
        if let Some(ver) = ubuntu_version() {
            assert!(ver.description.starts_with("Ubuntu"));
            if ver.release == "20.10" {
                assert_eq!(ver.codename, "groovy");
            }
        }
    }
}
