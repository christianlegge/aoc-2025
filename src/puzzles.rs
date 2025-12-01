use seq_macro::seq;

seq!(N in 1..=12 {
    pub mod day~N;
});
