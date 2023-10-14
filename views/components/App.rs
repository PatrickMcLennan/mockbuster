use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub head: Html,
}

#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <script defer={true} src="/assets/shared.js" />
                <link rel="stylesheet" href="/assets/shared.css" />
                {props.head.clone()}
            </head>
            <body>
                <Suspense {fallback}>
                    {props.children.clone()}
                </Suspense>
            </body>
        </html>
    }
}
