use crate::{
    bees::Bees,
    input::{is_button_down, is_button_pressed, Button as Btn},
    platform::Platform,
    render::set_2bpp_colors,
    sprites::*,
    util::{clamp, distance},
    wasm4::*,
    BEE_COUNT, MAX_POS,
};
use paste::paste;

const JUMP_FORCE: f32 = 2.5;
const MAX_HORIZONTAL_SPEED: f32 = 1.0;
const HORIZONTAL_FORCE: f32 = 0.8;
const GRAVITY: f32 = 0.1;

const BEE_PROXIMITY: f32 = 2.0;
const BEE_SLOWING: f32 = 0.06;
const BEE_X_DISPLACEMENT: f32 = 1.2;
const BEE_Y_DISPLACEMENT: f32 = 2.0;

const BEE_JUMP_PROXIMITY: f32 = 16.0;
const BEE_JUMP_FORCE: f32 = 2.0;
const BEE_JUMP_DISPLACEMENT: f32 = 4.0;

enum Facing {
    Left,
    Right,
}

pub struct Beeman {
    pub pos_x: f32,
    pub pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    grounded: bool,
    facing: Facing,
    animation: &'static Animation,
    frame: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AnimationType {
    Stand,
    Run,
    Jump,
    Fall,
    DirectForward,
    DirectUp,
    DirectDown,
}

struct Animation {
    anim_type: AnimationType,
    frames: &'static [u8],
    width: u32,
    height: u32,
    flags: u32,
    frame_count: u8,
}

macro_rules! animation {
    ($name:ident, $type:ident, $frame:ident, $frame_count:expr) => {
        paste! {
            static $name: Animation = Animation {
                anim_type: AnimationType::$type,
                frames: &$frame,
                width: [<$frame _WIDTH>],
                height: [<$frame _WIDTH>],
                flags: [<$frame _FLAGS>],
                frame_count: $frame_count,
            };
        }
    };
}

animation!(BEEMAN_STAND_ANIM, Stand, BEEMAN_STAND, 1);
animation!(BEEMAN_RUN_ANIM, Run, BEEMAN_RUN, 2);
animation!(BEEMAN_JUMP_ANIM, Jump, BEEMAN_JUMP, 1);
animation!(BEEMAN_FALL_ANIM, Fall, BEEMAN_FALL, 1);
animation!(
    BEEMAN_DIRECT_FORWARD_ANIM,
    DirectForward,
    BEEMAN_DIRECT_FORWARD,
    1
);
animation!(BEEMAN_DIRECT_UP_ANIM, DirectUp, BEEMAN_DIRECT_UP, 1);
animation!(BEEMAN_DIRECT_DOWN_ANIM, DirectDown, BEEMAN_DIRECT_DOWN, 1);

impl Beeman {
    pub fn new() -> Self {
        Self {
            pos_x: 80.0,
            pos_y: 80.0,
            vel_x: 0.0,
            vel_y: 0.0,
            grounded: false,
            facing: Facing::Right,
            animation: &BEEMAN_STAND_ANIM,
            frame: 0,
        }
    }

    pub fn init(&mut self, pos_x: f32, pos_y: f32) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;
        self.vel_x = 0.0;
        self.vel_y = 0.0;
        self.grounded = false;
        self.facing = Facing::Right;
        self.animation = &BEEMAN_STAND_ANIM;
        self.frame = 0;
    }

