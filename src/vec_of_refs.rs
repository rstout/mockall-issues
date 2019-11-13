use mockall::*;
use mockall::predicate::*;

pub struct Artifact {}

pub trait ArtifactStore {
    fn get_artifacts(&self) -> Vec<&Artifact>;
}

mock! {
    pub ArtifactStore {}

    pub trait ArtifactStore<'a> {
        // Using `Vec<&Artifact>`:
        // * error[E0637]: `&` without an explicit lifetime name cannot be used here
        fn get_artifacts<'a>(&self) -> Vec<&'a Artifact>;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_blah() {
        let mut mock_store = MockArtifactStore::new();
        let artifact = Artifact {};
//        mock_store.expect_get_artifacts()
//            .returning(move || vec![&artifact]);
        assert!(true);
    }
}