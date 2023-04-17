use yew::prelude::*;

/// Detect OS
///
/// # Example
/// 
/// ```rust
/// use yew::prelude::*;
/// use yew_os::prelude::*;
/// 
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let os = use_os();
/// 
///     html! {
///         <div>{os}</div>
///     }
/// }
/// ```

#[hook]
pub fn use_os() -> String {
    let os = use_state(|| "".to_string());

    use_effect_with_deps({
        let os = os.clone();
        move |_| {
            let user_agent = web_sys::window()
                .unwrap()
                .navigator()
                .user_agent()
                .unwrap();
            
            if user_agent.contains("Mac") && !user_agent.contains("iPhone") && !user_agent.contains("iPad") {
                os.set("Mac".to_string());

            } else if user_agent.contains("Windows") {
                os.set("Windows".to_string());

            } else if user_agent.contains("Fedora") {
                os.set("Fedora".to_string());

            } else if user_agent.contains("Ubuntu") {
                os.set("Ubuntu".to_string());

            } else if user_agent.contains("Linux") {
                os.set("Linux".to_string());
            }
            || {}
        }
    }, ());

    os.to_string()
}