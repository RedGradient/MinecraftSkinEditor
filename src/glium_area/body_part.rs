#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum BodyPart {
    Head,
    Torso,
    RightArm,
    LeftArm,
    RightLeg,
    LeftLeg,
    HeadOuter,
    TorsoOuter,
    RightArmOuter,
    LeftArmOuter,
    RightLegOuter,
    LeftLegOuter,
}

impl BodyPart {
    pub fn is_outer(&self) -> bool {
        matches!(
            self,
            Self::HeadOuter
                | Self::TorsoOuter
                | Self::RightArmOuter
                | Self::LeftArmOuter
                | Self::RightLegOuter
                | Self::LeftLegOuter
        )
    }

    pub fn outer_counterpart(self) -> Option<Self> {
        match self {
            Self::Head => Some(Self::HeadOuter),
            Self::Torso => Some(Self::TorsoOuter),
            Self::RightArm => Some(Self::RightArmOuter),
            Self::LeftArm => Some(Self::LeftArmOuter),
            Self::RightLeg => Some(Self::RightLegOuter),
            Self::LeftLeg => Some(Self::LeftLegOuter),
            _ => None,
        }
    }

    pub fn inner_counterpart(self) -> Option<Self> {
        match self {
            Self::HeadOuter => Some(Self::Head),
            Self::TorsoOuter => Some(Self::Torso),
            Self::RightArmOuter => Some(Self::RightArm),
            Self::LeftArmOuter => Some(Self::LeftArm),
            Self::RightLegOuter => Some(Self::RightLeg),
            Self::LeftLegOuter => Some(Self::LeftLeg),
            _ => None,
        }
    }
}
