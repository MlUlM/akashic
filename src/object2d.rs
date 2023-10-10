pub mod entity;


pub trait Object2D {
    /// Horizontal position of the anchor.
    fn anchor_x(&self) -> Option<f32>;


    /// If the anchor position is changed by this method,
    /// you must call [`AkashicEntity::modified`]
    fn set_anchor_x(&self, x: Option<f32>);


    /// Vertical position of the anchor.
    fn anchor_y(&self) -> Option<f32>;


    /// If the anchor position is changed by this method,
    /// you must call [`AkashicEntity::modified`]
    fn set_anchor_y(&self, y: Option<f32>);


    /// Returns the clockwise angle(degree) of this object.
    fn angle(&self) -> f32;


    /// Set the clockwise angle(degree).
    fn set_angle(&self, angle: f32);


    /// Returns the width of this object.
    fn width(&self) -> f32;


    /// Set the width of this object.
    ///
    ///
    /// If the width is changed by this method, you must call [`AkashicEntity::modified`]
    fn set_width(&self, width: f32);


    /// Returns the height of this object.
    fn height(&self) -> f32;


    /// Set the height of this object.
    ///
    ///
    /// If the height is changed by this method, you must call [`AkashicEntity::modified`]
    fn set_height(&self, height: f32);


    /// Returns the horizontal scale.
    fn scale_x(&self) -> f32;


    /// Set the horizontal scale. of this object.
    ///
    /// If the horizontal scale is changed by this method, you must call [`AkashicEntity::modified`]
    fn set_scale_x(&self, scale_x: f32);


    /// Returns the vertical scale.
    fn scale_y(&self) -> f32;


    /// Set the vertical scale of this object.
    ///
    /// If the vertical scale is changed by this method, you must call [`AkashicEntity::modified`]
    fn set_scale_y(&self, scale_y: f32);


    /// Returns the opacity(0~1). Default is 1.
    ///
    /// If the value is 0, renderer will skip the drawing process.
    fn opacity(&self) -> f32;


    /// Set the opacity(0~1).
    ///
    /// If the opacity is changed by this method, you must call [`AkashicEntity::modified`].
    fn set_opacity(&self, opacity: f32);


    /// Returns the horizontal position.
    fn x(&self) -> f32;


    /// Set the horizontal position.
    ///
    /// If the horizontal position is changed by this method, you must call [`AkashicEntity::modified`].
    fn set_x(&self, x: f32);


    /// Returns the vertical position.
    fn y(&self) -> f32;


    /// Set the vertical position.
    ///
    /// If the vertical position is changed by this method, you must call [`AkashicEntity::modified`].
    fn set_y(&self, y: f32);


    /// Set the anchor.
    ///
    /// If the anchor is changed by this method, you must call [`AkashicEntity::modified`].
    fn anchor(&self, x: f32, y: f32);


    /// Move this object.
    ///
    /// If the position is changed by this method, you must call [`AkashicEntity::modified`].
    ///
    /// * `relative_x` - Relative horizontal position to the current position.
    /// * `relative_y` - Relative vertical position to the current position.
    fn move_by(&self, relative_x: f32, relative_y: f32);


    /// Set the position of this object.
    ///
    /// If the position is changed by this method, you must call [`AkashicEntity::modified`].
    fn move_to(&self, x: f32, y: f32);


    /// Resize this object.
    ///
    /// If the size is changed by this method, you must call [`AkashicEntity::modified`].
    ///
    /// * `relative_width` - Relative width to the current width.
    /// * `relative_height` - Relative height to the current height.
    fn resize_by(&self, relative_width: f32, relative_height: f32);


    /// Resize this object.
    ///
    /// If the size is changed by this method, you must call [`AkashicEntity::modified`].
    fn resize_to(&self, width: f32, height: f32);


    /// Set the scale of this object.
    ///
    /// If the scale is changed by this method, you must call [`AkashicEntity::modified`].
    fn scale(&self, scale: f32);


    #[inline(always)]
    fn half_width(&self) -> f32 {
        self.width() * 0.5
    }


    #[inline(always)]
    fn half_height(&self) -> f32 {
        self.height() * 0.5
    }
}
