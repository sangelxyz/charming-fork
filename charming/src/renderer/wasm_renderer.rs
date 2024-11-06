use crate::{theme::Theme, Chart, EchartsError};
use serde::Serialize;
use serde_wasm_bindgen::from_value;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
// graphic insert
// use js_sys::Function;
// use serde_json::json;
// use web_sys::console;
//
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::window;

pub struct WasmRenderer {
    theme: Theme,
    width: Option<u32>,  // Optional width
    height: Option<u32>, // Optional height.
}

impl WasmRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            theme: Theme::Default,
            width: Some(width),
            height: Some(height),
        }
    }
    pub fn new_opt(width: Option<u32>, height: Option<u32>) -> Self {
        Self {
            theme: Theme::Default,
            width,
            height,
        }
    }
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn render(&self, id: &str, chart: &Chart) -> Result<Echarts, EchartsError> {
        let window = web_sys::window().ok_or(EchartsError::WasmError(
            "no `window` object found".to_string(),
        ))?;
        let document = window.document().ok_or(EchartsError::WasmError(
            "no `document` object found".to_string(),
        ))?;
        let element = document
            .get_element_by_id(id)
            .ok_or(EchartsError::WasmError(format!(
                "no element with id `{}` found",
                id
            )))?;
        let echarts = init(
            &element,
            self.theme.to_str().0,
            to_value(&ChartSize {
                width: self.width,
                height: self.height,
            })
            .unwrap(),
        );
        echarts.set_option(to_value(chart).unwrap());

        Ok(echarts)
    }

    /// Resizes a chart with options specified in [`ChartResize`]
    pub fn resize_chart(echarts: &Echarts, chart_size: ChartResize) {
        echarts.resize(); //to_value(&chart_size).expect("could not convert resize options to `JsValue`")
    }

    pub fn update(echarts: &Echarts, chart: &Chart) {
        echarts.set_option(to_value(chart).unwrap());
    }

    /// Saves the current ECharts instance as an image.
    ///
    /// This function retrieves the data URL of the chart in the specified format
    /// (e.g., 'png' or 'jpeg') and can adjust the quality using the pixel ratio.
    ///
    /// # Arguments
    ///
    /// * `echarts` - A reference to the `Echarts` instance from which to generate the image.
    /// * `option` - A `JsValue` containing options for the `getDataURL` method.
    ///   This should include:
    ///   - `type`: The image format, which can be 'png' or 'jpeg'.
    ///   - `pixelRatio`: A factor to adjust the image quality (default is 1).
    ///   - `backgroundColor`: An optional background color for the image, specified as a hex string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let options = JsValue::from_serde(&json!({
    ///     "type": "png",
    ///     "pixelRatio": 2,
    ///     "backgroundColor": "#fff"
    /// })).unwrap();
    /// save_image(&my_echarts_instance, options);
    pub fn save_image(echarts: &Echarts, option: JsValue) {
        echarts.get_data_url(option);
    }
}

#[derive(Serialize)]
struct ChartSize {
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Serialize)]
pub struct ChartResize {
    /// New width in px
    width: u32,
    /// New height in px
    height: u32,
    /// If true, emits events on resize
    silent: bool,
    /// Resize animation options
    animation: Option<Animation>,
}

#[derive(Serialize)]
pub struct Animation {
    /// duration of the animation
    pub duration: u32,
    /// easing function used for the animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub easing: Option<Easing>,
}

/// available easing functions in echarts
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Easing {
    #[default]
    Linear,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuarticIn,
    QuarticOut,
    QuarticInOut,
    QuinticIn,
    QuinticOut,
    QuinticInOut,
    SinusoidalIn,
    SinusoidalOut,
    SinusoidalInOut,
    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,
    CircularIn,
    CircularOut,
    CircularInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            duration: 100,
            easing: Some(Easing::default()),
        }
    }
}

impl Animation {
    pub fn new(duration: u32, easing: Option<Easing>) -> Self {
        Self {
            duration,
            easing: easing.or_else(|| Some(Easing::default())),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = echarts)]
    pub type Echarts;

    #[wasm_bindgen(js_namespace = echarts, js_name = init)]
    fn init(id: &web_sys::Element, theme: &str, size: JsValue) -> Echarts;

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_option(this: &Echarts, option: JsValue);

    #[wasm_bindgen(method, js_name = "resize")]
    pub fn resize(this: &Echarts); //

    #[wasm_bindgen(method, js_name = "getDataUrl")]
    pub fn get_data_url(this: &Echarts, option: JsValue);
}

#[wasm_bindgen]
pub fn add_resize_listener(chart: JsValue) {
    let window = window().expect("should have a window in this context");

    // Create a closure that will call the `resize` method on the Echarts instance
    let closure = Closure::wrap(Box::new(move || {
        // Call the resize method on the Echarts JsValue with `null` as the options
        chart.clone().unchecked_into::<Echarts>().resize(); //JsValue::NULL
    }) as Box<dyn FnMut()>);

    // Add the closure as the event listener for the `resize` event
    window
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .expect("should register `resize` event listener");

    // Forget the closure to avoid it being dropped
    closure.forget();
}

#[wasm_bindgen]
pub fn datazoom(chart: JsValue) -> Result<(), JsValue> {
    use serde_json::json;
    use web_sys::js_sys::Function;
    use web_sys::js_sys::Reflect;
    // Create the dataZoom option as a JsValue

    // // Get the setOption function from the chart JsValue
    // let set_option = Reflect::get(&chart, &JsValue::from_str("setOption"))?;

    // // Ensure the setOption is a function
    // let set_option_fn = set_option.dyn_into::<Function>()?;

    // // Call the setOption function with the dataZoom option
    // set_option_fn.call1(&chart, &data_zoom_option)?;
    let closure = Closure::wrap(Box::new(move || {
        let data_zoom_option = to_value(&json!(
            r#"{
                title: {
                    text: 'Basic Bar Chart'
                },
                tooltip: {},
                xAxis: {
                    type: 'category',
                    data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
                },
                yAxis: {
                    type: 'value'
                },
                series: [{
                    name: 'Sales',
                    type: 'bar',
                    data: [150, 230, 224, 218, 135, 147, 260],
                    emphasis: {
                        focus: 'series'
                    }
                }]
                }"#
        ));
        // Call the resize method on the Echarts JsValue with `null` as the options
        chart
            .clone()
            .unchecked_into::<Echarts>()
            .set_option(data_zoom_option.unwrap()); //JsValue::NULL
    }) as Box<dyn FnMut()>);
    Ok(())
}
