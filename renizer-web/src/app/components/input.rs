use leptos::{html::Input, *};
use wasm_bindgen::JsValue;

#[component]
pub fn Input<T: Clone + Into<JsValue> + 'static>(
    label: String,
    value: ReadSignal<T>,
    #[prop(optional)]
    name: String,
    #[prop(default=String::from("text"))]
    type_: String,
    #[prop(into, default=Callback::new(|_| {}))]
    on_input: Callback<ev::Event, ()>, 
    #[prop(into, default=Callback::new(|_| {}))]
    on_change: Callback<ev::Event>,
    #[allow(unused)]
    #[prop(optional)]
    node_ref: NodeRef<Input>
) -> impl IntoView {
    view! {
        <div class="relative h-11 w-full">
            <input
                type=type_
                placeholder=label.clone()
                class="peer h-full w-full border-b border-gray-200 bg-transparent pt-4 pb-1.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border-gray-200 focus:border-gray-500 focus:outline-0 disabled:border-0 disabled:bg-blue-gray-50 placeholder:opacity-0 focus:placeholder:opacity-100"
                name=name
                prop:value=value
                on:input=on_input
                on:change=on_change
                _ref=node_ref
            />
            <label class="after:content[''] pointer-events-none absolute left-0  -top-1.5 flex h-full w-full select-none !overflow-visible truncate text-[11px] font-normal leading-tight text-light transition-all after:absolute after:-bottom-1.5 after:block after:w-full after:scale-x-0 after:border-b-2 after:border-gray-500 after:transition-transform after:duration-300 peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[4.25] peer-placeholder-shown:text-white peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-white peer-focus:after:scale-x-100 peer-focus:after:border-blue-gray-900 peer-disabled:text-transparent peer-disabled:peer-placeholder-shown:text-white">
                {label}
            </label>
        </div>
    }
}