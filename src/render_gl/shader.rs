

use gl;
use std::ffi::{ CString, CStr };
use crate::resources::Resources;
use crate::resources::Error as ResourcesError;

pub struct Shader {
    id: gl::types::GLuint,
    gl: gl::Gl,
}

pub struct Program {
    id: gl::types::GLuint,
    gl: gl::Gl,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    UnableToLoadResource { name: String, #[cause] error: ResourcesError },

    #[fail(display = "Can not determine shader type for resource {}", name)]
    CannotDetermineShaderTypeForResource { name : String },

    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { message: String, name: String },

    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String },
}

impl Shader {
    pub fn from_res (
        gl: &gl::Gl,
        res: &Resources,
        name: &str
    ) -> Result<Shader, Error>
    {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER),
        ];

        let shader_kind = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| {
                name.ends_with(file_extension)
            })
            .map(|&(_, kind)| kind)
            .ok_or(Error::CannotDetermineShaderTypeForResource{ name: String::from(name) })?;

        let source: CString = res.load_cstring(name)
            .map_err(|error| Error::UnableToLoadResource{ name: String::from(name), error })?;

        Shader::from_source(gl, &source, shader_kind)
    }

    pub fn from_source(
        gl: &gl::Gl,
        source: &CStr,
        kind: gl::types::GLenum,
    ) -> Result<Shader, Error>
    {
        let id = shader_from_source(&gl, source, kind)?;
        Ok(Shader{ id, gl: gl.clone() })
    }

    pub fn from_vert_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, Error> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, Error> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

impl Program {
    /**
     *  Expects the shader to have a .vert and a .frag file associated with it
     */
    pub fn from_res(gl: &gl::Gl, res: Resources, name: &str)
        -> Result<Program, Error>
    {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vert",
            ".frag",
        ];

        let shaders = POSSIBLE_EXT
            .iter()
            .map(|file_extension| {
                Shader::from_res(gl, &res, &format!("{}{}", name, file_extension))
            })
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(gl, &shaders[..]).map_err(|message| Error::LinkError {
            name: name.into(),
            message
        })
    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe { gl.AttachShader( program_id, shader.id()); }
        }

        unsafe { gl.LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;

        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;

            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                )
            }
            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id()); } // Delete Shader will fail if the shader is still attached to a program when the shader is dropped
        }

        Ok(Program{ id: program_id, gl: gl.clone() })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop (&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // build a buffer of usize + 1 to store the error message
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize)); // Fills the buffer with len spaces

    unsafe { CString::from_vec_unchecked(buffer) }
}

fn shader_from_source(
    gl: &gl::Gl,
    source: &CStr, // Rust strings are not null terminated, so we need to use C strings instead
    shader_kind: gl::types::GLuint
) -> Result<gl::types::GLuint, Error> {
    let id = unsafe { gl.CreateShader(shader_kind) }; // obtain the shader id

    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success); // Check if the shader compiled successfully
    }

    if success == 0 { // Error
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); // get the length of the log message before reading it out
        }

        let error: CString = create_whitespace_cstring_with_len(len as usize);

        // get the error message
        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }
        return Err(Error::CompileError { message: error.to_string_lossy().into_owned(), name: source.to_string_lossy().into_owned() });
    }
    Ok(id)
}
