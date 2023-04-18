use yew::prelude::*;

/// Detect browser
///
/// # Example
/// 
/// ```rust
/// use yew::prelude::*;
/// use yew_os::prelude::*;
/// 
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     // Custom user-agent config
///     let config = UaConfig {
///         custom_ua: "Mozilla/5.0 (PlayBook; U; RIM Tablet OS 1.0.0; en-US) AppleWebKit/534.11 (KHTML, like Gecko) Version/7.1.0.7 Safari/534.11".to_string(),
///     };
///     
///     // Custom user-agent string
///     let custom_browser = use_browser(Some(config));
///     
///     // Default user-agent
///     let browser = use_browser(None);
/// 
///     html! {
///         <div>
///             <h1>{custom_browser}</h1>
///             <h1>{browser}</h1>
///         </div>
///     }
/// }
/// ```

pub struct UaConfig {
    pub custom_ua: String,
}

impl Default for UaConfig {
    fn default() -> Self {
        Self {
            custom_ua: "".to_string(),
        }
    }
}

#[hook]
pub fn use_browser(config: Option<UaConfig>) -> String {
    let browser = use_state(|| "".to_string());

    use_effect_with_deps({
        let browser = browser.clone();
        let custom_ua = config.as_ref().map_or_else(
            || web_sys::window()
                .unwrap()
                .navigator()
                .user_agent()
                .unwrap(),
            |config| config.custom_ua.clone(),
        );
        move |_| {
            if custom_ua.contains("Firefox") {
                browser.set("Firefox".to_string());
            } else if custom_ua.contains("Chrome") {
                browser.set("Chrome".to_string());
            } else if custom_ua.contains("Safari") {
                browser.set("Safari".to_string());
            } else if custom_ua.contains("Edge") {
                browser.set("Edge".to_string());
            } else if custom_ua.contains("Opera") {
                browser.set("Opera".to_string());
            }
            || {}
        }
    }, ());

    browser.to_string()
}