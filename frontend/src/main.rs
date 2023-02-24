#![feature(async_closure)]
#![feature(async_fn_in_trait)]
#![feature(cell_update)]
#![feature(strict_provenance)]
#![feature(vec_into_raw_parts)]
#![feature(yeet_expr)]

mod root;

use gloo::net::http::Request;
use log::Level;
use wasm_logger::Config;
use yew::{function_component, Html, html, use_state, use_effect, platform::spawn_local};
use yew_router::{BrowserRouter, Switch, Routable};

use root::Root;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Root,

    #[at("/index.html")]
    Root2,

    #[at("/hello-server")]
    HelloServer,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Root | Route::Root2 => html! {
            <Root/>
        },
        
        Route::HelloServer => html! {
            <HelloServer/>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

#[function_component(HelloServer)]
fn hello_server() -> Html {
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let response = Request::get("/api/root").send().await.unwrap();

                    let result = if response.ok() {
                        response.text().await.map_err(|err| err.to_string())
                    }
                    else {
                        Err(format!(
                            "Error fetching data {} ({})",
                            response.status(),
                            response.status_text(),
                        ))
                    };

                    data.set(Some(result));
                })
            }
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }

        Some(Ok(val)) => {
            html! {
                <div>{"Got server response: "}{val}</div>
            }
        }

        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server"}{err}</div>
            }
        }
    }
}

fn main() {
    wasm_logger::init(Config::new(Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}