use mockall::*;
use mockall::predicate::*;

pub trait Committee {
    fn threshold(&self) -> u32;
}

pub trait Config {
    fn get_committee(&self, threshold: u32) -> &dyn Committee;
}

mock! {
    pub Config {}

    pub trait Config {
        // Using `&dyn Committee`:
        // * error[E0277]: the size for values of type `(dyn Committee + 'static)` cannot
        // * be known at compilation time
        //
        // Using `&'static Committee`:
        // * warning: trait objects without an explicit `dyn` are deprecated
        fn get_committee(&self, threshold: u32) -> &'static Committee;
    }
}
