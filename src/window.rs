use dioxus::prelude::*;

#[component]
pub fn Window(
    title: String,
    window_color: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    children: Element,
) -> Element {
    let mut x_state = use_signal(|| x);
    let mut y_state = use_signal(|| y);

    let mut x_start = use_signal(|| 0.);
    let mut y_start = use_signal(|| 0.);

    let mut dragging = use_signal(|| false);

    let onmousedown = move |event: Event<MouseData>| {
        dragging.set(true);
        let coords = event.data().client_coordinates();

        x_start.set(coords.x as f64 - x_state());
        y_start.set(coords.y as f64 - y_state());
    };

    let onmouseup = move |event: Event<MouseData>| {
        event.stop_propagation();
        dragging.set(false);
    };

    let onmouseleave = move |event: Event<MouseData>| {
        event.stop_propagation();
        dragging.set(false);
    };

    let onmousemove = move |event: Event<MouseData>| {
        if !dragging() {
            return;
        }

        let coords = event.data().client_coordinates();

        x_state.set(coords.x as f64 - x_start());
        y_state.set(coords.y as f64 - y_start());
        log::info!("dragging {:?}", (x));
    };

    let mut max_width = String::from("");
    // if props.maxwidth != "0" {
    //     max_width = format!("max-width: {};", &props.maxwidth);
    // }

    let style = format!(
        "left: calc(50% + {:?}px); top: {:?}px; min-width: {width}px; min-height: {height}px; {}",
        x_state() - (x_state() % 6.),
        y_state() - (y_state() % 6.),
        &max_width
    );
    rsx! {
        div {
            class:"window",
            style,
            div {
                class:"window-header",
                onmousedown,
                onmouseup,
                onmousemove,
                onmouseleave,
                div {
                    style: "display: flex;flex-direction: row; margin: -3px;",
                    div { "{title}" }
                    div { class:"filler" }
                    button {
                        class:"min header-button" }
                    div { class:"header-padding" }
                    button { class: "max header-button" }
                    div { class: "header-padding" }
                    button { class:"close header-button" }
                }
            }
            div {
                class:"window-body",
                background: window_color,
                { children }
            }
        }
    }
}
