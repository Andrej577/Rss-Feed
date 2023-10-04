#![windows_subsystem = "windows"]
use std::{thread, time::Duration};

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use tokio::runtime;

mod database;
use crate::database::*;


fn main() 
{
    loop 
    {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new().with_resizable(false).with_inner_size(
            dioxus_desktop::wry::application::dpi::LogicalSize::new(500.0, 1000.0),
        )),);

    let duration = Duration::from_secs(60);
    thread::sleep(duration);
    }
}

fn app(cx: Scope) -> Element
{
    let rt = runtime::Runtime::new().unwrap();
    let _ = rt.block_on(request_and_save());
    let rss_items = print_from_db();
    render!(
    head 
        {
            link 
            {
                link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/bootstrap@5.1.1/dist/css/bootstrap.min.css" }
                link { rel: "script", href: "https://cdn.jsdelivr.net/npm/bootstrap@5.1.1/dist/js/bootstrap.bundle.min.js" }
            }
        }
        rsx!(
        ul
        {         
            //class: "m-1", 
            for item in rss_items
            {
                button
                {
                    value: "{item.link}",
                    class: "m-1 border rounded bg-info pe-auto",
                    onclick: 
                    {
                        let link1 = item.link;
                        to_owned![link1];
                        move |_| 
                        {
                            open_link(&link1);
                        }
                    },
                    h5
                    {
                        class: "m-4",
                        "{item.title}"
                    },
                    p
                    {
                        class: "m-4",
                        "{item.description}"
                    }
                    p
                    {
                        class: "m-4",
                        "{item.date}"
                    }
                    p
                    {
                        
                    }
                }
            }
        }
        
    ))
}

fn open_link(link1: &str)
{
    let _ = open::that(link1);
}

