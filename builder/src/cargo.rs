use std::{collections::BTreeMap, process::Command};

#[derive(Clone)]
pub struct CargoOpts {
    package: String,
    action: String,
    release: bool,
    features: Vec<String>,
    env: BTreeMap<String, String>,
    target: Option<String>,
    config: Vec<String>,
    extra_args: Vec<String>,
}

#[allow(dead_code)]
impl CargoOpts {
    pub fn new(package: String) -> Self {
        Self {
            package,
            action: "build".into(),
            release: false,
            features: Vec::new(),
            env: BTreeMap::new(),
            target: None,
            config: Vec::new(),
            extra_args: Vec::new(),
        }
    }

    pub fn release(&mut self) -> &mut Self {
        self.release = true;
        self
    }

    pub fn target(&mut self, target: String) -> &mut Self {
        self.target = Some(target);
        self
    }

    pub fn action<S: AsRef<str>>(&mut self, action: S) -> &mut Self {
        self.action = action.as_ref().to_string();
        self
    }

    pub fn feature<S: AsRef<str>>(&mut self, feature: S) -> &mut Self {
        self.features.push(feature.as_ref().to_string());
        self
    }

    pub fn env<S1: AsRef<str>, S2: AsRef<str>>(&mut self, name: S1, value: S2) -> &mut Self {
        self.env.insert(name.as_ref().into(), value.as_ref().into());
        self
    }

    pub fn config(&mut self, cfg: String) -> &mut Self {
        self.config.push(cfg);
        self
    }

    pub fn arg<S: AsRef<str>>(&mut self, a: S) -> &mut Self {
        self.extra_args.push(a.as_ref().into());
        self
    }

    pub fn done(&mut self) -> anyhow::Result<()> {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
        let mut cmd = Command::new(cargo);
        cmd.arg(&self.action);
        cmd.arg("-p").arg(&self.package);

        if self.release {
            cmd.arg("--release");
        }
        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }

        if !self.features.is_empty() {
            cmd.arg("--features").arg(self.features.join(","));
        }

        for cfg in &self.config {
            cmd.arg("--config").arg(cfg);
        }

        for (name, value) in self.env.iter() {
            cmd.env(name, value);
        }

        for a in &self.extra_args {
            cmd.arg(a);
        }

        let status = cmd.status()?;
        anyhow::ensure!(
            status.success(),
            "cargo {} -p {} failed",
            self.action,
            self.package
        );
        Ok(())
    }
}
