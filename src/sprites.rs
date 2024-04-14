





use crate::wasm4::*;

// BeemanDirectDown
pub const BEEMAN_DIRECT_DOWN_WIDTH: u32 = 16;
pub const BEEMAN_DIRECT_DOWN_HEIGHT: u32 = 16;
pub const BEEMAN_DIRECT_DOWN_FLAGS: u32 = BLIT_2BPP;
pub const BEEMAN_DIRECT_DOWN: [u8; 64] = [ 0xff,0xff,0xff,0xff,0xff,0xfd,0x77,0xff,0xff,0xf5,0x5f,0xff,0xff,0xf5,0x0f,0xff,0xff,0xd4,0xaf,0xff,0xff,0xfc,0x0f,0xff,0xff,0xc2,0xa3,0xff,0xff,0x3a,0x28,0xff,0xfc,0xfa,0x2f,0x3f,0xff,0x3e,0x2f,0xcf,0xff,0xde,0x2f,0xf7,0xff,0xf2,0xe3,0xff,0xff,0xf2,0xe3,0xff,0xff,0xf2,0xe3,0xff,0xff,0xfa,0xeb,0xff,0xff,0xea,0xea,0xff ];

// BeemanDirectForward
pub const BEEMAN_DIRECT_FORWARD_WIDTH: u32 = 16;
pub const BEEMAN_DIRECT_FORWARD_HEIGHT: u32 = 16;
pub const BEEMAN_DIRECT_FORWARD_FLAGS: u32 = BLIT_2BPP;
pub const BEEMAN_DIRECT_FORWARD: [u8; 64] = [ 0xff,0xff,0xff,0xff,0xff,0xfd,0x77,0xff,0xff,0xf5,0x5f,0xff,0xff,0xf5,0x0f,0xff,0xff,0xd4,0xaf,0xff,0xff,0xfc,0x0f,0xff,0xff,0xc2,0xa0,0x01,0xff,0x3a,0x2b,0xff,0xfc,0xfa,0x2f,0xff,0xff,0x3e,0x2f,0xff,0xff,0xde,0x2f,0xff,0xff,0xf2,0xe3,0xff,0xff,0xf2,0xe3,0xff,0xff,0xf2,0xe3,0xff,0xff,0xfa,0xeb,0xff,0xff,0xea,0xea,0xff ];

// BeemanDirectUp
pub const BEEMAN_DIRECT_UP_WIDTH: u32 = 16;
pub const BEEMAN_DIRECT_UP_HEIGHT: u32 = 16;
pub const BEEMAN_DIRECT_UP_FLAGS: u32 = BLIT_2BPP;
pub const BEEMAN_DIRECT_UP: [u8; 64] = [ 0xff,0xff,0xff,0xff,0xff,0xfd,0x77,0xff,0xff,0xf5,0x5f,0xf7,0xff,0xf5,0x0f,0xcf,0xff,0xd4,0xaf,0x3f,0xff,0xfc,0x0c,0xff,0xff,0xc2,0xa3,0xff,0xff,0x3a,0x2b,0xff,0xfc,0xfa,0x2f,0xff,0xff,0x3e,0x2f,0xff,0xff,0xde,0x2f,0xff,0xff,0xf2,0xe3,0xff,0xff,0xf2,0xe3,0xff,0xff,0xf2,0xe3,0xff,0xff,0xfa,0xeb,0xff,0xff,0xea,0xea,0xff ];

// BeemanFall
pub const BEEMAN_FALL_WIDTH: u32 = 16;
pub const BEEMAN_FALL_HEIGHT: u32 = 16;
pub const BEEMAN_FALL_FLAGS: u32 = BLIT_2BPP;
pub const BEEMAN_FALL: [u8; 64] = [ 0xff,0xfd,0x77,0xff,0xff,0xf5,0x5f,0xff,0xfd,0xf5,0xff,0x7f,0xfc,0xd7,0x0f,0x3f,0xfc,0xfc,0xaf,0x3f,0xff,0x3c,0x0f,0x3f,0xff,0xc2,0xa0,0xff,0xff,0xfa,0x2f,0xff,0xff,0xfa,0x2f,0xff,0xff,0xfe,0x2f,0xff,0xff,0xf2,0x23,0xff,0xfe,0xca,0xe8,0xef,0xfe,0x2b,0xfa,0x2f,0xfe,0xaf,0xfe,0xaf,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff ];

