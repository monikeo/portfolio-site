use leptos::*;

use crate::components::theme_controller::ThemeController;

// Define the navigation items
const NAVIGATIONS: &[(&str, &str)] = &[
    ("/projects", "Projects"),
    ("/blogs", "Blogs"),
    ("/resume", "Resume"),
    ("/contact", "Contact"),
    ("/about", "About"),
];

#[warn(non_snake_case)]
#[component]
pub fn NavLink(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <a class="transition hover:text-teal-500 dark:hover:text-teal-400" href=href>
            {children()}
        </a>
    }
}

#[warn(non_snake_case)]
#[component]
pub fn DropDownMenuContent(navigations_link: &'static [(&'static str, &'static str)]) -> impl IntoView {
    view!{
        <ul
            tabIndex="0"
            class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 bg-opacity-100 rounded-box w-52"
        >
            {navigations_link.iter().map(|nav| {
                view!{
                    <li>
                        <NavLink href=nav.0>
                            {nav.1}
                        </NavLink>
                    </li>
                }
            }).collect::<Vec<_>>()} 
        </ul>
    }
}

#[component]
pub fn NavBarStart() -> impl IntoView {
    let title = "Moni Keo";
    view!{
        <div class="navbar-start">
            <div class="dropdown">
                <label tabIndex="0" class="btn btn-ghost lg:hidden">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-5 w-5"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth="2"
                            d="M4 6h16M4 12h8m-8 6h16"
                        />
                    </svg>
                </label>

                <DropDownMenuContent navigations_link=NAVIGATIONS />
            </div>

            /*
            <div class="avatar">
                <div class="w-11 rounded-xl">

                </div>
            </div>
            */

            <div>
                <NavLink href="/">
                <a class="btn btn-ghost normal-case text-xl">
                    {title}
                </a>
                </NavLink>
            </div>
        </div>
    }
}

#[component]
pub fn NavBarCenter(navigations_link: &'static [(&'static str, &'static str)]) -> impl IntoView {
    view!{
        <div class="navbar-center hidden lg:flex">
            <ul class="menu menu-horizontal p-1">
                {navigations_link.iter().map(|nav| {
                    view!{
                        <li>
                            <NavLink href={nav.0}>
                                {nav.1}
                            </NavLink>
                            /*
                            <a href={nav.0}>
                                {nav.1}
                            </a>
                            */
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
pub fn NavBarEnd() -> impl IntoView {
    view!{
        <div class="navbar-end">
            /*
            <a href="#">
                <button class="btn btn-ghost btn-circle  h-2 p-3">
                    <p>end</p> 
                </button>
            </a>
            */
            <button class="btn btn-ghost btn-circle h-2 p-3">
            <ThemeController />
            </button>
        </div>
    }
}

#[component]
pub fn Nav() -> impl IntoView {
    view!{
        <div class="shadow-sm backdrop-blur-md w-full navbar bg-base-200 bg-opacity-10">
                <NavBarStart /> 
                <NavBarCenter navigations_link=NAVIGATIONS/>
                <NavBarEnd />
        </div>
    }
}

