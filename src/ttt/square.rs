use bevy::{
    ecs::component::TableStorage,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use stttwmdtt_derive::{Builder, SquareType};

#[derive(Component)]
pub struct SquareSize(pub f32);

pub trait SquareType: Component + Default {}

#[derive(SquareType)]
pub struct Cell;
#[derive(SquareType)]
pub struct Hover;
#[derive(SquareType)]
pub struct GameActive;
#[derive(SquareType)]
pub struct Square;

#[derive(Bundle)]
pub struct SquareBundle<M: Material2d, S: SquareType> {
    mesh: MaterialMesh2dBundle<M>,
    square_type: S,
    size: SquareSize,
}
impl<M: Material2d, S: SquareType> Default for SquareBundle<M, S> {
    fn default() -> Self {
        Self {
            mesh: default(),
            square_type: default(),
            size: SquareSize(50.0),
        }
    }
}

#[derive(Builder)]
pub struct SquareBuilder<'a, S: SquareType> {
    color: Color,
    size: f32,
    optical_size: f32,
    square_type: S,
    z_index: f32,
    visibility: Visibility,
    position: Vec2,
    meshes: Option<&'a mut Assets<Mesh>>,
    materials: Option<&'a mut Assets<ColorMaterial>>,
}
impl<S: SquareType> From<SquareBuilder<'_, S>> for SquareBundle<ColorMaterial, S> {
    fn from(value: SquareBuilder<S>) -> Self {
        if value.meshes.is_none() || value.materials.is_none() {
            panic!("Someone forgot the materials and/or meshes on the SquareBuilder");
        }
        Self {
            mesh: MaterialMesh2dBundle {
                mesh: value
                    .meshes
                    .unwrap()
                    .add(shape::Quad::new(Vec2::splat(value.optical_size)).into())
                    .into(),
                material: value
                    .materials
                    .unwrap()
                    .add(ColorMaterial::from(value.color)),
                visibility: value.visibility,
                transform: Transform::from_translation(value.position.extend(value.z_index)),
                ..default()
            },
            size: SquareSize(value.size),
            square_type: value.square_type,
        }
    }
}
impl<S: SquareType> Default for SquareBuilder<'_, S> {
    fn default() -> Self {
        Self {
            color: default(),
            size: default(),
            optical_size: default(),
            square_type: default(),
            z_index: 0.0,
            visibility: Visibility::Visible,
            position: Vec2::splat(0.0),
            meshes: default(),
            materials: default(),
        }
    }
}
impl<'a, S: SquareType> SquareBuilder<'a, S> {
    pub fn new(meshes: &'a mut Assets<Mesh>, materials: &'a mut Assets<ColorMaterial>) -> Self {
        Self {
            meshes: Some(meshes),
            materials: Some(materials),
            ..default()
        }
    }

    pub fn build(self) -> SquareBundle<ColorMaterial, S> {
        self.into()
    }
}
