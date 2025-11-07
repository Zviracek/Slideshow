use leptos::mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(RotatingText);
}

use leptos::prelude::*;
use gloo_timers::callback::{Interval, Timeout};

#[component]
pub fn RotatingText() -> impl IntoView {
    // The texts to rotate through.
    // Wrap in Rc so the same owned vector can be shared safely
    // between the view closures and the timer closure.
    let texts = vec![
        "Welcome to the slideshow!".to_string(),
        "Now showing: Nature".to_string(),
        "Sponsored message".to_string(),
        "Breaking: Important news".to_string(),
        "Pog".to_string(),
    ];

    // number of milliseconds between changes
    let interval_ms = 5_000;
    // capture length for the timer closure

    let (index_a, set_index_a) = signal(0);
    let (index_b, set_index_b) = signal(0);
    let (show_new, set_show_new) = signal(true);
    let (current_index_a, set_current_index_a) = signal(true);
    let n = texts.len();

    Effect::new(move |_| {
        // The timer closure updates the reactive index on each tick.

        let index_a = index_a.clone();
        let index_b = index_b.clone();
        let set_show_new = set_show_new.clone();
        let current_index_a = current_index_a.clone();

        let interval = Interval::new(interval_ms, move || {

            let mut new_index;
            if current_index_a.get() {
                new_index = index_a.get() + 1;
            } else {
                new_index = index_b.get() + 1;
            }

            if new_index >= n {
                new_index = 0;
            }

            if current_index_a.get() {
                set_current_index_a.set(false);
                set_index_b.set(new_index);
            }
            else {
                set_current_index_a.set(true);
                set_index_a.set(new_index);
            }

            set_show_new.set(false); // start fade out
            leptos::task::spawn_local({
                let set_show_new = set_show_new.clone();
                async move {
                    let timeout = Timeout::new(1_000, move || {
                        set_show_new.set(true);
                    });

                    // Since we don't plan on cancelling the timeout, call `forget`.
                    timeout.forget();
                }
            });
        });


        // on_cleanup will run when the component is disposed; dropping the interval stops it
        interval.forget();
    });

    let text_a = texts.clone();
    let text_b = texts.clone();

    // render current text
    view! { 
        //<div style="font-family: system-ui, sans-serif; font-size: 28px; padding: 16px;">
        //    { move || texts[index.get()].clone() }
        //</div>
        <div class="fade-container">
            // Old text fading out
            <p class=move || if show_new.get() { "fade-text hidden" } else { "fade-text visible" }>
                { move || text_a[index_a.get()].clone() }
            </p>
            // New text fading in
            <p class=move || if show_new.get() { "fade-text visible" } else { "fade-text hidden" }>
                { move || text_b[index_b.get()].clone() }
            </p>
        </div>
    }
}