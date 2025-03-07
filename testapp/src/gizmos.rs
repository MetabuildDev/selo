use bevy::{ecs::system::SystemParam, prelude::*};

pub trait GizmosExt {
    fn plane_3d(&mut self, origin: Vec3, normal: Dir3, color: impl Into<Color>);
}

impl<Config, Clear> GizmosExt for Gizmos<'_, '_, Config, Clear>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
{
    fn plane_3d(&mut self, origin: Vec3, normal: Dir3, color: impl Into<Color>) {
        let color = color.into();
        self.arrow(origin, origin + normal.as_vec3(), color);
        let rotation = Quat::from_rotation_arc(Vec3::Z, normal.as_vec3());
        self.grid(
            Isometry3d::new(origin, rotation),
            UVec2::splat(100),
            Vec2::splat(1.0),
            color,
        );
    }
}

#[derive(SystemParam, Deref, DerefMut)]
pub struct AnimatedGizmos<'w, 's> {
    #[deref]
    pub gizmos: Gizmos<'w, 's>,
    pub time: Res<'w, Time>,
}

impl AnimatedGizmos<'_, '_> {
    pub fn animated_line(
        &mut self,
        start: Vec3,
        end: Vec3,
        color: impl Into<Color>,
        speed: f32,
        segments: usize,
    ) {
        let delta_t = self.time.elapsed_secs();
        let part_length_scalar = (segments as f32 * 2.0).recip();
        let diff = end - start;
        let length = diff.length();
        let color = color.into();
        (0..=segments)
            .map(|n| n as f32 / segments as f32)
            .map(|percent| {
                // this makes the points
                //
                // - start_p E (-1..N) / N
                // - end_p E (0..N+1) / N
                //
                // and then we take max(0.0) and min(1.0) respectively
                // for a smooth transition at the boundaries
                let percent_offset = percent + delta_t * speed / length;
                let modulo = 1.0 + (segments as f32).recip();
                let percent_final = percent_offset % modulo;
                (
                    (percent_final - part_length_scalar).clamp(0.0, 1.0),
                    percent_final.clamp(0.0, 1.0),
                )
            })
            .map(|(start_p, end_p)| (start + start_p * diff, start + end_p * diff))
            .for_each(|(start, end)| {
                self.gizmos.line(start, end, color);
            });
    }
}
