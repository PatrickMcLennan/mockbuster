use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_page: i64,
    pub total_pages: i64,
    pub previous_url: String,
    pub next_url: String,
    pub numbered_url: String,
}

#[function_component(Pagination)]
pub fn pagination(props: &Props) -> Html {
    let is_first_page = &props.current_page == &1;
    let is_last_page = props.current_page == props.total_pages as i64;
    let has_more_than_5_pages_remaining = props.current_page + 5 <= props.total_pages;
    let has_more_than_5_pages_in_total = props.current_page - 5 > 0;
    let has_more_than_2_pages_remaining = props.current_page + 2 <= props.total_pages;
    let has_more_than_2_pages_in_total = props.current_page - 2 > 0;

    let offered_pagination = if is_first_page {
        if has_more_than_5_pages_remaining {
            (1..=5).collect::<Vec<i64>>()
        } else {
            (1..=props.total_pages).collect::<Vec<i64>>()
        }
    } else if is_last_page {
        if has_more_than_5_pages_in_total {
            ((props.current_page - 4)..=props.current_page).collect::<Vec<i64>>()
        } else {
            (1..=props.current_page).collect::<Vec<i64>>()
        }
    } else {
        if has_more_than_2_pages_remaining && has_more_than_2_pages_in_total {
            ((props.current_page - 2)..=(props.current_page + 2)).collect::<Vec<i64>>()
        } else {
            (1..=props.current_page).collect::<Vec<i64>>()
        }
    };

    html! { 
        <footer class="mt-4 pt-4 border-top container">
            <nav aria-label="Search pagination">
                <ul class="pagination justify-content-center">
                    <li class={classes!(
                        "page-item",
                        if is_first_page { Some("disabled") } else { None }
                    )}>
                        <a
                            class="page-link"
                            href={if is_first_page { "#".to_string() } else { props.previous_url.to_string() }}
                            tabindex={if is_first_page { Some("-1") } else { None }}
                            aria-disabled={if is_first_page { Some("true") } else { None }}
                        >
                            {"Previous"}
                        </a>
                    </li>
                    {
                        offered_pagination
                            .into_iter()
                            .map(|page| {
                                let is_current_page = page == props.current_page;
                                html! {
                                    <li
                                        class={classes!(
                                            "page-item",
                                            if is_current_page { Some("active disabled") } else { None }
                                        )}
                                        key={page}
                                    >
                                        <a
                                            class="page-link"
                                            href={if is_current_page { "#".to_string() } else { format!("{}&page={}", props.numbered_url, &page) }}
                                            tabindex={if is_current_page { Some("-1") } else { None }}
                                            aria-current={if is_first_page { Some("true") } else { None }}
                                            aria-disabled={if is_current_page { Some("true") } else { None }}
                                        >
                                            {page}
                                        </a>
                                    </li>
                                }
                            })
                            .collect::<Html>()
                    }
                    <li class={classes!(
                        "page-item",
                        if is_last_page { Some("disabled") } else { None }
                    )}>
                        <a
                            class="page-link"
                            href={if is_last_page { "#".to_string() } else { props.next_url.to_string() }}
                            tabindex={if is_first_page { Some("-1") } else { None }}
                            aria-disabled={if is_first_page { Some("true") } else { None }}
                        >
                            {"Next"}
                        </a>
                    </li>
                </ul>
            </nav>
        </footer>
     }
}
