use yew::prelude::*;
use web_sys::{Navigator, window};

/// Detect and display OS
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
                os.set("mac".to_string());

            } else if user_agent.contains("Windows") {
                os.set("windows".to_string());

            } else if user_agent.contains("Fedora") {
                os.set("fedora".to_string());

            } else if user_agent.contains("Ubuntu") {
                os.set("ubuntu".to_string());

            } else if user_agent.contains("Linux") {
                os.set("linux".to_string());
            }
            || {}
        }
    }, ());

    os.to_string()
}