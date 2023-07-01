pub mod packs;

#[cfg(test)]
pub(crate) mod test_util {
    use packs::parsing::ruby::zeitwerk_utils::get_zeitwerk_constant_resolver;
    use std::path::PathBuf;

    use packs::configuration;

    use crate::packs::{
        self, parsing::ruby::packwerk::constant_resolver::ConstantResolver,
    };

    pub const SIMPLE_APP: &str = "tests/fixtures/simple_app";

    pub fn get_absolute_root(fixture_name: &str) -> PathBuf {
        PathBuf::from(fixture_name).canonicalize().unwrap()
    }

    pub fn get_resolver(fixture_name: &str) -> ConstantResolver {
        let absolute_root = get_absolute_root(fixture_name);
        let configuration = configuration::get(&absolute_root);

        get_zeitwerk_constant_resolver(
            &configuration.pack_set,
            &absolute_root,
            &configuration.cache_directory,
        )
    }
}
