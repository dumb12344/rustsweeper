use bevy::prelude::*;

use crate::{constants::NUMBER_COLORS, tile_states::Tile};

#[derive(EntityEvent, Clone)]
pub struct ResetTileEvent(pub Entity);

pub fn reset () -> impl Fn(On<ResetTileEvent>, Commands, Query<(&mut Tile, &mut MeshMaterial2d<ColorMaterial>, &Children, &mut Transform)>, ResMut<Assets<ColorMaterial>>) {
    move |ev: On<'_, '_, ResetTileEvent>, mut commands, mut info, mut color_materials| {   
        let Ok((tile, material, children, mut transform)) = info.get_mut(ev.event_target()) else {return};
        let Some(mut color) = color_materials.get_mut(&material.0) else {return};
        let mut flag_entity = commands.entity(*children.get(1).unwrap());
        flag_entity.insert(Text2d::new(tile.surrounding_mines.to_string()));
        flag_entity.insert(TextColor(Color::from(Srgba::hex(NUMBER_COLORS.get(tile.surrounding_mines as usize).unwrap()).unwrap())));
        // flag_entity.insert(TextColor(Color::WHITE));

        [0, 1, 3].iter().for_each(|i| {
            commands.entity(*children.get(*i).unwrap()).insert(Visibility::Hidden);
        });
        transform.translation.z = 0.0;
        color.color = tile.get_color();
    }
}