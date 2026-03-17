use avian2d::prelude::*;

#[derive(PhysicsLayer, Default)]
#[allow(dead_code)]
pub enum GameLayer {
    #[default]
    Default,
    Enemy,
	Building,
}