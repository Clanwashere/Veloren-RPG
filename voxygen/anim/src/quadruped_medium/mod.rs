pub mod alpha;
pub mod beta;
pub mod dash;
pub mod feed;
pub mod hoof;
pub mod idle;
pub mod jump;
pub mod leapmelee;
pub mod run;
pub mod stunned;

// Reexports
pub use self::{
    alpha::AlphaAnimation, beta::BetaAnimation, dash::DashAnimation, feed::FeedAnimation,
    hoof::HoofAnimation, idle::IdleAnimation, jump::JumpAnimation, leapmelee::LeapMeleeAnimation,
    run::RunAnimation, stunned::StunnedAnimation,
};

use super::{make_bone, vek::*, FigureBoneData, Skeleton};
use common::comp::{self};
use core::convert::TryFrom;

pub type Body = comp::quadruped_medium::Body;

skeleton_impls!(struct QuadrupedMediumSkeleton {
    + head,
    + neck,
    + jaw,
    + tail,
    + torso_front,
    + torso_back,
    + ears,
    + leg_fl,
    + leg_fr,
    + leg_bl,
    + leg_br,
    + foot_fl,
    + foot_fr,
    + foot_bl,
    + foot_br,
});

impl Skeleton for QuadrupedMediumSkeleton {
    type Attr = SkeletonAttr;
    type Body = Body;

    const BONE_COUNT: usize = 15;
    #[cfg(feature = "use-dyn-lib")]
    const COMPUTE_FN: &'static [u8] = b"quadruped_medium_compute_mats\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "quadruped_medium_compute_mats")]
    fn compute_matrices_inner(
        &self,
        base_mat: Mat4<f32>,
        buf: &mut [FigureBoneData; super::MAX_BONE_COUNT],
    ) -> Vec3<f32> {
        let torso_front_mat = base_mat * Mat4::<f32>::from(self.torso_front);
        let torso_back_mat = torso_front_mat * Mat4::<f32>::from(self.torso_back);
        let neck_mat = torso_front_mat * Mat4::<f32>::from(self.neck);
        let leg_fl_mat = torso_front_mat * Mat4::<f32>::from(self.leg_fl);
        let leg_fr_mat = torso_front_mat * Mat4::<f32>::from(self.leg_fr);
        let leg_bl_mat = torso_back_mat * Mat4::<f32>::from(self.leg_bl);
        let leg_br_mat = torso_back_mat * Mat4::<f32>::from(self.leg_br);
        let head_mat = neck_mat * Mat4::<f32>::from(self.head);

        *(<&mut [_; Self::BONE_COUNT]>::try_from(&mut buf[0..Self::BONE_COUNT]).unwrap()) = [
            make_bone(head_mat),
            make_bone(neck_mat),
            make_bone(head_mat * Mat4::<f32>::from(self.jaw)),
            make_bone(torso_back_mat * Mat4::<f32>::from(self.tail)),
            make_bone(torso_front_mat),
            make_bone(torso_back_mat),
            make_bone(head_mat * Mat4::<f32>::from(self.ears)),
            make_bone(leg_fl_mat),
            make_bone(leg_fr_mat),
            make_bone(leg_bl_mat),
            make_bone(leg_br_mat),
            make_bone(leg_fl_mat * Mat4::<f32>::from(self.foot_fl)),
            make_bone(leg_fr_mat * Mat4::<f32>::from(self.foot_fr)),
            make_bone(leg_bl_mat * Mat4::<f32>::from(self.foot_bl)),
            make_bone(leg_br_mat * Mat4::<f32>::from(self.foot_br)),
        ];
        Vec3::default()
    }
}

pub struct SkeletonAttr {
    head: (f32, f32),
    neck: (f32, f32),
    jaw: (f32, f32),
    tail: (f32, f32),
    torso_back: (f32, f32),
    torso_front: (f32, f32),
    ears: (f32, f32),
    leg_f: (f32, f32, f32),
    leg_b: (f32, f32, f32),
    feet_f: (f32, f32, f32),
    feet_b: (f32, f32, f32),
    scaler: f32,
    startangle: f32,
    tempo: f32,
    spring: f32,
    feed: (bool, f32),
}

impl<'a> std::convert::TryFrom<&'a comp::Body> for SkeletonAttr {
    type Error = ();

