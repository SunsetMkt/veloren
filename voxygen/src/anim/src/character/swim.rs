use super::{super::Animation, CharacterSkeleton, SkeletonAttr};
use common::comp::item::{Hands, ToolKind};
use std::{f32::consts::PI, ops::Mul};
use vek::*;

pub struct SwimAnimation;

type SwimAnimationDependency = (
    Option<ToolKind>,
    Option<ToolKind>,
    Vec3<f32>,
    Vec3<f32>,
    Vec3<f32>,
    f64,
    Vec3<f32>,
);

impl Animation for SwimAnimation {
    type Dependency = SwimAnimationDependency;
    type Skeleton = CharacterSkeleton;

    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"character_swim\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "character_swim")]

    fn update_skeleton_inner(
        skeleton: &Self::Skeleton,
        (active_tool_kind, second_tool_kind, velocity, orientation, last_ori, global_time, avg_vel): Self::Dependency,
        anim_time: f64,
        rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        let speed = Vec3::<f32>::from(velocity).magnitude();
        *rate = 1.0;
        let tempo = if speed > 0.5 { 1.0 } else { 0.7 };
        let intensity = if speed > 0.5 { 1.0 } else { 0.3 };

        let lab = 1.0 * tempo;

        let short = (anim_time as f32 * lab as f32 * 6.0).sin();

        let shortalt = (anim_time as f32 * lab as f32 * 6.0 + PI / 2.0).sin();

        let foot = (anim_time as f32 * lab as f32 * 6.0).sin();

        let wave_stop = (anim_time as f32 * 9.0).min(PI / 2.0 / 2.0).sin();

        let footrotl = (((1.0)
            / (0.2
                + (0.8)
                    * ((anim_time as f32 * 6.0 * lab as f32 + PI * 1.4).sin()).powf(2.0 as f32)))
        .sqrt())
            * ((anim_time as f32 * 6.0 * lab as f32 + PI * 1.4).sin());

        let footrotr = (((1.0)
            / (0.2
                + (0.8)
                    * ((anim_time as f32 * 6.0 * lab as f32 + PI * 0.4).sin()).powf(2.0 as f32)))
        .sqrt())
            * ((anim_time as f32 * 6.0 * lab as f32 + PI * 0.4).sin());

        let foothoril = (anim_time as f32 * 6.0 * lab as f32 + PI * 1.45).sin();
        let foothorir = (anim_time as f32 * 6.0 * lab as f32 + PI * (0.45)).sin();
        let head_look = Vec2::new(
            ((global_time + anim_time) as f32 / 4.0 * (1.0 / tempo))
                .floor()
                .mul(7331.0)
                .sin()
                * 0.2,
            ((global_time + anim_time) as f32 / 4.0 * (1.0 / tempo))
                .floor()
                .mul(1337.0)
                .sin()
                * 0.1,
        );
        let ori: Vec2<f32> = Vec2::from(orientation);
        let last_ori = Vec2::from(last_ori);
        let tilt = if Vec2::new(ori, last_ori)
            .map(|o| o.magnitude_squared())
            .map(|m| m > 0.001 && m.is_finite())
            .reduce_and()
            && ori.angle_between(last_ori).is_finite()
        {
            ori.angle_between(last_ori).min(0.2)
                * last_ori.determine_side(Vec2::zero(), ori).signum()
        } else {
            0.0
        } * 1.3;

        let adjust = if speed > 0.5 { -1.57 } else { -3.14 * speed };
        next.head.offset = Vec3::new(
            0.0,
            -3.0 + skeleton_attr.head.0,
            skeleton_attr.head.1 - 1.0 + short * 0.3,
        );
        next.head.ori = Quaternion::rotation_z(head_look.x + short * -0.6 * intensity)
            * Quaternion::rotation_x(
                (0.6 * head_look.y * (1.0 / intensity)).abs()
                    + 0.45 * intensity
                    + velocity.z * 0.02,
            );
        next.head.scale = Vec3::one() * skeleton_attr.head_scale;

        next.chest.offset = Vec3::new(
            0.0,
            skeleton_attr.chest.0,
            -13.0 + skeleton_attr.chest.1 + short * 1.3 * intensity,
        );
        next.chest.ori = Quaternion::rotation_z(short * 0.4 * intensity);
        next.chest.scale = Vec3::one();

        next.belt.offset = Vec3::new(0.0, skeleton_attr.belt.0, skeleton_attr.belt.1);
        next.belt.ori = Quaternion::rotation_x(velocity.z * 0.01)
            * Quaternion::rotation_z(short * 0.2 * intensity);
        next.belt.scale = Vec3::one();

        next.back.offset = Vec3::new(0.0, skeleton_attr.back.0, skeleton_attr.back.1);
        next.back.ori = Quaternion::rotation_z(0.0);
        next.back.scale = Vec3::one() * 1.02;

        next.shorts.offset = Vec3::new(0.0, skeleton_attr.shorts.0, skeleton_attr.shorts.1);
        next.shorts.ori = Quaternion::rotation_x(velocity.z * 0.02)
            * Quaternion::rotation_z(short * 0.3 * intensity);
        next.shorts.scale = Vec3::one();

        next.l_hand.offset = Vec3::new(
            -skeleton_attr.hand.0,
            1.5 + skeleton_attr.hand.1 - foot * 1.2 * intensity,
            2.0 + skeleton_attr.hand.2 + foot * -3.0 * intensity,
        );
        next.l_hand.ori =
            Quaternion::rotation_x(0.8 + foot * -0.6 * intensity) * Quaternion::rotation_y(0.2);
        next.l_hand.scale = Vec3::one();

        next.r_hand.offset = Vec3::new(
            skeleton_attr.hand.0,
            1.5 + skeleton_attr.hand.1 + foot * 1.2 * intensity,
            2.0 + skeleton_attr.hand.2 + foot * 3.0 * intensity,
        );
        next.r_hand.ori =
            Quaternion::rotation_x(0.8 + foot * 0.6 * intensity) * Quaternion::rotation_y(-0.2);
        next.r_hand.scale = Vec3::one();

        next.l_foot.offset = Vec3::new(
            -skeleton_attr.foot.0,
            skeleton_attr.foot.1 + foothorir * 1.5 * intensity,
            -15.0 + skeleton_attr.foot.2 + footrotl * 3.0 * intensity,
        );
        next.l_foot.ori = Quaternion::rotation_x(-0.8 + footrotl * 0.4 * intensity);
        next.l_foot.scale = Vec3::one();

        next.r_foot.offset = Vec3::new(
            skeleton_attr.foot.0,
            skeleton_attr.foot.1 + foothorir * 1.5 * intensity,
            -15.0 + skeleton_attr.foot.2 + footrotr * 3.0 * intensity,
        );
        next.r_foot.ori = Quaternion::rotation_x(-0.8 + footrotr * 0.4 * intensity);
        next.r_foot.scale = Vec3::one();

        next.l_shoulder.offset = Vec3::new(
            -skeleton_attr.shoulder.0,
            skeleton_attr.shoulder.1,
            skeleton_attr.shoulder.2,
        );
        next.l_shoulder.ori = Quaternion::rotation_x(short * 0.15 * intensity);
        next.l_shoulder.scale = Vec3::one() * 1.1;

        next.r_shoulder.offset = Vec3::new(
            skeleton_attr.shoulder.0,
            skeleton_attr.shoulder.1,
            skeleton_attr.shoulder.2,
        );
        next.r_shoulder.ori = Quaternion::rotation_x(short * -0.15 * intensity);
        next.r_shoulder.scale = Vec3::one() * 1.1;

        next.glider.offset = Vec3::new(0.0, 0.0, 10.0);
        next.glider.scale = Vec3::one() * 0.0;

        match active_tool_kind {
            Some(ToolKind::Dagger(_)) => {
                next.main.offset = Vec3::new(-4.0, -5.0, 7.0);
                next.main.ori =
                    Quaternion::rotation_y(0.25 * PI) * Quaternion::rotation_z(1.5 * PI);
            },
            Some(ToolKind::Shield(_)) => {
                next.main.offset = Vec3::new(-0.0, -5.0, 3.0);
                next.main.ori =
                    Quaternion::rotation_y(0.25 * PI) * Quaternion::rotation_z(-1.5 * PI);
            },
            _ => {
                next.main.offset = Vec3::new(-7.0, -5.0, 15.0);
                next.main.ori = Quaternion::rotation_y(2.5) * Quaternion::rotation_z(1.57);
            },
        }
        next.main.scale = Vec3::one();

        match second_tool_kind {
            Some(ToolKind::Dagger(_)) => {
                next.second.offset = Vec3::new(4.0, -6.0, 7.0);
                next.second.ori =
                    Quaternion::rotation_y(-0.25 * PI) * Quaternion::rotation_z(-1.5 * PI);
            },
            Some(ToolKind::Shield(_)) => {
                next.second.offset = Vec3::new(0.0, -4.0, 3.0);
                next.second.ori =
                    Quaternion::rotation_y(-0.25 * PI) * Quaternion::rotation_z(1.5 * PI);
            },
            _ => {
                next.second.offset = Vec3::new(-7.0, -5.0, 15.0);
                next.second.ori = Quaternion::rotation_y(2.5) * Quaternion::rotation_z(1.57);
            },
        }
        next.second.scale = Vec3::one();

        next.lantern.offset = Vec3::new(
            skeleton_attr.lantern.0,
            skeleton_attr.lantern.1,
            skeleton_attr.lantern.2,
        );
        next.lantern.ori = Quaternion::rotation_x(0.0) * Quaternion::rotation_y(0.0);
        next.lantern.scale = Vec3::one() * 0.65;

        next.torso.offset = Vec3::new(0.0, 0.0, 1.0) * skeleton_attr.scaler;
        next.torso.ori = Quaternion::rotation_x(adjust + avg_vel.z * 0.12)
            * Quaternion::rotation_z(tilt * 12.0 + short * 0.4 * intensity);
        next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;

        next.control.scale = Vec3::one();

        next.l_control.scale = Vec3::one();

        next.r_control.scale = Vec3::one();

        next.second.scale = match (
            active_tool_kind.map(|tk| tk.hands()),
            second_tool_kind.map(|tk| tk.hands()),
        ) {
            (Some(Hands::OneHand), Some(Hands::OneHand)) => Vec3::one(),
            (_, _) => Vec3::zero(),
        };

        next
    }
}
