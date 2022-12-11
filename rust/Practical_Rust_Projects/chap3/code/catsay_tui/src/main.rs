extern crate cursive;
use std::collections::TryReserveError;

use cursive::Cursive;
use cursive::traits::Identifiable;
use cursive::views::{ TextView, Dialog, Checkbox, EditView, ListView };
use cursive::event::Key;

// wrap all form fields value in one struct so we can pass around easily
struct CatsayOptions<'a> {
    message: &'a str,
    dead: bool,
}

fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new().title("Please fill out the form for the cat")
        .content(
            ListView::new()
                .child("Message:", EditView::new().with_id("message"))
                .child("Dead?", Checkbox::new().with_id("dead")),
        )
        .button(
            "OK", 
            |s| {
                let message = s.call_on_id("message", |t: &mut EditView| t.get_content()).unwrap();
                let is_dead = s.call_on_id("dead", |t: &mut Checkbox| t.is_checked()).unwrap();
                let options = CatsayOptions {
                    message: &message,
                    dead: is_dead,
                };
                result_step(s, &options)
            },
        )
    );
}

fn result_step(siv: &mut Cursive, options: &CatsayOptions) {
    let eye = if options.dead { "x" } else { "o" };
    let cat_text = format!(
        "{msg}
  \\
   \\
    /\\_/\\
   ( {eye} {eye} )
   =( I )=",
        msg = options.message,
        eye = eye,
    );
    siv.pop_layer();
    siv.add_layer(
        Dialog::around(TextView::new(cat_text))
            .title("the cat says...")
            .button("OK", |s| s.quit())
    );
}

fn main() {
/*     let mut siv = Cursive::default(); */
/*     let cat_text = "Meow! */
/*  \\ */
/*   \\ */
/*     /\\_/\\ */
/*    ( o o ) */
/*    =( I )="; */
/*  */
/*     // declaring the app layout */
/*     siv.add_layer(TextView::new(cat_text)); */
/*     // Listen to Key::Esc and quit */
/*     // s in the callback func is a "Cursive" struct itself, so we could call s.quit() */
/*     siv.add_global_callback(Key::Esc, |s| s.quit()); */
/*     //dialog layer */
/*     siv.add_layer( */
/*         Dialog::around(TextView::new(cat_text)) */
/*             .button("Ok", |s| s.quit())); */
/*  */
    let mut siv = Cursive::default();
    input_step(&mut siv);
    siv.run();
}
