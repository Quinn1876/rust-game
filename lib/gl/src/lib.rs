mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


pub use bindings::*;
pub use bindings::Gl as InnerGl; // This is to allow us to view docs for Gl since we are shadowing it with our custom Gl implementation
use std::rc::Rc;
use std::ops::Deref;

/**
 * Here we are redefining Gl to be a reference counted struct
 * so that when we copy it, we are shallow copying it instead
 * of deep copying it.
 */

#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl Gl {
    pub fn load_with<F>(loadfn: F) -> Gl
        where F: FnMut(&'static str) -> *const types::GLvoid
    {
        Gl {
            inner: Rc::new(bindings::Gl::load_with(loadfn))
        }
    }
}

impl Deref for Gl {
    type Target = bindings::Gl;
    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}
