

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {

        }
        fn seat_at_table() {

        }
    }

    mod serving {
        fn task_order() {

        }
        fn server_order() {

        }
        fn task_payment() {

        }
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}