use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx!{
        nav {
            h2 {
                "rhaskia"
            }

            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }

                li {
                    Link { to: Route::Blog {}, "Projects" }
                }

                li {
                    Link { to: Route::Blog {}, "Blog" }
                }

                li {
                    Link { to: Route::Blog {}, "Contact" }
                }
            }
        }
        Outlet::<Route> {}
    }
}
