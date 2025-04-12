/// A macro to measure the execution time of a block of code.
/// 
/// # Arguments
/// 
/// - `$duration_holder`: The variable that will hold the elapsed time as a `std::time::Duration`.
/// - `$result` (optional): The variable that will hold the result of the code block, if any.
/// - `$code_block`: The block of code whose execution time is to be measured.
/// 
/// # Example
/// 
/// ```rust
/// howlast!(elapsed_time, result => {
///     // do something that takes time
///     std::thread::sleep(std::time::Duration::from_secs(2));
///     42
/// });
/// println!("Elapsed time: {:?}", elapsed_time);
/// ```
#[macro_export]
macro_rules! howlast {
    ($duration_holder:ident $(, $result:ident)? => $code_block:block) => {
        let $duration_holder = ::std::time::Instant::now();
        $(let $result = )?
                $code_block;
        let $duration_holder: ::std::time::Duration = $duration_holder.elapsed();
    };
}