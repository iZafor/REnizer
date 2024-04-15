use leptos::*;
use leptos_router::use_navigate;
use crate::{app::{components::Logo, UserCtx}, auth::Logout};

#[component]
pub fn Navbar() -> impl IntoView {
    let user_ctx = use_context::<UserCtx>().unwrap();
    
    let logout_action = use_context::<Action<Logout, Result<(), ServerFnError>>>().unwrap();

    view! {
        <Transition
            fallback=move || view! {}
        >
        <nav class="flex h-16 w-full items-center border-b border-gray-200/50 backdrop-filter bg-dark">
            <a class="flex items-center gap-2 px-4 md:px-6" href="/">
                <Logo height="140px".into() width="140px".into()/>
            </a>

            <Show
                when=move || !matches!(user_ctx.get(), Some(Ok(Some(_))))
                fallback=move || {
                    view! {
                        <button
                            class="btn m-auto mr-5 relative inline-flex items-center justify-start overflow-hidden font-medium transition-all bg-dark rounded hover:bg-white group py-1.5 px-6"
                            on:click=move |_| {
                                logout_action.dispatch(Logout {});
                                use_navigate()("", Default::default());
                            }
                        >

                            <span class="w-56 h-48 rounded bg-gray-900 absolute bottom-0 left-0 translate-x-full ease-out duration-500 transition-all translate-y-full mb-9 ml-9 group-hover:ml-0 group-hover:mb-32 group-hover:translate-x-0"></span>
                            <span class="relative text-light text-2xl cursor-pointer transition-all ease-in-out before:transition-[width] before:ease-in-out before:duration-700 before:absolute before:bg-gray-50 before:origin-center before:h-[3px] before:w-0 hover:before:w-[50%] before:bottom-0 before:left-[50%] after:transition-[width] after:ease-in-out after:duration-700 after:absolute after:bg-gray-50 after:origin-center after:h-[3px] after:w-0 hover:after:w-[50%] after:bottom-0 after:right-[50%]">
                                Logout
                            </span>
                        </button>
                    }
                }
            >

                <div class="flex-1 flex items-center justify-between gap-4 mx-4 md:mx-6">
                    <div class="flex justify-center gap-4 text-base font-medium not-italic tabular-nums">
                        <a
                            class="flex w-full items-center gap-1 rounded-md bg-gray-800 px-3 py-2 text-sm font-semibold text-light"
                            href="#"
                        >
                            Dashboard
                        </a>
                        <a
                            class="flex w-full items-center gap-1 rounded-md hover:bg-gray-700 px-3 py-2 text-sm font-semibold text-light"
                            href="#"
                        >
                            Team
                        </a>
                        <a
                            class="flex w-full items-center gap-1 rounded-md hover:bg-gray-700 px-3 py-2 text-sm font-semibold text-light"
                            href="#"
                        >
                            Projects
                        </a>
                        <a
                            class="flex w-full items-center gap-1 rounded-md hover:bg-gray-700 px-3 py-2 text-sm font-semibold text-light"
                            href="#"
                        >
                            Calendar
                        </a>
                    </div>
                    <div class="flex items-center gap-4 md:gap-6">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="fill-gray-50 cursor-pointer hover:fill-gray-500"
                            height="24px"
                            width="24px"
                            viewBox="0 0 448 512"
                        >
                            <path d="M224 0c-17.7 0-32 14.3-32 32V51.2C119 66 64 130.6 64 208v18.8c0 47-17.3 92.4-48.5 127.6l-7.4 8.3c-8.4 9.4-10.4 22.9-5.3 34.4S19.4 416 32 416H416c12.6 0 24-7.4 29.2-18.9s3.1-25-5.3-34.4l-7.4-8.3C401.3 319.2 384 273.9 384 226.8V208c0-77.4-55-142-128-156.8V32c0-17.7-14.3-32-32-32zm45.3 493.3c12-12 18.7-28.3 18.7-45.3H224 160c0 17 6.7 33.3 18.7 45.3s28.3 18.7 45.3 18.7s33.3-6.7 45.3-18.7z"></path>
                        </svg>

                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="fill-gray-50 cursor-pointer hover:fill-gray-500"
                            height="24px"
                            width="24px"
                            viewBox="0 0 448 512"
                        >
                            <path d="M224 256A128 128 0 1 0 224 0a128 128 0 1 0 0 256zm-45.7 48C79.8 304 0 383.8 0 482.3C0 498.7 13.3 512 29.7 512H418.3c16.4 0 29.7-13.3 29.7-29.7C448 383.8 368.2 304 269.7 304H178.3z"></path>
                        </svg>
                    </div>
                </div>
            </Show>
        </nav>
    </Transition>
    }
}