use eframe::{
    egui::Key,
    epaint::{Color32, Rgba},
};
use serde::{Deserialize, Serialize};

use crate::widgets::class_selector::EnumIter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Classes {
    Ball,
    Robot,
    GoalPost,
    PenaltySpot,
}

impl EnumIter for Classes {
    fn list() -> Vec<Self> {
        use Classes::*;
        vec![Ball, Robot, GoalPost, PenaltySpot]
    }
}

impl From<usize> for Classes {
    fn from(value: usize) -> Self {
        match value {
            0 => Classes::Ball,
            1 => Classes::Robot,
            2 => Classes::GoalPost,
            3 => Classes::PenaltySpot,
            _ => panic!("{value} is not a valid class"),
        }
    }
}
impl From<&Classes> for usize {
    fn from(value: &Classes) -> Self {
        match value {
            Classes::Ball => 0,
            Classes::Robot => 1,
            Classes::GoalPost => 2,
            Classes::PenaltySpot => 3,
        }
    }
}

impl Classes {
    pub fn from_key(key: Key) -> Option<Classes> {
        match key {
            Key::Num1 => Some(Classes::Ball),
            Key::Num2 => Some(Classes::Robot),
            Key::Num3 => Some(Classes::GoalPost),
            Key::Num4 => Some(Classes::PenaltySpot),
            _ => None,
        }
    }

    pub fn color(&self) -> Color32 {
        let color = match self {
            Classes::Robot => Color32::BLUE,
            Classes::Ball => Color32::LIGHT_RED,
            Classes::GoalPost => Color32::DARK_RED,
            Classes::PenaltySpot => Color32::GOLD,
        };
        let [r, g, b, _] = color.to_normalized_gamma_f32();
        Rgba::from_rgba_unmultiplied(r, g, b, 0.05).into()
    }

    pub fn previous(class: &Classes) -> Classes {
        let classes = Self::list();
        let index = classes
            .iter()
            .position(|&candidate| candidate == *class)
            .unwrap_or_else(|| panic!("{:?} not found in Classes list", class));

        classes[(index + classes.len() - 1) % classes.len()]
    }

    pub fn next(class: &Classes) -> Classes {
        let classes = Self::list();
        let index = classes
            .iter()
            .position(|&candidate| candidate == *class)
            .unwrap_or_else(|| panic!("{:?} not found in Classes list", class));

        classes[(index + 1) % classes.len()]
    }
}
