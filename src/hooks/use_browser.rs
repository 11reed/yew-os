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
///     let browser = use_browser();
/// 
///     html! {
///         <div>{browser}</div>
///     }
/// }
/// ```

#[hook]
pub fn use_browser() -> String {
    let browser = use_state(|| "".to_string());

    use_effect_with_deps({
        let browser = browser.clone();
        move |_| {
            let user_agent = web_sys::window()
                .unwrap()
                .navigator()
                .user_agent()
                .unwrap();
            if user_agent.contains("Firefox") {
                browser.set("Firefox".to_string());
            } else if user_agent.contains("Chrome") {
                browser.set("Chrome".to_string());
            } else if user_agent.contains("Safari") {
                browser.set("Safari".to_string());
            } else if user_agent.contains("Edge") {
                browser.set("Edge".to_string());
            } else if user_agent.contains("Opera") {
                browser.set("Opera".to_string());
            }
            || {}
        }
    }, ());

    browser.to_string()
}