    fn try_from(body: &'a comp::Body) -> Result<Self, Self::Error> {
        match body {
            comp::Body::QuadrupedMedium(body) => Ok(SkeletonAttr::from(body)),
            _ => Err(()),
        }
    }
}

impl Default for SkeletonAttr {
    fn default() -> Self {
        Self {
            head: (0.0, 0.0),
            neck: (0.0, 0.0),
            jaw: (0.0, 0.0),
            tail: (0.0, 0.0),
            torso_back: (0.0, 0.0),
            torso_front: (0.0, 0.0),
            ears: (0.0, 0.0),
            leg_f: (0.0, 0.0, 0.0),
            leg_b: (0.0, 0.0, 0.0),
            feet_f: (0.0, 0.0, 0.0),
            feet_b: (0.0, 0.0, 0.0),
            scaler: 0.0,
            startangle: 0.0,
            tempo: 0.0,
            spring: 0.0,
            feed: (false, 0.0),
        }
    }
}

impl<'a> From<&'a Body> for SkeletonAttr {
    fn from(body: &'a Body) -> Self {
        use comp::quadruped_medium::{BodyType::*, Species::*};
        Self {
            head: match (body.species, body.body_type) {
                (Grolgar, _) => (0.0, -1.0),
                (Saber, _) => (5.0, -3.0),
                (Tuskram, _) => (2.0, 1.0),
                (Lion, _) => (4.5, 2.0),
                (Tarasque, _) => (-4.0, 3.5),
                (Tiger, _) => (2.0, 1.0),
                (Wolf, _) => (1.5, 3.0),
                (Frostfang, _) => (1.0, -2.0),
                (Mouflon, _) => (0.5, 1.5),
                (Catoblepas, _) => (-1.0, -6.5),
                (Bonerattler, _) => (1.0, 2.5),
                (Deer, Male) => (1.5, 3.5),
                (Deer, Female) => (1.5, 3.5),
                (Hirdrasil, _) => (0.0, 5.0),
                (Roshwalr, _) => (1.0, 0.5),
                (Donkey, _) => (4.5, -3.0),
                (Camel, _) => (-0.5, 5.0),
                (Zebra, _) => (3.0, -2.0),
                (Antelope, _) => (1.5, 2.5),
                (Kelpie, _) => (4.0, -1.0),
                (Horse, _) => (0.5, 1.5),
            },
            neck: match (body.species, body.body_type) {
                (Grolgar, _) => (1.0, -1.0),
                (Saber, _) => (-3.5, -2.0),
                (Tuskram, _) => (-1.0, 1.0),
                (Lion, _) => (-1.5, 1.0),
                (Tarasque, _) => (-1.5, -4.0),
                (Tiger, _) => (0.0, 0.0),
                (Wolf, _) => (-4.5, 2.0),
                (Frostfang, _) => (0.5, 1.5),
                (Mouflon, _) => (-1.0, 1.0),
                (Catoblepas, _) => (19.5, -2.0),
                (Bonerattler, _) => (9.0, -0.5),
                (Deer, _) => (-2.5, 1.0),
                (Hirdrasil, _) => (-1.0, 0.5),
                (Roshwalr, _) => (0.0, 1.0),
                (Donkey, _) => (1.0, 3.5),
                (Camel, _) => (3.5, -1.5),
                (Zebra, _) => (1.0, 3.5),
                (Antelope, _) => (0.5, 2.5),
                (Kelpie, _) => (2.0, 1.0),
                (Horse, _) => (1.5, 1.5),
            },
            jaw: match (body.species, body.body_type) {
                (Grolgar, _) => (7.0, 2.0),
                (Saber, _) => (2.5, -2.0),
                (Tuskram, _) => (5.5, -3.5),
                (Lion, _) => (3.5, -4.0),
                (Tarasque, _) => (9.0, -9.5),
                (Tiger, _) => (3.0, -3.5),
                (Wolf, _) => (5.0, -2.5),
                (Frostfang, _) => (4.0, -2.5),
                (Mouflon, _) => (6.0, 1.0),
                (Catoblepas, _) => (1.0, -3.5),
                (Bonerattler, _) => (3.0, -2.5),
                (Deer, _) => (3.5, 2.5),
                (Hirdrasil, _) => (2.5, 3.0),
                (Roshwalr, _) => (4.0, -1.0),
                (Donkey, _) => (1.0, 1.0),
                (Camel, _) => (2.0, 2.5),
                (Zebra, _) => (4.0, 0.0),
                (Antelope, _) => (3.0, 0.5),
                (Kelpie, _) => (1.0, 1.0),
                (Horse, _) => (4.0, 1.0),
            },
            tail: match (body.species, body.body_type) {
                (Grolgar, _) => (-11.5, -0.5),
                (Saber, _) => (-11.0, 0.0),
                (Tuskram, _) => (-9.0, 2.0),
                (Lion, _) => (-11.0, 1.0),
                (Tarasque, _) => (-11.0, 0.0),
                (Tiger, _) => (-13.5, 3.0),
                (Wolf, _) => (-11.0, 0.0),
                (Frostfang, _) => (-7.0, -3.5),
                (Mouflon, _) => (-10.5, 3.0),
                (Catoblepas, _) => (-8.0, -2.0),
                (Bonerattler, _) => (-10.0, 1.5),
                (Deer, _) => (-8.5, 0.5),
                (Hirdrasil, _) => (-11.0, 2.0),
                (Roshwalr, _) => (-8.5, -1.0),
                (Donkey, _) => (-11.0, 1.5),
                (Camel, _) => (-14.0, -1.0),
                (Zebra, _) => (-10.0, 1.5),
                (Antelope, _) => (-10.0, 2.0),
                (Kelpie, _) => (-9.0, 3.0),
                (Horse, _) => (-9.0, 1.5),
            },
            torso_front: match (body.species, body.body_type) {
                (Grolgar, _) => (10.0, 13.0),
                (Saber, _) => (14.0, 13.0),
                (Tuskram, _) => (10.0, 14.5),
                (Lion, _) => (10.0, 12.5),
                (Tarasque, _) => (11.5, 17.5),
                (Tiger, _) => (10.0, 13.0),
                (Wolf, _) => (12.0, 13.0),
                (Frostfang, _) => (9.0, 11.5),
                (Mouflon, _) => (11.0, 14.0),
                (Catoblepas, _) => (7.5, 19.5),
                (Bonerattler, _) => (6.0, 12.5),
                (Deer, _) => (11.0, 13.5),
                (Hirdrasil, _) => (11.0, 14.5),
                (Roshwalr, _) => (6.0, 12.5),
                (Donkey, _) => (10.0, 15.5),
                (Camel, _) => (11.0, 22.5),
                (Zebra, _) => (10.0, 16.5),
                (Antelope, _) => (10.0, 14.0),
                (Kelpie, _) => (10.0, 16.0),
                (Horse, _) => (7.0, 16.0),
            },
            torso_back: match (body.species, body.body_type) {
                (Grolgar, _) => (-10.0, 1.5),
                (Saber, _) => (-13.5, 0.0),
                (Tuskram, _) => (-12.5, -2.0),
                (Lion, _) => (-12.0, -0.5),
                (Tarasque, _) => (-14.0, -1.0),
                (Tiger, _) => (-13.0, -0.5),
                (Wolf, _) => (-12.5, 1.0),
                (Frostfang, _) => (-10.5, 0.0),
                (Mouflon, _) => (-8.5, -0.5),
                (Catoblepas, _) => (-8.5, -4.5),
                (Bonerattler, _) => (-5.0, 0.0),
                (Deer, _) => (-9.0, 0.5),
                (Hirdrasil, _) => (-9.0, -0.5),
                (Roshwalr, _) => (-9.0, -3.5),
                (Donkey, _) => (-6.0, -1.0),
                (Camel, _) => (-12.0, -0.5),
                (Zebra, _) => (-6.0, -1.0),
                (Antelope, _) => (-7.0, 0.0),
                (Kelpie, _) => (-8.0, -1.0),
                (Horse, _) => (-8.0, -1.5),
            },
            ears: match (body.species, body.body_type) {
                (Grolgar, _) => (5.0, 8.0),
                (Saber, _) => (3.0, 5.5),
                (Tuskram, _) => (5.5, 12.0),
                (Lion, _) => (2.0, 3.5),
                (Tarasque, _) => (12.0, -3.0),
                (Tiger, _) => (2.5, 4.0),
                (Wolf, _) => (3.0, 2.5),
                (Frostfang, _) => (2.0, 3.5),
                (Mouflon, _) => (2.5, 5.0),
                (Catoblepas, _) => (11.0, -3.0),
                (Bonerattler, _) => (2.0, 3.5),
                (Deer, _) => (2.5, 5.0),
                (Hirdrasil, _) => (2.5, 5.0),
                (Roshwalr, _) => (5.0, 8.0),
                (Donkey, _) => (-1.0, 8.0),
                (Camel, _) => (2.5, 5.0),
                (Zebra, _) => (0.0, 7.0),
                (Antelope, _) => (2.5, 5.0),
                (Kelpie, _) => (1.0, 7.5),
                (Horse, _) => (1.0, 7.0),
            },
            leg_f: match (body.species, body.body_type) {
                (Grolgar, _) => (7.5, -5.5, -1.0),
                (Saber, _) => (7.0, -4.0, -2.5),
                (Tuskram, _) => (6.0, -6.5, -4.0),
                (Lion, _) => (6.5, -6.5, -1.5),
                (Tarasque, _) => (7.0, -8.0, -6.0),
                (Tiger, _) => (6.0, -6.0, -1.5),
                (Wolf, _) => (4.5, -6.5, -1.5),
                (Frostfang, _) => (5.5, -5.5, -2.0),
                (Mouflon, _) => (4.0, -5.0, -4.0),
                (Catoblepas, _) => (7.0, 2.0, -5.0),
                (Bonerattler, _) => (5.5, 5.0, -4.0),
                (Deer, _) => (3.5, -4.5, -3.5),
                (Hirdrasil, _) => (4.5, -5.0, -2.5),
                (Roshwalr, _) => (8.0, -2.5, -2.5),
                (Donkey, _) => (4.0, -3.5, -4.0),
                (Camel, _) => (4.5, -3.5, -5.5),
                (Zebra, _) => (4.0, -2.5, -4.5),
                (Antelope, _) => (4.0, -4.5, -2.5),
                (Kelpie, _) => (4.5, -3.5, -3.5),
                (Horse, _) => (4.5, -2.5, -3.0),
            },
            leg_b: match (body.species, body.body_type) {
                (Grolgar, _) => (6.0, -6.5, -4.0),
                (Saber, _) => (6.0, -7.0, -3.5),
                (Tuskram, _) => (5.0, -4.5, -2.5),
                (Lion, _) => (6.0, -5.0, -1.5),
                (Tarasque, _) => (6.0, -6.5, -6.5),
                (Tiger, _) => (6.0, -7.0, -1.0),
                (Wolf, _) => (5.0, -6.5, -3.0),
                (Frostfang, _) => (3.5, -4.5, -2.0),
                (Mouflon, _) => (3.5, -8.0, -3.5),
                (Catoblepas, _) => (6.0, -2.5, -2.5),
                (Bonerattler, _) => (6.0, -8.0, -4.0),
                (Deer, _) => (3.0, -6.5, -3.5),
                (Hirdrasil, _) => (4.0, -6.5, -3.0),
                (Roshwalr, _) => (7.0, -7.0, -2.5),
                (Donkey, _) => (4.0, -9.0, -3.0),
                (Camel, _) => (4.5, -10.5, -5.0),
                (Zebra, _) => (3.5, -8.0, -3.5),
                (Antelope, _) => (3.5, -7.5, -3.5),
                (Kelpie, _) => (3.5, -7.0, -2.5),
                (Horse, _) => (3.5, -7.0, -2.0),
            },
            feet_f: match (body.species, body.body_type) {
                (Grolgar, _) => (0.0, 0.0, -4.0),
                (Saber, _) => (1.0, -3.5, -2.5),
                (Tuskram, _) => (0.5, 0.5, -3.0),
                (Lion, _) => (0.5, 0.5, -3.5),
                (Tarasque, _) => (1.0, 0.0, -3.0),
                (Tiger, _) => (0.5, 0.0, -4.5),
                (Wolf, _) => (0.5, 0.0, -2.0),
                (Frostfang, _) => (0.5, 1.5, -3.5),
                (Mouflon, _) => (-0.5, -0.5, -3.0),
                (Catoblepas, _) => (1.0, 0.0, -6.0),
                (Bonerattler, _) => (-0.5, -3.0, -2.5),
                (Deer, _) => (-0.5, -0.5, -2.5),
                (Hirdrasil, _) => (-0.5, -3.0, -3.5),
                (Roshwalr, _) => (0.5, 0.0, -3.0),
                (Donkey, _) => (0.5, 1.0, -3.5),
                (Camel, _) => (0.0, 0.0, -8.0),
                (Zebra, _) => (-0.5, 0.5, -4.0),
                (Antelope, _) => (-0.5, 0.0, -3.5),
                (Kelpie, _) => (-0.5, 0.5, -4.5),
                (Horse, _) => (-0.5, 0.5, -5.0),
            },
            feet_b: match (body.species, body.body_type) {
                (Grolgar, _) => (0.5, -1.5, -3.0),
                (Saber, _) => (1.0, -1.0, -1.0),
                (Tuskram, _) => (0.5, -1.0, -2.5),
                (Lion, _) => (0.5, -1.0, -3.0),
                (Tarasque, _) => (1.5, -1.0, -2.5),
                (Tiger, _) => (0.5, -1.0, -4.0),
                (Wolf, _) => (0.0, -1.0, -1.5),
                (Frostfang, _) => (0.0, -1.5, -3.5),
                (Mouflon, _) => (-1.0, 0.0, -0.5),
                (Catoblepas, _) => (0.5, 0.5, -4.0),
                (Bonerattler, _) => (0.0, 3.0, -2.5),
                (Deer, _) => (-1.0, -0.5, -2.0),
                (Hirdrasil, _) => (-1.0, -2.0, -4.5),
                (Roshwalr, _) => (0.5, -1.0, -3.5),
                (Donkey, _) => (0.5, -1.0, -3.5),
                (Camel, _) => (0.0, 0.5, -9.0),
                (Zebra, _) => (0.5, -1.0, -3.0),
                (Antelope, _) => (-0.5, -1.5, -3.5),
                (Kelpie, _) => (0.5, -0.5, -3.5),
                (Horse, _) => (0.5, -1.5, -3.5),
            },
            scaler: match (body.species, body.body_type) {
                (Grolgar, _) => (1.3),
                (Saber, _) => (1.1),
                (Tuskram, _) => (1.2),
                (Lion, _) => (1.3),
                (Tarasque, _) => (1.3),
                (Tiger, _) => (1.2),
                (Catoblepas, _) => (1.3),
                (Roshwalr, _) => (1.2),
                _ => (1.0),
            },
            startangle: match (body.species, body.body_type) {
                //changes the default angle of front feet
                (Grolgar, _) => (-0.3),
                (Saber, _) => (-0.2),
                (Tuskram, _) => (0.3),
                (Lion, _) => (0.2),
                (Tarasque, _) => (-0.5),
                (Catoblepas, _) => (-0.5),
                (Bonerattler, _) => (-0.7),
                (Roshwalr, _) => (-0.3),
                _ => (0.0),
            },
            tempo: match (body.species, body.body_type) {
                (Grolgar, _) => (0.85),
                (Saber, _) => (1.1),
                (Tuskram, _) => (0.9),
                (Lion, _) => (0.95),
                (Tarasque, _) => (0.95),
                (Wolf, _) => (1.1),
                (Mouflon, _) => (0.85),
                (Catoblepas, _) => (1.1),
                (Deer, _) => (0.85),
                (Hirdrasil, _) => (0.85),
                (Roshwalr, _) => (0.75),
                (Donkey, _) => (0.85),
                (Zebra, _) => (0.85),
                (Kelpie, _) => (0.85),
                (Horse, _) => (0.85),
                _ => (1.0),
            },
            spring: match (body.species, body.body_type) {
                (Grolgar, _) => (0.9),
                (Saber, _) => (0.9),
                (Tuskram, _) => (0.9),
                (Wolf, _) => (1.2),
                (Mouflon, _) => (0.9),
                (Catoblepas, _) => (0.55),
                (Bonerattler, _) => (1.1),
                (Deer, _) => (0.9),
                (Hirdrasil, _) => (1.1),
                (Donkey, _) => (0.85),
                (Camel, _) => (0.85),
                (Zebra, _) => (0.85),
                (Antelope, _) => (1.2),
                (Kelpie, _) => (0.95),
                (Horse, _) => (0.85),
                _ => (1.0),
            },
            feed: match (body.species, body.body_type) {
                (Tuskram, _) => (true, 0.5),
                (Mouflon, _) => (true, 1.0),
                (Deer, _) => (true, 1.0),
                (Hirdrasil, _) => (true, 0.9),
                (Donkey, _) => (false, 1.0),
                (Zebra, _) => (false, 1.0),
                (Antelope, _) => (false, 0.9),
                (Kelpie, _) => (false, 1.0),
                (Horse, _) => (true, 0.85),
                _ => (false, 0.0),
            },
        }
    }
}
