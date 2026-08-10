// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{Blob, Extend};

/// Defines the pixel format of an [image](Image).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Format {
    /// 32-bit RGBA with 8-bit channels.
    Rgba8,
}

impl Format {
    /// Returns the required size in bytes for an image in this format
    /// of the given dimensions.
    ///
    /// A result of `None` indicates an overflow in the size calculation.
    #[must_use]
    pub fn size_in_bytes(self, width: u32, height: u32) -> Option<usize> {
        match self {
            Self::Rgba8 => 4usize
                .checked_mul(width as usize)
                .and_then(|x| x.checked_mul(height as usize)),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImageUsageType {
    TRANSPARENT,
    MASKED,
    NORMAL,
}

#[cfg(feature = "serde")]
use once_cell::sync::Lazy;
#[cfg(feature = "serde")]
use std::sync::atomic::{AtomicU32, Ordering};
#[cfg(feature = "serde")]
use std::sync::Arc;
#[cfg(feature = "serde")]
static IMAGE_TRACE_ID_COUNTER: Lazy<Arc<AtomicU32>> = Lazy::new(|| Arc::new(AtomicU32::new(0)));

/// Owned shareable image resource.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Image {
    /// Blob containing the image data.
    pub data: Option<Blob<u8>>,
    /// Pixel format of the image.
    pub format: Format,
    /// Width of the image.
    pub width: u32,
    /// Height of the image.
    pub height: u32,
    /// Extend mode
    pub extend: Extend,
    /// Usage of this image.
    pub usage: ImageUsageType,
    /// Sprite sheet configs.
    pub sprite_sheet: Option<SpriteSheet>,
    /// Mip flags: 0 = full mipmap chain, 1 = mip 0 only (no mipmap).
    pub mip_flags: u32,
    #[cfg(feature = "serde")]
    pub trace_id: u32,
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpriteSheet {
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpriteSheetPlayConfig {
    pub play_in_loop: bool,
    pub fps: f32,
    pub start_time: f32,
}

impl Image {
    /// Creates a new image with the given data, [format](Format) and dimensions.
    #[must_use]
    pub fn new(data: Blob<u8>, format: Format, width: u32, height: u32) -> Self {
        Self {
            data: Some(data),
            format,
            width,
            height,
            extend: Extend::Pad,
            usage: ImageUsageType::TRANSPARENT,
            sprite_sheet: None,
            mip_flags: 0,
            #[cfg(feature = "serde")]
            trace_id: IMAGE_TRACE_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
        }
    }

    /// Builder method for setting the image [extend mode](Extend).
    #[must_use]
    pub fn with_extend(mut self, mode: Extend) -> Self {
        self.extend = mode;
        self
    }

    #[must_use]
    pub fn with_usage(mut self, usage: ImageUsageType) -> Self {
        self.usage = usage;
        self
    }

    #[must_use]
    pub fn with_sprite_sheet(mut self, sprite_sheet: Option<SpriteSheet>) -> Self {
        self.sprite_sheet = sprite_sheet;
        self
    }
}

#[cfg(feature = "serde")]
static PBR_IMAGE_TRACE_ID_COUNTER: Lazy<Arc<AtomicU32>> = Lazy::new(|| Arc::new(AtomicU32::new(0)));

/// Definition of a gradient that transitions between two or more colors.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PBRImages {
    pub albedo: Image,
    pub normals: Image,
    pub metallic: f32,
    pub roughness: f32,
    #[cfg(feature = "serde")]
    pub trace_id: u32,
}

impl PBRImages {
    #[must_use]
    pub fn new(albedo: Image, normals: Image, metallic: f32, roughness: f32) -> Self {
        Self {
            albedo: albedo.with_usage(ImageUsageType::MASKED),
            normals: normals.with_usage(ImageUsageType::NORMAL),
            metallic,
            roughness,
            #[cfg(feature = "serde")]
            trace_id: PBR_IMAGE_TRACE_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
        }
    }
}
