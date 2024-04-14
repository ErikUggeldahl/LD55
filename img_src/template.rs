use crate::wasm4::*;

{{#sprites}}
// {{name}}
pub const {{rustName}}_WIDTH: u32 = {{width}};
pub const {{rustName}}_HEIGHT: u32 = {{height}};
pub const {{rustName}}_FLAGS: u32 = {{flagsHumanReadable}};
pub const {{rustName}}: [u8; {{length}}] = [ {{bytes}} ];

{{/sprites}}
