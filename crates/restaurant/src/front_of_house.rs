//Modules ----------------------------------
pub mod hosting;

mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}

// This doesnt work
// mod front_of_house {
    // pub mod hosting;

    // mod serving {
    //     fn take_order() {}
    //     fn serve_order() {}
    //     fn take_payment() {}
    // }
// }