    pub fn update(&mut self, platforms: &[Platform], bees: &mut Bees, new_frame: bool) {
        // Left and right
        let mut vel_x = self.vel_x;
        if is_button_pressed(Btn::Left) && vel_x > -MAX_HORIZONTAL_SPEED {
            vel_x -= HORIZONTAL_FORCE;
            self.facing = Facing::Left;
        }
        if is_button_pressed(Btn::Right) && vel_x < MAX_HORIZONTAL_SPEED {
            vel_x += HORIZONTAL_FORCE;
            self.facing = Facing::Right;
        }

        // Friction
        vel_x *= 0.8;

        self.vel_x = vel_x;

        // Gravity
        self.vel_y += GRAVITY;

        // Jump
        if self.grounded && is_button_down(Btn::Up) {
            self.vel_y = -JUMP_FORCE;
            self.grounded = false;

            // Push bees
            for i in 0..BEE_COUNT {
                if distance(self.pos_x, self.pos_y, bees.pos[i].x, bees.pos[i].y)
                    < BEE_JUMP_PROXIMITY
                {
                    bees.vel[i].y += BEE_JUMP_FORCE;
                    let pos_y = bees.pos[i].y + BEE_JUMP_DISPLACEMENT;
                    bees.pos[i].y = clamp(pos_y, 0.0, MAX_POS);
                }
            }
        }

        // Platform collision
        self.platform_collision(platforms);

        // Bee collision
        self.bee_collision(bees);

        // Velocity movement
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;

        // Grounding
        if self.pos_y == MAX_POS {
            self.grounded = true;
            self.vel_y = 0.0;
        }

        // Advance frame
        if new_frame {
            self.frame += 1;
            if self.frame == self.animation.frame_count {
                self.frame = 0;
            }
        }
    }

    fn platform_collision(&mut self, platforms: &[Platform]) {
        if self.vel_y < 0.0 {
            return;
        }

        for platform in platforms {
            if self.pos_x > platform.pos_x
                && self.pos_x < platform.pos_x + platform.width
                && (self.pos_y + self.vel_y) as i32 >= platform.pos_y as i32
                && (self.pos_y + self.vel_y) as i32 <= (platform.pos_y + platform.height) as i32
            {
                self.pos_y = platform.pos_y - 1.0;
                self.vel_y = 0.0;
                self.grounded = true;

                break;
            }
        }
    }

    fn bee_collision(&mut self, bees: &mut Bees) {
        // Ignore if moving up
        if self.vel_y < 0.0 {
            return;
        }

        // Slowdown by and displacement of bees
        for i in 0..BEE_COUNT {
            if distance(self.pos_x, self.pos_y, bees.pos[i].x, bees.pos[i].y) < BEE_PROXIMITY {
                self.vel_y -= BEE_SLOWING;
                if bees.pos[i].x < self.pos_x {
                    bees.vel[i].x -= BEE_X_DISPLACEMENT;
                } else {
                    bees.vel[i].x += BEE_X_DISPLACEMENT;
                }
                bees.vel[i].y += BEE_Y_DISPLACEMENT;
            }
        }

        // Ground on bees
        if self.vel_y < 0.0 {
            self.vel_y = 0.0;
            self.grounded = true;
        }
    }

    pub fn draw(&mut self) {
        set_2bpp_colors();

        let prev_animation = self.animation;

        // Determine animation
        if self.grounded {
            if self.vel_x.abs() > 0.1 {
                self.animation = &BEEMAN_RUN_ANIM;
            } else if is_button_pressed(Btn::One) {
                let mouse_x = unsafe { *MOUSE_X } as f32;
                let mouse_y = unsafe { *MOUSE_Y } as f32;

                self.facing = if mouse_x < self.pos_x {
                    Facing::Left
                } else {
                    Facing::Right
                };
                self.animation = if mouse_y < self.pos_y - 16.0 {
                    &BEEMAN_DIRECT_UP_ANIM
                } else if mouse_y > self.pos_y + 16.0 {
                    &BEEMAN_DIRECT_DOWN_ANIM
                } else {
                    &BEEMAN_DIRECT_FORWARD_ANIM
                };
            } else {
                self.animation = &BEEMAN_STAND_ANIM;
            }
        } else if self.vel_y < 0.0 {
            self.animation = &BEEMAN_JUMP_ANIM;
        } else {
            self.animation = &BEEMAN_FALL_ANIM;
        }

        if self.animation.anim_type != prev_animation.anim_type {
            self.frame = 0;
        }

        self.draw_animation(self.animation);
    }

    fn draw_animation(&self, animation: &Animation) {
        let flip = match self.facing {
            Facing::Left => BLIT_FLIP_X,
            Facing::Right => 0,
        };

        blit_sub(
            animation.frames,
            self.pos_x as i32 - animation.width as i32 / 2,
            self.pos_y as i32 - (animation.height - 1) as i32,
            animation.width,
            animation.height,
            0,
            self.frame as u32 * animation.width,
            animation.width,
            animation.flags | flip,
        );
    }
}
