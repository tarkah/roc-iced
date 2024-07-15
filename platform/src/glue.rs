use core::ffi::c_void;
use core::fmt;
use core::mem::ManuallyDrop;

use roc_std::{RocBox, RocList, RocStr};

#[derive(Debug)]
#[repr(C)]
pub struct Button {
    pub content: Element,
    pub on_press: Action<RocBox<c_void>>,
    pub style: ButtonStyleFn,
    pub height: Optional<Length>,
    pub padding: Padding,
    pub width: Optional<Length>,
    pub clip: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct ButtonStyle {
    pub background: Optional<Color>,
    pub border: Border,
    pub text_color: Color,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ButtonStatus {
    Active,
    Disabled,
    Hovered,
    Pressed,
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ButtonStyleFn {
    closure_data: RocBox<c_void>,
}

impl ButtonStyleFn {
    pub fn force_thunk(&self, arg0: ButtonStatus) -> Optional<ButtonStyle> {
        extern "C" {
            fn roc__mainForHost_5_caller(
                arg0: *const ButtonStatus,
                closure_data: *const c_void,
                output: *mut Optional<ButtonStyle>,
            );
        }

        let mut output = core::mem::MaybeUninit::uninit();

        unsafe {
            roc__mainForHost_5_caller(&arg0, &*self.closure_data, output.as_mut_ptr());

            output.assume_init()
        }
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct CheckboxOnToggle {
    closure_data: RocBox<c_void>,
}

impl CheckboxOnToggle {
    pub fn force_thunk(&self, arg0: bool) -> RocBox<c_void> {
        extern "C" {
            fn roc__mainForHost_4_caller(
                arg0: *const bool,
                closure_data: *const c_void,
                output: *mut RocBox<c_void>,
            );
        }

        let mut output = core::mem::MaybeUninit::uninit();

        unsafe {
            roc__mainForHost_4_caller(&arg0, &*self.closure_data, output.as_mut_ptr());

            output.assume_init()
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Checkbox {
    pub label: RocStr,
    pub on_toggle: Action<CheckboxOnToggle>,
    pub is_checked: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct Column {
    pub children: RocList<Element>,
    pub height: Length,
    pub max_width: f32,
    pub padding: Padding,
    pub spacing: f32,
    pub width: Length,
    pub align_items: Alignment,
    pub clip: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct Row {
    pub children: RocList<Element>,
    pub height: Length,
    pub padding: Padding,
    pub spacing: f32,
    pub width: Length,
    pub align_items: Alignment,
    pub clip: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum LengthTag {
    Fill = 0,
    FillPortion = 1,
    Fixed = 2,
    Shrink = 3,
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub union LengthPayload {
    fill: (),
    fill_portion: u16,
    fixed: f32,
    shrink: (),
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Length {
    pub payload: LengthPayload,
    pub tag: LengthTag,
}

impl fmt::Debug for Length {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use LengthTag::*;

        unsafe {
            match self.tag {
                Fill => f
                    .debug_tuple("Length::Fill")
                    .field(&self.payload.fill)
                    .finish(),
                FillPortion => f
                    .debug_tuple("Length::FillPortion")
                    .field(&self.payload.fill_portion)
                    .finish(),
                Fixed => f
                    .debug_tuple("Length::Fixed")
                    .field(&self.payload.fixed)
                    .finish(),
                Shrink => f
                    .debug_tuple("Length::Shrink")
                    .field(&self.payload.shrink)
                    .finish(),
            }
        }
    }
}

impl Length {
    pub fn fill_portion(self) -> u16 {
        debug_assert_eq!(self.tag, LengthTag::FillPortion);
        unsafe { self.payload.fill_portion }
    }

    pub fn fixed(self) -> f32 {
        debug_assert_eq!(self.tag, LengthTag::Fixed);
        unsafe { self.payload.fixed }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Container {
    pub content: Element,
    pub height: Optional<Length>,
    pub max_height: f32,
    pub max_width: f32,
    pub padding: Padding,
    pub style: ContainerStyle,
    pub width: Optional<Length>,
    pub clip: bool,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
}

#[derive(Debug)]
#[repr(C)]
pub struct ContainerStyle {
    pub background: Optional<Color>,
    pub border: Border,
    pub text_color: Optional<Color>,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Text {
    pub value: RocStr,
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct TextInputOnInput {
    closure_data: RocBox<c_void>,
}

impl TextInputOnInput {
    pub fn force_thunk(&self, arg0: &str) -> RocBox<c_void> {
        extern "C" {
            fn roc__mainForHost_3_caller(
                arg0: *const RocStr,
                closure_data: *const c_void,
                output: *mut RocBox<c_void>,
            );
        }

        let mut output = core::mem::MaybeUninit::uninit();

        // Roc will decrement
        let arg = ManuallyDrop::new(RocStr::from(arg0));

        unsafe {
            roc__mainForHost_3_caller(&*arg, &*self.closure_data, output.as_mut_ptr());

            output.assume_init()
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct TextInput {
    pub on_input: Action<TextInputOnInput>,
    pub on_submit: Action<RocBox<c_void>>,
    pub value: RocStr,
    pub width: Length,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum ElementTag {
    Button = 0,
    Checkbox = 1,
    Column = 2,
    Container = 3,
    Row = 4,
    Text = 5,
    TextInput = 6,
}

#[repr(transparent)]
pub struct Element(*mut ElementPayload);

impl Element {
    pub fn tag(&self) -> ElementTag {
        let discriminants = {
            use ElementTag::*;

            [Button, Checkbox, Column, Container, Row, Text, TextInput]
        };

        if self.0.is_null() {
            unreachable!("this pointer cannot be NULL")
        } else {
            match std::mem::size_of::<usize>() {
                4 => discriminants[self.0 as usize & 0b011],
                8 => discriminants[self.0 as usize & 0b111],
                _ => unreachable!(),
            }
        }
    }

    fn unmasked_pointer(&self) -> *mut ElementPayload {
        debug_assert!(!self.0.is_null());

        let mask = match std::mem::size_of::<usize>() {
            4 => !0b011usize,
            8 => !0b111usize,
            _ => unreachable!(),
        };

        ((self.0 as usize) & mask) as *mut ElementPayload
    }

    pub fn button(&self) -> &Button {
        unsafe { &*self.unmasked_pointer().cast() }
    }

    pub fn checkbox(&self) -> &Checkbox {
        unsafe { &*self.unmasked_pointer().cast() }
    }

    pub fn column(&self) -> &Column {
        unsafe { &*self.unmasked_pointer().cast() }
    }

    pub fn container(&self) -> &Container {
        unsafe { &*self.unmasked_pointer().cast() }
    }

    pub fn row(&self) -> &Row {
        unsafe { &*self.unmasked_pointer().cast() }
    }

    pub fn text(&self) -> &Text {
        unsafe { &*self.unmasked_pointer().cast() }
    }

    pub fn text_input(&self) -> &TextInput {
        unsafe { &*self.unmasked_pointer().cast() }
    }
}

impl Drop for Element {
    fn drop(&mut self) {
        use ElementTag::*;

        unsafe {
            let payload = &mut *self.unmasked_pointer();

            match self.tag() {
                Button => ManuallyDrop::drop(&mut payload.button),
                Checkbox => ManuallyDrop::drop(&mut payload.checkbox),
                Column => ManuallyDrop::drop(&mut payload.column),
                Container => ManuallyDrop::drop(&mut payload.container),
                Row => ManuallyDrop::drop(&mut payload.row),
                Text => ManuallyDrop::drop(&mut payload.text),
                TextInput => ManuallyDrop::drop(&mut payload.text_input),
            }
        }
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ElementTag::*;

        match self.tag() {
            Button => f
                .debug_tuple("Element::Button")
                .field(self.button())
                .finish(),
            Checkbox => f
                .debug_tuple("Element::Checkbox")
                .field(self.checkbox())
                .finish(),
            Column => f
                .debug_tuple("Element::Column")
                .field(self.column())
                .finish(),
            Container => f
                .debug_tuple("Element::Container")
                .field(self.container())
                .finish(),
            Row => f.debug_tuple("Element::Row").field(self.row()).finish(),
            Text => f.debug_tuple("Element::Text").field(self.text()).finish(),
            TextInput => f
                .debug_tuple("Element::TextInput")
                .field(self.text_input())
                .finish(),
        }
    }
}

#[repr(C)]
union ElementPayload {
    button: ManuallyDrop<Button>,
    checkbox: ManuallyDrop<Checkbox>,
    column: ManuallyDrop<Column>,
    container: ManuallyDrop<Container>,
    row: ManuallyDrop<Row>,
    text: ManuallyDrop<Text>,
    text_input: ManuallyDrop<TextInput>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum ActionTag {
    Active = 0,
    Disabled = 1,
}

#[repr(C)]
pub union ActionPayload<T> {
    active: ManuallyDrop<T>,
    disabled: (),
}

#[repr(C)]
pub struct Action<T> {
    payload: ActionPayload<T>,
    tag: ActionTag,
}

impl<T> Action<T> {
    pub fn active(&self) -> Option<&T> {
        if matches!(self.tag, ActionTag::Active) {
            unsafe { Some(&self.payload.active) }
        } else {
            None
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Action<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ActionTag::*;

        unsafe {
            match self.tag {
                Active => f
                    .debug_tuple("Action::Active")
                    .field(&self.payload.active)
                    .finish(),
                Disabled => f
                    .debug_tuple("Action::Disabled")
                    .field(&self.payload.disabled)
                    .finish(),
            }
        }
    }
}

impl<T> Drop for Action<T> {
    fn drop(&mut self) {
        use ActionTag::*;

        unsafe {
            match self.tag {
                Active => ManuallyDrop::drop(&mut self.payload.active),
                Disabled => {}
            }
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Color {
    pub a: f32,
    pub b: f32,
    pub g: f32,
    pub r: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum OptionalTag {
    None = 0,
    Some = 1,
}

#[repr(C)]
pub union OptionalPayload<T> {
    none: (),
    some: ManuallyDrop<T>,
}

#[repr(C)]
pub struct Optional<T> {
    pub payload: OptionalPayload<T>,
    pub tag: OptionalTag,
}

impl<T: fmt::Debug> fmt::Debug for Optional<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use OptionalTag::*;

        unsafe {
            match self.tag {
                None => "None".fmt(f),
                Some => f.debug_tuple("Some").field(&self.payload.some).finish(),
            }
        }
    }
}

impl<T> Drop for Optional<T> {
    fn drop(&mut self) {
        use OptionalTag::*;

        unsafe {
            match self.tag {
                None => {}
                Some => ManuallyDrop::drop(&mut self.payload.some),
            }
        }
    }
}

impl<T> Optional<T> {
    pub fn as_option(&self) -> Option<&T> {
        match self.tag {
            OptionalTag::None => None,
            OptionalTag::Some => unsafe { Some(&*self.payload.some) },
        }
    }

    pub fn into_option(mut self) -> Option<T> {
        match self.tag {
            OptionalTag::None => None,
            OptionalTag::Some => unsafe { Some(ManuallyDrop::take(&mut self.payload.some)) },
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Border {
    pub color: Color,
    pub radius: f32,
    pub width: f32,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Padding {
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
    pub top: f32,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Size {
    pub height: f32,
    pub width: f32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Settings {
    pub default_text_size: f32,
    pub window: WindowSettings,
    pub antialiasing: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct WindowSettings {
    pub max_size: Optional<Size>,
    pub min_size: Optional<Size>,
    pub size: Size,
    pub decorations: bool,
    pub resizable: bool,
    pub transparent: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct Init {
    pub model: RocBox<c_void>,
    pub settings: Settings,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Alignment {
    Center,
    End,
    Start,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum VerticalAlignment {
    Bottom,
    Center,
    Top,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum HorizontalAlignment {
    Center,
    Left,
    Right,
}
