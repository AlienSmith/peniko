// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{color::GlowColor, image::PBRImages};

use super::{Color, Gradient, Image, ProcedureImage};

/// Describes the color content of a filled or stroked shape.
///
/// See also [`BrushRef`] which can be used to avoid allocations.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Brush {
    SolidGlow(GlowColor),
    /// Solid color brush.
    Solid(Color),
    /// Gradient brush.
    Gradient(Gradient),
    /// Image brush.
    Image(Image),
    ///Procedure Image brush
    ProcedureImage(ProcedureImage),
    /// PbrImages,
    PBRImage(PBRImages),
}

impl From<GlowColor> for Brush {
    fn from(g: GlowColor) -> Self{
        Self::SolidGlow(g)
    }
}

impl From<Color> for Brush {
    fn from(c: Color) -> Self {
        Self::Solid(c)
    }
}

impl From<Gradient> for Brush {
    fn from(g: Gradient) -> Self {
        Self::Gradient(g)
    }
}

impl From<ProcedureImage> for Brush {
    fn from(value: ProcedureImage) -> Self {
        Self::ProcedureImage(value)
    }
}

impl From<PBRImages> for Brush {
    fn from(images: PBRImages) -> Self {
        Self::PBRImage(images)
    }
}

impl Default for Brush {
    fn default() -> Self {
        Self::Solid(Color::default())
    }
}

/// Reference to a [brush](Brush).
///
/// This is useful for methods that would like to accept brushes by reference. Defining
/// the type as `impl<Into<BrushRef>>` allows accepting types like `&LinearGradient`
/// directly without cloning or allocating.
#[derive(Clone, PartialEq, Debug)]
pub enum BrushRef<'a> {
    /// Solid color brush.
    Solid(Color),
    /// Solid color exceeding 1.
    SolidGlow(GlowColor),
    /// Gradient brush.
    Gradient(Gradient),
    /// Image brush.
    Image(&'a Image),
    /// Procedure Image
    ProcedureImage(ProcedureImage),
    /// PbrImages,
    PBRImage(&'a PBRImages),
}

impl<'a> BrushRef<'a> {
    /// Converts the reference to an owned brush.
    #[must_use]
    pub fn to_owned(&self) -> Brush {
        match self {
            Self::Solid(color) => Brush::Solid(*color),
            Self::SolidGlow(glow_color) => Brush::SolidGlow(*glow_color),
            Self::Gradient(gradient) => Brush::Gradient((*gradient).clone()),
            Self::Image(image) => Brush::Image((*image).clone()),
            Self::ProcedureImage(value) => Brush::ProcedureImage(*value),
            Self::PBRImage(value) => Brush::PBRImage((*value).clone()),
        }
    }
}

impl From<Color> for BrushRef<'_> {
    fn from(color: Color) -> Self {
        Self::Solid(color)
    }
}

impl From<GlowColor> for BrushRef<'_> {
    fn from(glow_color: GlowColor) -> Self{
        Self::SolidGlow(glow_color)
    }
}

impl<'a> From<&'a Color> for BrushRef<'_> {
    fn from(color: &'a Color) -> Self {
        Self::Solid(*color)
    }
}

impl<'a> From<&'a Gradient> for BrushRef<'_> {
    fn from(gradient: &'a Gradient) -> Self {
        Self::Gradient((*gradient).clone())
    }
}

impl<'a> From<&'a Image> for BrushRef<'a> {
    fn from(image: &'a Image) -> Self {
        Self::Image(image)
    }
}

impl<'a> From<&'a PBRImages> for BrushRef<'a> {
    fn from(images: &'a PBRImages) -> Self {
        Self::PBRImage(images)
    }
}

impl From<ProcedureImage> for BrushRef<'_> {
    fn from(value: ProcedureImage) -> Self {
        Self::ProcedureImage(value)
    }
}

impl<'a> From<&'a ProcedureImage> for BrushRef<'_> {
    fn from(value: &'a ProcedureImage) -> Self {
        Self::ProcedureImage(*value)
    }
}

impl<'a> From<&'a Brush> for BrushRef<'a> {
    fn from(brush: &'a Brush) -> Self {
        match brush {
            Brush::Solid(color) => Self::Solid(*color),
            Brush::SolidGlow(glow_color) => Self::SolidGlow(*glow_color),
            Brush::Gradient(gradient) => Self::Gradient((*gradient).clone()),
            Brush::Image(image) => Self::Image(image),
            Brush::ProcedureImage(value) => Self::ProcedureImage(*value),
            Brush::PBRImage(pbrimages) => Self::PBRImage(pbrimages),
        }
    }
}

/// Defines how a brush is extended when the content does not
/// fill a shape.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Extend {
    /// Extends the image by repeating the edge color of the brush.
    #[default]
    Pad,
    /// Extends the image by repeating the brush.
    Repeat,
    /// Extends the image by reflecting the brush.
    Reflect,
}
