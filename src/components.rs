use std::time::Duration;

use benimator::FrameRate;
use bevy::{prelude:: *};

#[derive(Component)]
pub struct Name(String);

// #[derive(Component)]
// pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Debug)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig{
    pub fn new(first: usize, last: usize, fps: u8) -> Self{
        Self{
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer{
        Timer::new(Duration::from_secs_f32(1. / fps as f32), TimerMode::Repeating)
    }
}


impl Default for AnimationConfig{
    fn default() -> Self {
        Self::new(0, 3, 12)
    }
}


#[derive(Component, Deref, Clone, Debug)]
pub struct Animation(pub benimator::Animation);

impl Default for Animation{
    fn default() -> Self {
        Self(benimator::Animation::from_indices(0..=3, FrameRate::from_fps(12.0)))
    }
}

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(pub benimator::State);


#[derive(Component, Deref, DerefMut, Default)]
pub struct Acceleration{
    #[deref]
    x: f32,
    y: f32,
}

impl Acceleration{
    pub fn new(x: f32, y: f32) -> Self{
        Self{x, y}
    }
    pub fn empty() -> Self{
        Self { x:0., y: 0.}
    }
}

#[derive(Resource)]
pub struct FrameTimer(pub Timer);