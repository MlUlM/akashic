// use bevy_mod_raycast::prelude::RaycastVisibility;
// use bevy_mod_raycast::system_param::Raycast;
//
// pub fn update_hits(
//     raycast: Raycast
// ) {
//     let settings = bevy_mod_raycast::system_param::RaycastSettings {
//         visibility: RaycastVisibility::MustBeVisibleAndInView,
//         filter: &|entity| {
//             let marker_requirement =
//                 !backend_settings.require_markers || marked_targets.get(entity).is_ok();
//             let render_layers_match = match (cam_layers, layers.get(entity)) {
//                 (Some(cam_layers), Ok(entity_layers)) => {
//                     cam_layers.intersects(entity_layers)
//                 }
//                 _ => true, // If either `RenderLayers` components is not present, ignore.
//             };
//             marker_requirement && render_layers_match
//         },
//         early_exit_test: &|entity_hit| {
//             pickables
//                 .get(entity_hit)
//                 .is_ok_and(|pickable| pickable.should_block_lower)
//         },
//     };
// }