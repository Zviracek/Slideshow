use leptos::mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(RotatingText);
}

use leptos::prelude::*;
use gloo_timers::callback::{Interval, Timeout};
use std::rc::Rc;
use leptos::*;

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
    ];

    // reactive index signal
    let (index, set_index) = signal(0);

    // number of milliseconds between changes
    let interval_ms = 5_000;
    // capture length for the timer closure
    let n = texts.len();

    let (current, set_current) = signal(0);
    let (previous, set_previous) = signal(None::<usize>);
    let (show_new, set_show_new) = signal(true);
    let n = texts.len();

    Effect::new(move |_| {
        // Interval::new requires a 'static closure, so we move owned clones in.
        let set_index = set_index.clone();

        // The timer closure updates the reactive index on each tick.
        // We use modulo by `n` to wrap around.
        let interval = Interval::new(interval_ms, move || {
            //set_index.update(|i| *i = (*i + 1) % n);
        });

        let set_current = set_current.clone();
        let set_previous = set_previous.clone();
        let set_show_new = set_show_new.clone();

        let interval = Interval::new(5_000, move || {
            set_previous.set(Some(current.get()));
            set_show_new.set(false); // start fade out
            leptos::task::spawn_local({
                let set_current = set_current.clone();
                let set_show_new = set_show_new.clone();
                async move {
                    let timeout = Timeout::new(1_000, move || {
                        //set_current.update(|i| *i = (*i + 1) % n);
                        if current.get() >= n - 1 {
                            *set_current.write() = 0;
                        } else {
                            *set_current.write() += 1;
                        }
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

    let text_old = texts.clone();
    let text_new = texts.clone();

    // render current text
    view! { 
        //<div style="font-family: system-ui, sans-serif; font-size: 28px; padding: 16px;">
        //    { move || texts[index.get()].clone() }
        //</div>
        <div class="fade-container">
            // Old text fading out
            <p class=move || if show_new.get() { "fade-text hidden" } else { "fade-text visible" }>
                { move || previous.get().map(|i| text_new[i].clone()).unwrap_or_default() }
            </p>
            // New text fading in
            <p class=move || if show_new.get() { "fade-text visible" } else { "fade-text hidden" }>
                { move || text_old[current.get()].clone() }
            </p>
        </div>
    }
}