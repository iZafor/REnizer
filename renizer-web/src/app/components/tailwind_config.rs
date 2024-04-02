use leptos::*;
use leptos_meta::*;

#[component]
pub fn TailwindConfig() -> impl IntoView {
    view! {
        <Script>
            r#"
                tailwind.config = {
                    theme: {
                        extend: {
                          keyframes: {
                            slidein: {
                              from: {
                                opacity: "0",
                                transform: "translateY(-10px)",
                              },
                              to: {
                                opacity: "1",
                                transform: "translateY(0)",
                              },
                            },
                          },
                          animation: {
                            slidein: "slidein 1s ease 300ms",
                          },
                        }
                    }
                }
            "#
        </Script>
    }
}