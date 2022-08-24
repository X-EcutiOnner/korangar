use std::cmp::PartialOrd;

use cgmath::Vector2;
use derive_new::new;
use num::traits::NumOps;
use num::{NumCast, Zero};
use procedural::*;

use crate::interface::{ChangeEvent, ElementCell, FramedWindow, InterfaceSettings, PrototypeWindow, Size, Window, WindowCache, *};

#[derive(new)]
pub struct Vector2Window<T> {
    name: String,
    inner_pointer: *const Vector2<T>,
    minimum_value: Vector2<T>,
    maximum_value: Vector2<T>,
    change_event: Option<ChangeEvent>,
}

impl<T: Zero + NumOps + NumCast + Copy + PartialOrd + 'static> PrototypeWindow for Vector2Window<T> {

    fn to_window(
        &self,
        window_cache: &WindowCache,
        interface_settings: &InterfaceSettings,
        avalible_space: Size,
    ) -> Box<dyn Window + 'static> {

        let elements: Vec<ElementCell> = vec![
            cell!(Headline::new("x".to_string(), Headline::DEFAULT_SIZE)),
            cell!(Slider::new(
                unsafe { &(*self.inner_pointer).x as *const T },
                self.minimum_value.x,
                self.maximum_value.x,
                self.change_event
            )),
            cell!(Headline::new("y".to_string(), Headline::DEFAULT_SIZE)),
            cell!(Slider::new(
                unsafe { &(*self.inner_pointer).y as *const T },
                self.minimum_value.y,
                self.maximum_value.y,
                self.change_event
            )),
        ];

        Box::new(FramedWindow::new(
            window_cache,
            interface_settings,
            avalible_space,
            self.name.clone(),
            None,
            elements,
            constraint!(200 > 250 < 300, ?),
        ))
    }
}
