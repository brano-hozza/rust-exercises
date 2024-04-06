use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let btnCls = vec!["p-2", "bg-red-100", "rounded", "w-1/2"];

    html! {
        <div class="flex w-full h-screen justify-center items-center">
            <div class="w-1/2 flex justify-between">
                <button class={classes!(btnCls)} {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
