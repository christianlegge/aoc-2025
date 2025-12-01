use seq_macro::seq;

seq!(N in 1..=20 {
    pub mod day~N;
});
