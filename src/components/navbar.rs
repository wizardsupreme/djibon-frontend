use dioxus::prelude::*;
use crate::Route;
use crate::state::{use_app_state, Theme, toggle_theme};

#[component]
pub fn NavBar() -> Element {
    // Get the app state
    let state = use_app_state();

    // Get the current route
    let route = use_route::<Route>();

    // Helper function to determine if a link is active
    let _is_active = |r: &Route| -> bool {
        std::mem::discriminant(r) == std::mem::discriminant(&route)
    };

    // Check if the theme is dark
    let is_dark = matches!(state.read().theme, Theme::Dark);

    // Create an event handler for the theme toggle
    let handle_toggle = move |_| {
        toggle_theme(state);
    };

    rsx! {
        // Top navbar with toggle button
        nav {
            class: "navbar navbar-expand-lg border-bottom py-2 shadow-sm",
            div {
                class: "container-fluid",
                // Left side - Brand logo
                Link {
                    to: Route::Home {},
                    class: "navbar-brand d-flex align-items-center",
                    // Tree of life logo
                    svg {
                        width: "30",
                        height: "30",
                        view_box: "0 0 24 24",
                        fill: "currentColor",
                        class: "me-2",

                        // Tree of life logo
                        path {
                            d: "M12 2c0 0-9 6.5-9 13c0 4.4 4 7 9 7s9-2.6 9-7C21 8.5 12 2 12 2zm0 18c-3.9 0-7-1.7-7-5c0-4.9 7-10.5 7-10.5s7 5.6 7 10.5C19 18.3 15.9 20 12 20z"
                        }
                        path {
                            d: "M12 6.5c0 0-5 3.5-5 7.5c0 2.5 2.2 4 5 4s5-1.5 5-4C17 10 12 6.5 12 6.5zm0 9.5c-1.9 0-3-0.9-3-2c0-2.2 3-4.8 3-4.8s3 2.6 3 4.8C15 15.1 13.9 16 12 16z"
                        }
                        circle {
                            cx: "12",
                            cy: "13",
                            r: "1.5"
                        }
                    }
                    "Jeebon"
                }

                // Center spacer to push elements to the right
                div { class: "flex-grow-1" }

                // Right side - Theme toggle button
                button {
                    class: "btn btn-link text-decoration-none me-3",
                    onclick: handle_toggle,
                    i {
                        class: if is_dark {
                            "bi bi-sun fs-5"
                        } else {
                            "bi bi-moon fs-5"
                        }
                    }
                }

                // Right side - Menu toggle button
                button {
                    class: "navbar-toggler",
                    r#type: "button",
                    "data-bs-toggle": "offcanvas",
                    "data-bs-target": "#sidebarMenu",
                    "aria-controls": "sidebarMenu",
                    "aria-expanded": "false",
                    "aria-label": "Toggle navigation",
                    span { class: "navbar-toggler-icon" }
                }
            }
        }

        // Sidebar offcanvas menu
        div {
            class: "offcanvas offcanvas-start w-75",
            "data-bs-backdrop": "true",
            "data-bs-scroll": "false",
            id: "sidebarMenu",
            tabindex: "-1",
            "aria-labelledby": "sidebarMenuLabel",

            // Offcanvas header
            div { class: "offcanvas-header shadow-sm",
                div { class: "d-flex align-items-center",
                    svg {
                        width: "30",
                        height: "30",
                        view_box: "0 0 24 24",
                        fill: "currentColor",
                        class: "me-2",

                        // Tree of life logo
                        path {
                            d: "M12 2c0 0-9 6.5-9 13c0 4.4 4 7 9 7s9-2.6 9-7C21 8.5 12 2 12 2zm0 18c-3.9 0-7-1.7-7-5c0-4.9 7-10.5 7-10.5s7 5.6 7 10.5C19 18.3 15.9 20 12 20z"
                        }
                        path {
                            d: "M12 6.5c0 0-5 3.5-5 7.5c0 2.5 2.2 4 5 4s5-1.5 5-4C17 10 12 6.5 12 6.5zm0 9.5c-1.9 0-3-0.9-3-2c0-2.2 3-4.8 3-4.8s3 2.6 3 4.8C15 15.1 13.9 16 12 16z"
                        }
                        circle {
                            cx: "12",
                            cy: "13",
                            r: "1.5"
                        }
                    }
                    h5 { class: "offcanvas-title mb-0", id: "sidebarMenuLabel", "Jeebon" }
                }
                button {
                    r#type: "button",
                    class: "btn",
                    "data-bs-dismiss": "offcanvas",
                    "aria-label": "Close",
                    i { class: "bi bi-chevron-left fs-4" }
                }
            }

            // Offcanvas body
            div { class: "offcanvas-body d-flex flex-column p-0 overflow-auto",
                // Navigation menu - make it scrollable if needed
                div { class: "nav-container overflow-auto flex-grow-1 p-3",
                    ul {
                        class: "nav flex-column",
                        li {
                            class: "nav-item mb-2",
                            Link {
                                to: Route::Home {},
                                class: "nav-link d-flex align-items-center rounded py-2 px-3 mb-1",
                                active_class: "bg-primary text-white fw-bold",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-house-door me-2 fs-5" }
                                "Home"
                            }
                        }
                        li {
                            class: "nav-item mb-2",
                            Link {
                                to: Route::Profile {},
                                class: "nav-link d-flex align-items-center rounded py-2 px-3 mb-1",
                                active_class: "bg-primary text-white fw-bold",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-person me-2 fs-5" }
                                "Profile"
                            }
                        }
                        li {
                            class: "nav-item mb-2",
                            Link {
                                to: Route::Comms {},
                                class: "nav-link d-flex align-items-center rounded py-2 px-3 mb-1",
                                active_class: "bg-primary text-white fw-bold",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-chat-dots me-2 fs-5" }
                                "Comms"
                            }
                        }
                        li {
                            class: "nav-item mb-2",
                            Link {
                                to: Route::Circles {},
                                class: "nav-link d-flex align-items-center rounded py-2 px-3 mb-1",
                                active_class: "bg-primary text-white fw-bold",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-people me-2 fs-5" }
                                "Circles"
                            }
                        }
                        li {
                            class: "nav-item mb-2",
                            Link {
                                to: Route::Tree {},
                                class: "nav-link d-flex align-items-center rounded py-2 px-3 mb-1",
                                active_class: "bg-primary text-white fw-bold",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-diagram-3 me-2 fs-5" }
                                "Trees"
                            }
                        }
                        li {
                            class: "nav-item mb-2",
                            Link {
                                to: Route::Settings {},
                                class: "nav-link d-flex align-items-center rounded py-2 px-3 mb-1",
                                active_class: "bg-primary text-white fw-bold",
                                "data-bs-dismiss": "offcanvas",
                                i { class: "bi bi-gear me-2 fs-5" }
                                "Settings"
                            }
                        }
                    }
                }

                // System Info link at bottom - Fixed position
                div { class: "border-top p-3 mt-auto shadow-sm",
                    Link {
                        to: Route::SystemInfo {},
                        class: "d-flex align-items-center text-decoration-none",
                        "data-bs-dismiss": "offcanvas",
                        i { class: "bi bi-info-circle fs-4 me-2" }
                        { format!("v{}", state.read().version) }
                    }
                }
            }
        }
    }
}