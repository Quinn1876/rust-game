use gl;

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

// -----------------------------------------------------------------

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}
pub type ArrayBuffer = Buffer<BufferTypeArray>;

// ----------------------------------------------------------------

pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;

// ----------------------------------------------------------------

pub struct Buffer<B> where B : BufferType {
    vbo: gl::types::GLuint,
    gl: gl::Gl,
    _marker: ::std::marker::PhantomData<B>
}

impl<B> Buffer<B> where B : BufferType {
    pub fn new(gl: &gl::Gl)
    -> Buffer<B>
    {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }

        Buffer {
            vbo,
            gl: gl.clone(),
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn bind(&self)
    {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self)
    {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T])
    {
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of the data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to raw data
                gl::STATIC_DRAW, // usage
            )
        }
    }
}

impl<B> Drop for Buffer<B> where B : BufferType {
    fn drop(&mut self)
    {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vbo);
        }
    }
}

// --------------------------------------------------------------------------------------------------------------

pub struct VertexArray {
    gl: gl::Gl,
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new(gl: &gl::Gl)
    -> VertexArray
    {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        VertexArray {
            gl: gl.clone(),
            vao
        }
    }

    pub fn bind(&self)
    {
        unsafe {
            self.gl.BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self)
    {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self)
    {
        unsafe {
            self.gl.DeleteVertexArrays(1, &mut self.vao);
        }
    }
}
