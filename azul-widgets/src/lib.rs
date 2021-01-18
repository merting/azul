// #![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/maps4print/azul/master/assets/images/azul_logo_full_min.svg.png",
    html_favicon_url = "https://raw.githubusercontent.com/maps4print/azul/master/assets/images/favicon.ico",
)]

//! Built-in widgets for the Azul GUI system

extern crate azul;
extern crate alloc;

/// Button widget
pub mod button;
/// Label widget
pub mod label;
// Text input (two-way binding) widget
// pub mod text_input;
// Table view (iframe) widget
pub mod table_view;