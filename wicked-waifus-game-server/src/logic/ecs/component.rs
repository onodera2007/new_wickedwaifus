use wicked_waifus_protocol::EntityPb;

use crate::logic::components::*;

macro_rules! impl_component_container {
    ($($comp:ident;)*) => {
        pub enum ComponentContainer {
        $(
            $comp($comp),
        )*
        }

        impl ComponentContainer {
            pub fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
                match self {
                $(
                    Self::$comp(comp) => comp.set_pb_data(pb),
                )*
                }
            }
        }
    };
}

impl_component_container! {
    Attribute;
    Autonomous;
    CharacterAttach;
    Concomitant;
    EntityConfig;
    Equip;
    FightBuff;
    Fsm;
    Interact;
    LogicState;
    MonsterAi;
    Movement;
    OwnerPlayer;
    ParaglidingSkin;
    PlayerOwnedEntityMarker;
    Position;
    RoleSkin;
    SoarWingSkin;
    StateTag;
    Summoner;
    Tag;
    Visibility;
    VisionSkill;
    WeaponSkin;
}

pub trait Component {
    fn set_pb_data(&self, pb: &mut EntityPb);
}
