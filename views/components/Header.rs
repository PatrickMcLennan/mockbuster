use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="navbar navbar-light">
			<div class="container">
				<div class="col-2">
					<h2>{"mockbuster"}</h2>
				</div>
				<menu class="offset-5">
					<li>{"Menu Item 2"}</li>
				</menu>
			</div>
        </header>
    }
}
