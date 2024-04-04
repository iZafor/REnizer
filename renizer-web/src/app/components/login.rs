use leptos::*;
use super::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <section class="roboto-regular gradient-form h-screen bg-neutral-200 dark:bg-neutral-700 flex justify-center items-center">
            <div class="container h-full p-10">
                <div class="flex h-full flex-wrap items-center justify-center text-neutral-800 dark:text-neutral-200">
                    <div class="w-full">
                        <div class="block rounded-lg bg-white shadow-lg dark:bg-neutral-800">
                            <div class="g-0 lg:flex lg:flex-wrap">
                                <div class="px-4 md:px-0 lg:w-6/12">
                                    <div class="md:mx-6 md:p-12">
                                        <div class="grid place-items-center">
                                            <Logo/>
                                        </div>
                                        <form class="space-y-4">
                                            <p class="mb-4">Please login to your account</p>

                                            <Input
                                                label=String::from("Email")
                                                type_=String::from("email")
                                            />

                                            <Input
                                                label=String::from("Password")
                                                type_=String::from("password")
                                            />

                                            <div class="mb-12 pb-1 pt-1 text-center">
                                                <button
                                                    class="mb-3 inline-block w-full rounded px-6 pb-2 pt-2.5 text-xs font-medium uppercase leading-normal text-white shadow-dark-3 transition duration-150 ease-in-out hover:shadow-dark-2 focus:shadow-dark-2 focus:outline-none focus:ring-0 active:shadow-dark-2 dark:shadow-black/30 dark:hover:shadow-dark-strong dark:focus:shadow-dark-strong dark:active:shadow-dark-strong"
                                                    type="button"
                                                    data-twe-ripple-init
                                                    data-twe-ripple-color="light"
                                                    style="
                                                    background: linear-gradient(to right, #ee7724, #d8363a, #dd3675, #b44593);
                                                    "
                                                >
                                                    Log in
                                                </button>

                                                <a href="#!">Forgot password?</a>
                                            </div>

                                            <div class="flex items-center justify-between pb-6">
                                                <p class="mb-0 me-2">"Don't have an account?"</p>
                                                <button
                                                    type="button"
                                                    class="inline-block rounded border-2 border-danger px-6 pb-[6px] pt-2 text-xs font-medium uppercase leading-normal text-danger transition duration-150 ease-in-out hover:border-danger-600 hover:bg-danger-50/50 hover:text-danger-600 focus:border-danger-600 focus:bg-danger-50/50 focus:text-danger-600 focus:outline-none focus:ring-0 active:border-danger-700 active:text-danger-700 dark:hover:bg-rose-950 dark:focus:bg-rose-950"
                                                    data-twe-ripple-init
                                                    data-twe-ripple-color="light"
                                                    on:click=move |_| {
                                                        leptos_router::use_navigate()(
                                                            "/user-type",
                                                            Default::default(),
                                                        );
                                                    }
                                                >

                                                    Register
                                                </button>
                                            </div>
                                        </form>
                                    </div>
                                </div>

                                <div
                                    class="flex items-center rounded-b-lg lg:w-6/12 lg:rounded-e-lg lg:rounded-bl-none"
                                    style="background: linear-gradient(to right, #ee7724, #d8363a, #dd3675, #b44593)"
                                >
                                    <div class="w-full text-white text-center animate-slidein">
                                        <h4 class="mb-2 text-8xl font-bold">Welcome to REnizer,</h4>
                                        <p class="text-xl text-neutral-800">
                                            where innovation meets sustainability
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}