#![allow(missing_docs, unused_imports, dead_code)]

use super::raw::CombinedCatData;

#[derive(Debug)]
struct Cat {}

impl Cat {
    fn from_combined(_data: CombinedCatData) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::TEST_CONFIG, data::cat::raw::read_data_file};

    #[test]
    fn tmp() {
        #[allow(unused_variables)]
        let cond = true;
        let cond = false;
        if cond {
            return;
        }
        let file_name = "unit026.csv";
        let version = TEST_CONFIG.version.current_version();
        panic!(
            "{:#?}",
            read_data_file(file_name, version)
                .map(Cat::from_combined)
                .collect::<Vec<_>>()
        )
    }
}
