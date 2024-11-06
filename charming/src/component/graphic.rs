use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Graphics {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shape: Option<Shape>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<(f64, f64)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<(f64, f64)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Style>,

    #[serde(skip_serializing_if = "Option::is_none")]
    animation: Option<Animation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    invisible: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shape {
    #[serde(skip_serializing_if = "Option::is_none")]
    r: Option<f64>, // For circle radius

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    fill: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stroke: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_width: Option<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    easing: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<u64>,
}

impl Graphics {
    pub fn new() -> Self {
        Self {
            id: None,
            z: None,
            shape: None,
            position: None,
            rotation: None,
            scale: None,
            style: None,
            animation: None,
            invisible: None,
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn z<F: Into<f64>>(mut self, z: F) -> Self {
        self.z = Some(z.into());
        self
    }

    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Some((x, y));
        self
    }

    pub fn rotation<F: Into<f64>>(mut self, rotation: F) -> Self {
        self.rotation = Some(rotation.into());
        self
    }

    pub fn scale(mut self, x: f64, y: f64) -> Self {
        self.scale = Some((x, y));
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn animation(mut self, animation: Animation) -> Self {
        self.animation = Some(animation);
        self
    }

    pub fn invisible(mut self, invisible: bool) -> Self {
        self.invisible = Some(invisible);
        self
    }
}

// Example usage for Shape, Style, and Animation can be added similarly.
