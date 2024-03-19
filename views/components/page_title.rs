use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub h1: String,
    pub h2: String,
    #[prop_or_default]
    pub poster_path: Option<String>,
    #[prop_or_default]
    pub background_path: Option<String>,
    #[prop_or_default]
    pub cta: Option<Html>,
    #[prop_or_default]
    pub alert: Option<Html>,
    #[prop_or_default]
    pub rating: Option<Html>,
}

#[function_component(PageTitle)]
pub fn page_title(props: &Props) -> Html {
    let background_image_styles = match &props.background_path {
        Some(v) => format!("background-image: url({}); background-size: cover; background-repeat: no-repeat;", v),
        None => String::new()
    };

    let copy_column_styles = match &props.poster_path {
        Some(_) => "col-md-8",
        None => "col-md-12"
    };

    html! {
        <header 
            class="border-bottom mb-4 py-4" 
            style={background_image_styles}
            
            >
            <div class="container ">
                {match props.alert.clone() {
                    Some(alert) => alert,
                    None => html! { <></> }
                }}
                <div class="row g-0">
                    <div class={format!("d-flex flex-column {}", copy_column_styles)}>
                        // <h1 class="p-1" style="mix-blend-mode: difference; color: white; text-shadow: 1px 1px 2px white;">{&props.h1}</h1>
                        <h1 style="-webkit-text-stroke: 1px white;">{&props.h1}</h1>
                        <h2 class="mb-0" style="-webkit-text-stroke: 1px white;">{&props.h2}</h2>
                        {
                            match props.rating.clone() {
                                Some(score) => html! {
                                    <div class="mt-auto">
                                        {score}
                                    </div>

                                }
                                ,
                                None => html! { <></> }
                            }
                        }
                        {
                            match props.cta.clone() {
                                Some(cta) => cta,
                                None => html! { <></> }
                            }
                        }
                    </div>
                    {match props.poster_path.clone() {
                        Some(v) => html! {
                            <div class="col-md-4 d-flex align-items-center">
                                <img
                                    src={v}
                                    class="img-fluid rounded-start d-block mx-auto"
                                    alt={format!("Poster for {}", props.h1)}
                                    style="aspect-ratio: 2/3; width: auto; height: 300px;"
                                />
                            </div>
                        },
                        None => html! { <></> }
                    }}
                </div>
            </div>
        </header>
    }
}