// BeemanJump
pub const BEEMAN_JUMP_WIDTH: u32 = 16;
pub const BEEMAN_JUMP_HEIGHT: u32 = 16;
pub const BEEMAN_JUMP_FLAGS: u32 = BLIT_2BPP;
pub const BEEMAN_JUMP: [u8; 64] = [ 0xff,0xff,0xff,0xff,0xff,0xfd,0x77,0xff,0xff,0xf5,0x5f,0xff,0xff,0xf5,0x0f,0xf7,0xff,0xd4,0xaf,0xcf,0xff,0xfc,0x0f,0x3f,0xff,0xc2,0xa0,0xff,0xff,0xca,0x2f,0xff,0xff,0x3a,0x2f,0xff,0xff,0x3e,0x2f,0xff,0xff,0x7e,0x20,0xff,0xff,0xf8,0xea,0x3f,0xff,0xe3,0xfe,0x3f,0xff,0x8f,0xfe,0xbf,0xff,0xaf,0xfe,0xaf,0xff,0xeb,0xff,0xff ];

// BeemanRun
pub const BEEMAN_RUN_WIDTH: u32 = 16;
pub const BEEMAN_RUN_HEIGHT: u32 = 32;
pub const BEEMAN_RUN_FLAGS: u32 = BLIT_2BPP;
pub const BEEMAN_RUN: [u8; 128] = [ 0xff,0xfd,0x77,0xff,0xff,0xf5,0x5f,0xff,0xff,0xf5,0xff,0xff,0xff,0xd7,0x0f,0xff,0xff,0xfc,0xaf,0xff,0xff,0xfc,0x0f,0xf7,0xff,0x02,0xa0,0x0f,0xfc,0xfa,0x2f,0xff,0xf7,0xfa,0x2f,0xff,0xff,0xfe,0x2f,0xff,0xff,0xfe,0x20,0xff,0xff,0xf8,0xea,0x3f,0xff,0xa8,0xfe,0x3f,0xff,0x80,0xfe,0x3f,0xff,0xbf,0xfe,0xaf,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfd,0x77,0xff,0xff,0xf5,0x5f,0xff,0xff,0xf5,0x0f,0xff,0xff,0xd4,0xaf,0xff,0xff,0xfc,0x0f,0xff,0xff,0xc2,0xa3,0xff,0xff,0xca,0x23,0xff,0xff,0xca,0x23,0xff,0xff,0xc1,0x2c,0x7f,0xff,0xfe,0x2f,0xff,0xff,0xfe,0x28,0xff,0xff,0xfe,0x38,0xff,0xff,0xea,0x38,0xff,0xff,0xe0,0xfa,0xff ];

// BeemanStand
pub const BEEMAN_STAND_WIDTH: u32 = 16;
pub const BEEMAN_STAND_HEIGHT: u32 = 16;
pub const BEEMAN_STAND_FLAGS: u32 = BLIT_2BPP;
pub const BEEMAN_STAND: [u8; 64] = [ 0xff,0xff,0xff,0xff,0xff,0xfd,0x77,0xff,0xff,0xf5,0x5f,0xff,0xff,0xf5,0x0f,0xff,0xff,0xd4,0xaf,0xff,0xff,0xfc,0x0f,0xff,0xff,0xc2,0xa3,0xff,0xff,0xca,0x28,0xff,0xff,0x3a,0x2f,0x3f,0xff,0x3e,0x2c,0xff,0xff,0x7e,0x27,0xff,0xff,0xf2,0xe3,0xff,0xff,0xf2,0xe3,0xff,0xff,0xf2,0xe3,0xff,0xff,0xfa,0xeb,0xff,0xff,0xea,0xea,0xff ];


