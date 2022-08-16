use crate::environment;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
#[path = "./checkpoint_test.rs"]
mod checkpoint_test;

/// Environment variable checkpoint
///
/// A checkpoint stores all environment variables present at the time it was initialized,
/// which then can be restored using the [`restore`] method.
///
/// A checkpoint can be "impure", by excluding a set of variables, which should not be checkpointed,
/// if not specified or added this will reset **every** value.
pub struct Checkpoint {
    snapshot: HashMap<String, String>,
    exclude: HashSet<String>,
}

impl Checkpoint {
    /// Create a new Checkpoint
    ///
    /// This captures all currently present environment variables.
    pub fn new() -> Self {
        let mut env = HashMap::new();
        env.extend(environment::vars());

        Self {
            snapshot: env,
            exclude: HashSet::new(),
        }
    }

    /// Exclude a key from the [`Checkpoint::restore`]
    ///
    /// Keys that have been excluded will not be removed, created or modified in the restore process.
    pub fn exclude<T: Into<String>>(&mut self, key: T) -> &mut Self {
        self.exclude.insert(key.into());

        self
    }

    /// Restore the current state of environment to the state the checkpoint was taken in.
    ///
    /// This will remove added keys, re-create values and set new values.
    /// This will skip setting unchanged values.
    pub fn restore(self) {
        let chk = self.snapshot;

        let mut env = HashMap::new();
        env.extend(environment::vars());

        let exclude: HashSet<_> = self.exclude.iter().collect();

        // only change the variables that are of significance, this means we need to partition into
        // 3 piles:
        //  * to be removed
        //  * to be added
        //  * to be changed

        let chk_keys = &chk.keys().collect::<HashSet<_>>() - &exclude;
        let env_keys = &env.keys().collect::<HashSet<_>>() - &exclude;

        let remove = &env_keys - &chk_keys;
        let create = &chk_keys - &env_keys;

        let modify = (&chk_keys & &env_keys)
            .into_iter()
            .filter(|key| &chk[*key] != &env[*key])
            .collect::<HashSet<_>>();

        for key in remove {
            environment::remove(key);
        }

        for key in &create | &modify {
            environment::set(key, &chk[key]);
        }
    }
}
