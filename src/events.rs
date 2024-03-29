use bevy::ecs::{entity::Entity, event::Event};
use std::fmt;

use crate::animation::AnimationId;

/// A Bevy event emitted when an animation reaches a point of interest
///
/// * when a clip cycle ends
/// * when a clip ends (if the clip repeats multiple times, only occurs at the end of the last cycle)
/// * when an animation cycle ends
/// * when an animation ends (if the animation repeats multiple times, only occurs at the end of the last cycle)
/// * when an [animation marker](crate::prelude::AnimationClip::add_marker) is hit
///
/// # Example
///
/// You can use those events to be notified of a clip/animation ending.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// # fn go_to_main_menu() {}
/// fn death_transition_system(
///     mut events: EventReader<AnimationEvent>,
///     library: Res<SpritesheetLibrary>
/// ) {
///     for event in events.read() {
///         match event {
///             // Some animation just ended...
///             AnimationEvent::AnimationEnd { animation_id, .. } => {
///                 // ... it was the main character's death animation,
///                 // we can go back to the main menu
///
///                 if library.is_animation_name(*animation_id, "character dies") {
///                     go_to_main_menu();
///                 }
///             }
///             // Ignore other events
///             _ => (),
///         }
///     }
/// }
/// ```
///
/// # Example
///
/// You can also add markers to specific frames of a clip to be notified of an animation reaching points of interest.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// # let mut library = SpritesheetLibrary::new();
/// // Let's create a marker to be notified when the exact frame
/// // of the character shooting their gun is played
/// let marker_id = library.new_marker();
///
/// // Naming a marker is not required but it can be convenient to refer to it later
/// // if you don't want to keep its ID around
/// library.name_marker(marker_id, "bullet goes out");
///
/// let clip_id = library.new_clip(|clip| {
///     clip
///         .push_frame_indices([10, 11, 15, 16, 17])
///         // The character shoots their gun on the fourth frame
///         .add_marker(marker_id, 3);
/// });
/// ```
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// # fn spawn_bullet() {}
/// // We can watch events from any system and react to them
/// fn spawn_visual_effects_system(
///     mut events: EventReader<AnimationEvent>,
///     library: Res<SpritesheetLibrary>
///) {
///     for event in events.read() {
///         match event {
///             // Some marker was just hit...
///             AnimationEvent::MarkerHit { marker_id, .. } => {
///                 // ... it was our "bullet goes out" marker, let's spawn a bullet.
///
///                 if library.is_marker_name(*marker_id, "bullet goes out") {
///                     spawn_bullet();
///                 }
///             }
///             // Ignore other events
///             _ => (),
///         }
///     }
/// }
/// ```
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationEvent {
    /// An animation marker has been hit
    MarkerHit {
        entity: Entity,
        marker_id: AnimationMarkerId,
        animation_id: AnimationId,
        stage_index: usize,
    },
    /// A cycle of a clip has ended
    ClipCycleEnd {
        entity: Entity,
        animation_id: AnimationId,
        stage_index: usize,
    },
    /// An clip ended
    ClipEnd {
        entity: Entity,
        animation_id: AnimationId,
        stage_index: usize,
    },
    /// A cycle of an animation has ended
    AnimationCycleEnd {
        entity: Entity,
        animation_id: AnimationId,
    },
    /// An animation has ended
    AnimationEnd {
        entity: Entity,
        animation_id: AnimationId,
    },
}

/// An opaque identifier that references an animation marker.
///
/// Returned by [SpritesheetLibrary::new_marker](crate::prelude::SpritesheetLibrary::new_marker).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AnimationMarkerId {
    pub(crate) value: usize,
}

impl fmt::Display for AnimationMarkerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "marker:{}", self.value)
    }
}
