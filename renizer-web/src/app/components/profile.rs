use leptos::*;

use crate::app::UserCtx;

#[component]
pub fn Profile() -> impl IntoView {
    let user_ctx = use_context::<UserCtx>().unwrap();

    view! {
        <Transition
            fallback=move || view! {}
        >
        {move|| match user_ctx.get() {
            None | Some(Err(_)) | Some(Ok(None)) => view! {  }.into_view(),
            Some(Ok(Some(user))) => view! {
                        <section class="">
                            <div class="container h-full p-10">
                                <div class="flex h-full flex-wrap items-center justify-center text-light">
                                    <div class="w-1/3">
                                        <div class="bg-dark border-2 border-gray-50 block rounded-lg shadow-lg">
                                            <div class="g-0 lg:flex lg:flex-wrap">
                                                <div class="grid w-full md:px-0">
                                                    <div class="md:mx-6 md:p-12 space-y-1">
                                                        <p class="text-2xl font-semibold">User Profile</p>
                                                        <p class="text-sm text-gray-400">
                                                            Registered as
                                                            {if user.user_type == "I" {
                                                                "Investor"
                                                            } else {
                                                                "Project Associate"
                                                            }}
        
                                                        </p>
                                                        <div class="border-t border-gray-50 flex space-x-8 py-2">
                                                            <p class="text-md text-gray-200 font-semibold">
                                                                First Name
                                                            </p>
                                                            <p class="">{user.first_name}</p>
                                                        </div>
                                                        <div class="border-t border-gray-50 flex space-x-8 py-2">
                                                            <p class="text-md text-gray-200 font-semibold">Last Name</p>
                                                            <p class="">{user.last_name}</p>
                                                        </div>
                                                        <div class="border-t border-gray-50 flex space-x-8 py-2">
                                                            <p class="text-md text-gray-200 font-semibold">Email</p>
                                                            <p class="">{user.email}</p>
                                                        </div>
                                                        <div class="border-t border-gray-50 flex space-x-8 py-2">
                                                            <p class="text-md text-gray-200 font-semibold">
                                                                Contact Number
                                                            </p>
                                                            <p class="">{user.contact_number}</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>
                    }
                        .into_view()
        }}
        </Transition>
    }
}